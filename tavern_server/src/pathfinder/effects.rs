use super::summary::Summarize;
use super::{Attributes, CharacterStats, CombatStats, Links, Skills};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{Attribute, CharacterStat, CombatStat, Skill};
use diesel::prelude::*;
use diesel::result::Error as DieselError;

use crate::schema::{attributeunits, characterunits, combatunits, effects, miscunits, skillunits};
use crate::db::{Connection, TryFromDb, IntoDb, Error, GetAll, GetById, Delete, DeleteById, Insert, Update};
use crate::forms::{self, TryFromForm};
use warp::Rejection;
use nebula_form::Form;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Summarize, Clone, Ord, PartialOrd, PartialEq, Eq, Debug)]
pub struct Effect {
    links: Links,
    id: Uuid,
    name: String,
    #[description]
    short_description: String,
    long_description: Option<String>,
    attr_effects: Attributes,
    skill_effects: Skills,
    char_effects: CharacterStats,
    combat_effects: CombatStats,
    misc_effect: Option<String>,
}

impl Effect {
    const FIELD_NAME: &'static str = "name";
    const FIELD_SHORT_DESC: &'static str = "short-description";
    const FIELD_LONG_DESC: &'static str = "long-description";
    const FIELD_ATTR_EFFECTS: &'static str = "attr-effects";
    const FIELD_SKILL_EFFECTS: &'static str = "skill-effects";
    const FIELD_CHAR_EFFECTS: &'static str = "char-effects";
    const FIELD_COMBAT_EFFECTS: &'static str = "combat-effects";
    const FIELD_MISC_EFFECT: &'static str = "misc-effect";
}

impl TryFromForm for Effect {
    fn try_from_form(conn: &Connection, form: Form, this_id: Option<Uuid>, parent_id: Option<Uuid>) -> Result<Self, Rejection> where Self: Sized {
        let id = forms::valid_id_or_new::<Effect>(this_id, conn)?;
        let name = forms::get_required_form_text_field(&form, Effect::FIELD_NAME)?;
        let short_description = forms::get_required_form_text_field(&form, Effect::FIELD_SHORT_DESC)?;
        let long_description = forms::get_optional_form_text_field(&form, Effect::FIELD_LONG_DESC)?;

        let attr_effects: String = forms::get_required_form_text_field(&form, Effect::FIELD_ATTR_EFFECTS)?;
        let attr_effects: Attributes = serde_json::from_str::<BTreeMap<String, i16>>(&attr_effects)
            .map_err(|_| forms::field_is_invalid_error(Effect::FIELD_ATTR_EFFECTS))?
            .into_iter()
            .map(|(attr_string, modifier)| {
                attr_string.as_str().parse()
                    .map_err(|_| forms::field_is_invalid_error(Effect::FIELD_ATTR_EFFECTS))
                    .map(|attr| (attr, modifier))
            })
            .collect::<Result<_, _>>()?;

        let skill_effects: String = forms::get_required_form_text_field(&form, Effect::FIELD_SKILL_EFFECTS)?;
        let skill_effects: Skills = serde_json::from_str::<BTreeMap<String, i16>>(&skill_effects)
            .map_err(|_| forms::field_is_invalid_error(Effect::FIELD_SKILL_EFFECTS))?
            .into_iter()
            .map(|(skill_string, modifier)| {
                skill_string.as_str().parse()
                    .map_err(|_| forms::field_is_invalid_error(Effect::FIELD_SKILL_EFFECTS))
                    .map(|skill| (skill, modifier))
            })
            .collect::<Result<_, _>>()?;

        let combat_effects: String = forms::get_required_form_text_field(&form, Effect::FIELD_COMBAT_EFFECTS)?;
        let combat_effects: CombatStats = serde_json::from_str::<BTreeMap<String, i16>>(&combat_effects)
            .map_err(|_| forms::field_is_invalid_error(Effect::FIELD_COMBAT_EFFECTS))?
            .into_iter()
            .map(|(combat_string, modifier)| {
                combat_string.as_str().parse()
                    .map_err(|_| forms::field_is_invalid_error(Effect::FIELD_COMBAT_EFFECTS))
                    .map(|combat| (combat, modifier))
            })
            .collect::<Result<_, _>>()?;

        let char_effects: String = forms::get_required_form_text_field(&form, Effect::FIELD_CHAR_EFFECTS)?;
        let char_effects: CharacterStats = serde_json::from_str::<BTreeMap<String, i16>>(&char_effects)
            .map_err(|_| forms::field_is_invalid_error(Effect::FIELD_CHAR_EFFECTS))?
            .into_iter()
            .map(|(char_string, modifier)| {
                char_string.as_str().parse()
                    .map_err(|_| forms::field_is_invalid_error(Effect::FIELD_CHAR_EFFECTS))
                    .map(|char| (char, modifier))
            })
            .collect::<Result<_, _>>()?;

        let misc_effect: Option<String> = forms::get_optional_form_text_field(&form, Effect::FIELD_MISC_EFFECT)?;

        let effect = Effect {
            links: Default::default(),
            id,
            name,
            short_description,
            long_description,
            attr_effects,
            skill_effects,
            char_effects,
            combat_effects,
            misc_effect,
        };

        Ok(effect)
    }
}

impl TryFromDb for Effect {
    type DBType = DBEffect;

    fn try_from_db(other: Self::DBType, conn: &Connection) -> Result<Self, Error> where Self: Sized {
        let links = Links::new();
        let attr_effects = DBEffectAttributeUnit::belonging_to(&other)
            .load::<DBEffectAttributeUnit>(conn)
            .map_err(Error::RunQuery)?
            .into_iter()
            .map(|unit| (unit.attr, unit.modifier))
            .collect();
        let skill_effects = DBEffectSkillUnit::belonging_to(&other)
            .load::<DBEffectSkillUnit>(conn)
            .map_err(Error::RunQuery)?
            .into_iter()
            .map(|unit| (unit.skill, unit.modifier))
            .collect();
        let char_effects = DBEffectCharacterUnit::belonging_to(&other)
            .load::<DBEffectCharacterUnit>(conn)
            .map_err(Error::RunQuery)?
            .into_iter()
            .map(|unit| (unit.stat, unit.modifier))
            .collect();
        let combat_effects = DBEffectCombatUnit::belonging_to(&other)
            .load::<DBEffectCombatUnit>(conn)
            .map_err(Error::RunQuery)?
            .into_iter()
            .map(|unit| (unit.stat, unit.modifier))
            .collect();
        let misc_effect = {
            let result = DBEffectMiscUnit::belonging_to(&other)
                .first::<DBEffectMiscUnit>(conn);
            match result {
                Ok(val) => Some(val.description),
                Err(err) => match err {
                    DieselError::NotFound => None,
                    err => return Err(Error::RunQuery(err)),
                }
            }
        };

        let effect = Effect {
            id: other.id,
            links,
            name: other.name,
            short_description: other.short_description,
            long_description: other.long_description,
            attr_effects,
            char_effects,
            combat_effects,
            skill_effects,
            misc_effect,
        };

        Ok(effect)
    }
}

impl IntoDb for Effect {
    type DBType = (DBEffect, Vec<DBEffectAttributeUnit>, Vec<DBEffectSkillUnit>, Vec<DBEffectCharacterUnit>, Vec<DBEffectCombatUnit>, Option<DBEffectMiscUnit>);

    fn into_db(self) -> Self::DBType {
        let attr_units = self.attr_effects.iter()
            .map(|(attr, modifier)| DBEffectAttributeUnit {
                effect_id: self.id.clone(),
                attr: *attr,
                modifier: *modifier,
            }).collect();

        let skill_units = self.skill_effects.iter()
            .map(|(skill, modifier)| DBEffectSkillUnit {
                effect_id: self.id.clone(),
                skill: *skill,
                modifier: *modifier,
            }).collect();

        let char_units = self.char_effects.iter()
            .map(|(stat, modifier)| DBEffectCharacterUnit {
                effect_id: self.id.clone(),
                stat: *stat,
                modifier: *modifier,
            }).collect();

        let combat_units = self.combat_effects.iter()
            .map(|(stat, modifier)| DBEffectCombatUnit {
                effect_id: self.id.clone(),
                stat: *stat,
                modifier: *modifier,
            }).collect();

        let misc_unit = self.misc_effect.as_ref().map(|val| DBEffectMiscUnit {
            effect_id: self.id.clone(),
            description: val.to_owned(),
        });

        let effect = DBEffect {
            id: self.id.clone(),
            name: self.name,
            short_description: self.short_description,
            long_description: self.long_description,
        };

        (effect, attr_units, skill_units, char_units, combat_units, misc_unit)
    }
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(GetAll, GetById, Delete, DeleteById, Insert, Update)]
#[tavern(is_identifiable, is_insertable, is_queryable)]
#[table_name = "effects"]
pub struct DBEffect {
    id: Uuid,
    name: String,
    short_description: String,
    long_description: Option<String>,
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(GetAll, Delete, Insert, Update)]
#[tavern(is_identifiable, is_insertable, is_queryable)]
#[table_name = "attributeunits"]
#[primary_key(effect_id, attr)]
#[belongs_to(DBEffect, foreign_key = "effect_id")]
pub struct DBEffectAttributeUnit {
    effect_id: Uuid,
    attr: Attribute,
    modifier: i16,
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(GetAll, Delete, Insert, Update)]
#[tavern(is_identifiable, is_insertable, is_queryable)]
#[table_name = "characterunits"]
#[primary_key(effect_id, stat)]
#[belongs_to(DBEffect, foreign_key = "effect_id")]
pub struct DBEffectCharacterUnit {
    effect_id: Uuid,
    stat: CharacterStat,
    modifier: i16,
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(GetAll, Delete, Insert, Update)]
#[tavern(is_identifiable, is_insertable, is_queryable)]
#[table_name = "combatunits"]
#[primary_key(effect_id, stat)]
#[belongs_to(DBEffect, foreign_key = "effect_id")]
pub struct DBEffectCombatUnit {
    effect_id: Uuid,
    stat: CombatStat,
    modifier: i16,
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(GetAll, Delete, Insert, Update)]
#[tavern(is_identifiable, is_insertable, is_queryable)]
#[table_name = "miscunits"]
#[primary_key(effect_id)]
#[belongs_to(DBEffect, foreign_key = "effect_id")]
pub struct DBEffectMiscUnit {
    effect_id: Uuid,
    description: String,
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(GetAll, Delete, Insert, Update)]
#[tavern(is_identifiable, is_insertable, is_queryable)]
#[table_name = "skillunits"]
#[primary_key(effect_id, skill)]
#[belongs_to(DBEffect, foreign_key = "effect_id")]
pub struct DBEffectSkillUnit {
    effect_id: Uuid,
    skill: Skill,
    modifier: i16,
}
