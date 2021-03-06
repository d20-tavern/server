use serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::collections::BTreeSet;
use uuid::Uuid;

use super::effects::Effect;
use super::item::Item;
use super::summary::{Summarize, Summary};
use super::{Links, SaveThrow};
use tavern_derive::{Display, FromStr};

use crate::schema::{spells, spellcomponents, spelleffects};
use crate::status::Error;
use crate::db::{self, Connection, Delete, DeleteById, GetAll, GetById, Insert, Update, IntoDbWithId, TryFromDb, IntoDb, StandaloneDbMarker};
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use diesel::Connection as DieselConnection;
use diesel::result::Error as DieselError;
use crate::forms::TryFromForm;
use warp::Rejection;
use nebula_form::{Form, Field};
use nebula_status::{Status, StatusCode};
use crate::pathfinder::Skill::Spellcraft;
use crate::{forms, status};
use std::convert::TryFrom;

#[derive(Clone, Serialize, Deserialize, Summarize, Ord, PartialOrd, PartialEq, Eq,)]
pub struct Spell {
    pub links: Links,
    pub id: Uuid,
    pub level: i16,
    pub components: BTreeSet<SpellComponent>,
    pub name: String,
    pub school: MagicSchool,
    pub effects: BTreeSet<Summary<Effect>>,
    pub casting_time: i64,
    pub range: SpellRange,
    pub area: String,
    pub duration_per_level: i64,
    pub saving_throw: Option<SaveThrow>,
    pub spell_resistance: bool,
    pub description: String,
}

impl Spell {
    const FIELD_NAME: &'static str = "spell-name";
    const FIELD_SCHOOL: &'static str = "school";
    const FIELD_EFFECTS: &'static str = "spell-effects";
    const FIELD_CASTING_TIME: &'static str = "casting-time";
    const FIELD_RANGE: &'static str = "range";
    const FIELD_AREA: &'static str = "area-of-effect";
    const FIELD_DURATION: &'static str = "duration-per-level";
    const FIELD_SAVING_THROW: &'static str = "saving-throw";
    const FIELD_HAS_RESISTANCE: &'static str = "has-spell-resistance";
    const FIELD_DESCRIPTION: &'static str = "description";
}

impl TryFromForm for Spell {
    fn try_from_form(conn: &Connection, form: Form, this_id: Option<Uuid>, _parent_id: Option<Uuid>) -> Result<Self, Rejection> {
        let (id, components) = {
            match this_id {
                None => (Uuid::new_v4(), BTreeSet::new()),
                Some(spell_id) => {
                    let id = {
                        use crate::schema::spells::dsl::*;
                        let result = spells.filter(id.eq(&spell_id))
                            .first::<DBSpell>(conn);
                        match result {
                            Ok(_) => spell_id,
                            Err(err) => match err {
                                DieselError::NotFound => {
                                    let error = Error::new(format!("invalid spell id {}", spell_id));
                                    return Err(Status::<_>::with_data(&StatusCode::BAD_REQUEST, error).into());
                                },
                                err => return Err(status::server_error_into_rejection(err.to_string()))
                            }
                        }
                    };
                    let components = {
                        use crate::schema::spellcomponents::dsl::*;
                        let result = spellcomponents.filter(spell_id.eq(&id))
                            .load::<DBSpellComponent>(conn);
                        match result {
                            Ok(list) => {
                                list.into_iter()
                                    .map(|item| {
                                        SpellComponent::try_from_db(item, conn)
                                    })
                                    .collect::<Result<BTreeSet<SpellComponent>, _>>()
                                    .map_err(|err| forms::db_error_to_rejection(err, "id"))
                            },
                            Err(err) => match err {
                                DieselError::NotFound => Ok(BTreeSet::new()),
                                err => Err(status::server_error_into_rejection(err.to_string())),
                            }
                        }
                    }?;
                    (id, components)
                }
            }
        };
        let name: String = forms::get_required_form_text_field(&form, Spell::FIELD_NAME)?;
        let school: MagicSchool = forms::get_required_form_text_field(&form, Spell::FIELD_SCHOOL)?;
        let effects: String = forms::get_required_form_text_field(&form, Spell::FIELD_EFFECTS)?;
        let effects = serde_json::from_str::<Vec<Uuid>>(&effects).
            map_err(|_| {
                let error = Error::new(format!("could not parse {} as array of UUID", Spell::FIELD_EFFECTS));
                Rejection::from(Status::with_data(&StatusCode::BAD_REQUEST, error))
            })?
            .into_iter()
            .map(|id| {
                Summary::<Effect>::db_get_by_id(&id, conn)
                    .map_err(|err| forms::db_error_to_rejection(err, Spell::FIELD_EFFECTS))
            })
            .collect::<Result<BTreeSet<Summary<Effect>>, _>>()?;
        let casting_time: i64 = forms::get_required_form_text_field(&form, Spell::FIELD_CASTING_TIME)?;
        let range: SpellRange = forms::get_required_form_text_field(&form, Spell::FIELD_RANGE)?;
        let area: String = forms::get_required_form_text_field(&form, Spell::FIELD_AREA)?;
        let duration_per_level: i64 = forms::get_required_form_text_field(&form, Spell::FIELD_DURATION)?;
        let saving_throw: Option<SaveThrow> = forms::get_optional_form_text_field(&form, Spell::FIELD_SAVING_THROW)?;
        let spell_resistance: bool = forms::get_required_form_text_field(&form, Spell::FIELD_HAS_RESISTANCE)?;
        let description: String = forms::get_required_form_text_field(&form, Spell::FIELD_DESCRIPTION)?;

        let spell = Spell {
            links: Default::default(),
            id,
            components,
            level: 1, // TODO: Remove because it isn't needed
            name,
            school,
            effects,
            casting_time,
            range,
            area,
            duration_per_level,
            saving_throw,
            spell_resistance,
            description,
        };

        Ok(spell)
    }
}

impl TryFromDb for Spell {
    type DBType = DBSpell;
    fn try_from_db(db_spell: DBSpell, conn: &Connection) -> Result<Self, db::Error> {
        let effects = db_spell.get_effects(conn)?;

        let components = db_spell.get_components(conn)?;

        let spell = Spell {
            links: Default::default(),
            id: db_spell.id,
            components,
            level: db_spell.level,
            name: db_spell.name,
            //level: db_spell.level, TODO: remove from DBSpell and the migration script, reqs updating patch
            school: db_spell.school,
            effects,
            casting_time: db_spell.casting_time,
            range: db_spell.range,
            area: db_spell.area,
            duration_per_level: db_spell.duration_per_level,
            saving_throw: db_spell.saving_throw,
            spell_resistance: db_spell.spell_resistance,
            description: db_spell.description,
        };

        Ok(spell)
    }
}

impl Insert for Spell {
    fn db_insert(&self, conn: &Connection) -> Result<(), db::Error> {
        conn.transaction::<_, db::Error, _>(|| {
            let (spell, components, effects) = Spell::to_owned(self).into_db();
            spell.db_insert(conn)?;
            for component in components {
                component.db_insert(conn)?;
            }
            for effect in effects {
                effect.db_insert(conn)?;
            }
            Ok(())
        })
    }
}

impl Update for Spell {
    fn db_update(&self, conn: &Connection) -> Result<(), db::Error> {
        conn.transaction::<_, db::Error, _>(|| {
            let (spell, components, effects) = Spell::to_owned(self).into_db();
            spell.db_update(conn)?;

            // Get difference in current and updated effects
            let old_effects = spell.get_effects(conn)?;
            let delete_effects = old_effects.difference(&self.effects)
                .map(|effect| DBSpellEffect {
                    spell_id: self.id.to_owned(),
                    effect_id: effect.id().to_owned(),
                });
            let add_effects = self.effects.difference(&old_effects)
                .map(|effect| DBSpellEffect {
                    spell_id: self.id.to_owned(),
                    effect_id: effect.id().to_owned(),
                });

            // Get difference in current and updated components
            let old_components = spell.get_components(conn)?;
            let delete_components = old_components.difference(&self.components)
                .map(|comp| comp.to_owned().into_db(self.id.to_owned()));
            let add_components = self.components.difference(&old_components)
                .map(|comp| comp.to_owned().into_db(self.id.to_owned()));

            for effect in delete_effects {
                effect.db_delete(conn)?;
            }
            for effect in add_effects {
                effect.db_insert(conn)?;
            }

            for component in delete_components {
                component.db_delete(conn)?;
            }
            for component in add_components {
                component.db_insert(conn)?;
            }

            Ok(())
        })
    }
}

impl DeleteById for Spell {
    fn db_delete_by_id(del_id: &Uuid, conn: &Connection) -> Result<(), db::Error> {
        conn.transaction::<_, db::Error, _>(|| {
            use crate::schema::spellcomponents::dsl::*;
            DBSpell::db_delete_by_id(del_id, conn)?;
            DBSpellEffect::db_delete_by_id(del_id, conn)?;
            diesel::delete(spellcomponents.filter(spell_id.eq(del_id)))
                .execute(conn)
                .map_err(db::Error::RunQuery)
                .map(|_| ())
        })
    }
}

#[derive(
    AsChangeset,
    Associations,
    Identifiable,
    Insertable,
    Queryable,
    GetAll,
    GetById,
    Delete,
    DeleteById,
    Insert,
    Update,
Ord,
PartialOrd,
PartialEq,
Eq,
)]
#[tavern(is_insertable, is_identifiable, is_queryable)]
#[table_name = "spells"]
pub struct DBSpell {
    pub id: Uuid,
    pub name: String,
    pub level: i16,
    pub school: MagicSchool,
    pub casting_time: i64,
    pub range: SpellRange,
    pub area: String,
    pub duration_per_level: i64,
    pub saving_throw: Option<SaveThrow>,
    pub spell_resistance: bool,
    pub description: String,
}

impl DBSpell {
    fn get_components(&self, conn: &Connection) -> Result<BTreeSet<SpellComponent>, db::Error> {
        let spell_components = DBSpellComponent::belonging_to(self)
            .load::<DBSpellComponent>(conn)
            .map_err(db::Error::RunQuery)?;
        spell_components.into_iter()
            .map(|spell_component| SpellComponent::db_get_by_id(&spell_component.id, conn))
            .collect()
    }

    fn get_effects(&self, conn: &Connection) -> Result<BTreeSet<Summary<Effect>>, db::Error> {
        let spell_effects = DBSpellEffect::belonging_to(self)
            .load::<DBSpellEffect>(conn)
            .map_err(db::Error::RunQuery)?;
        spell_effects.into_iter()
            .map(|spell_effect| Summary::<Effect>::db_get_by_id(&spell_effect.effect_id, conn))
            .collect()
    }
}

impl IntoDb for Spell {
    type DBType = (DBSpell, BTreeSet<DBSpellComponent>, BTreeSet<DBSpellEffect>);
    fn into_db(self) -> (DBSpell, BTreeSet<DBSpellComponent>, BTreeSet<DBSpellEffect>) {
        let effects = self.effects.iter()
            .map(|effect| {
                DBSpellEffect {
                    spell_id: self.id.to_owned(),
                    effect_id: effect.id().to_owned(),
                }
            }).collect();

        let components = self.components.iter()
            .map(|comp| comp.to_owned().into_db(self.id.to_owned())).collect();

        let dbspell = DBSpell {
            id: self.id,
            name: self.name,
            level: self.level,
            school: self.school,
            casting_time: self.casting_time,
            range: self.range,
            area: self.area,
            duration_per_level: self.duration_per_level,
            saving_throw: self.saving_throw,
            spell_resistance: self.spell_resistance,
            description: self.description,
        };

        (dbspell, components, effects)
    }
}

#[derive(Associations, Identifiable, Insertable, Queryable, Delete, DeleteById, Insert, Ord, PartialOrd, PartialEq, Eq,)]
#[tavern(is_insertable, is_identifiable, is_queryable)]
#[tavern(id_field = "spell_id")]
#[table_name = "spelleffects"]
#[primary_key(spell_id, effect_id)]
#[belongs_to(DBSpell, foreign_key = "spell_id")]
pub struct DBSpellEffect {
    pub spell_id: Uuid,
    pub effect_id: Uuid,
}

#[derive(Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq, Clone, StandaloneDbMarker)]
pub struct SpellComponent {
    pub id: Uuid,
    pub item: Option<(Summary<Item>, i16)>,
    #[serde(rename = "type")]
    pub component_type: ComponentType,
}

impl SpellComponent {
    const FIELD_ITEM_ID: &'static str = "item-id";
    const FIELD_ITEM_COUNT: &'static str = "item-count";
    const FIELD_COMPONENT_TYPE: &'static str = "component-type";
}

impl TryFromForm for SpellComponent {
    fn try_from_form(conn: &Connection, form: Form, id: Option<Uuid>, _parent_id: Option<Uuid>) -> Result<Self, Rejection> {
        let id = id.unwrap_or_else(|| Uuid::new_v4());
        let item: Option<(Summary<Item>, i16)> = {
            let item_id = forms::get_optional_form_text_field(&form, SpellComponent::FIELD_ITEM_ID)?;
            item_id.map::<Result<(Summary<Item>, i16), Rejection>, _>(|id| {
                let item = Summary::<Item>::db_get_by_id(&id, conn)
                    .map_err(|err| forms::db_error_to_rejection(err, SpellComponent::FIELD_ITEM_ID))?;
                let item_count: i16 = forms::get_required_form_text_field(&form, SpellComponent::FIELD_ITEM_COUNT)?;
                Ok((item, item_count))
            }).transpose()?
        };
        let component_type: ComponentType = forms::get_required_form_text_field(&form, SpellComponent::FIELD_COMPONENT_TYPE)?;

        // Item should be given iff material component
        if (component_type == ComponentType::Material) != (item.is_some()) {
            return Err(forms::field_is_invalid_error(SpellComponent::FIELD_ITEM_ID));
        }

        let result = SpellComponent { id, item, component_type };
        Ok(result)
    }
}

impl Delete for SpellComponent {
    fn db_delete(&self, conn: &Connection) -> Result<(), db::Error> {
        SpellComponent::db_delete_by_id(&self.id, conn)
    }
}

impl IntoDbWithId for SpellComponent {
    type DBType = DBSpellComponent;
    fn into_db(self, spell_id: Uuid) -> DBSpellComponent {
        DBSpellComponent {
            id: self.id,
            spell_id,
            item_id: self.item.as_ref().map(|(item, _)| item.id().to_owned()),
            item_amount: self.item.as_ref().map(|(_, amount)| *amount),
            component_type: self.component_type,
        }
    }
}

impl TryFromDb for SpellComponent {
    type DBType = DBSpellComponent;
    fn try_from_db(db_sc: Self::DBType, conn: &Connection) -> Result<Self, db::Error> {
        // Database *should* be enforcing both item columns to be (not) null together
        // If code panics here, check the database design.
        let item = match db_sc.item_id {
            None => None,
            Some(id) => {
                match db_sc.item_amount {
                    None => return Err(db::Error::InvalidValues(vec!["item_id".to_string(), "item_amount".to_string()])),
                    Some(amt) => Some((Summary::<Item>::db_get_by_id(&id, conn)?, amt))
                }
            }
        };

        let result = SpellComponent {
            id: db_sc.id,
            item,
            component_type: db_sc.component_type,
        };

        Ok(result)
    }
}

#[derive(
    AsChangeset,
    Associations,
    Identifiable,
    Insertable,
    Queryable,
    GetById,
    Delete,
    DeleteById,
    Insert,
    Update,
    Ord,
    PartialOrd,
    PartialEq,
    Eq,
)]
#[tavern(is_insertable, is_identifiable, is_queryable)]
#[table_name = "spellcomponents"]
#[belongs_to(DBSpell, foreign_key = "spell_id")]
pub struct DBSpellComponent {
    pub id: Uuid,
    pub spell_id: Uuid,
    pub item_id: Option<Uuid>,
    pub item_amount: Option<i16>,
    pub component_type: ComponentType,
}

#[derive(Serialize, Deserialize, DbEnum, Debug, Display, FromStr, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum CasterType {
    Spontaneous,
    Prepared,
}

#[derive(Serialize, Deserialize, Display, FromStr, PartialOrd, Ord, PartialEq, Eq, DbEnum, Debug, Copy, Clone)]
pub enum ComponentType {
    Somatic,
    Material,
    Verbal,
}

#[derive(Serialize, Deserialize, DbEnum, Display, FromStr, Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum MagicSchool {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
}

#[derive(Serialize, Deserialize, DbEnum, Display, FromStr, Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum SpellRange {
    Personal,
    Touch,
    Close,
    Medium,
    Long,
    Unlimited,
}
