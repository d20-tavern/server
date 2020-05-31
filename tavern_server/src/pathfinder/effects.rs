use super::summary::Summarize;
use super::{Attributes, CharacterStats, CombatStats, Links, Skills};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{Attribute, CharacterStat, CombatStat, Skill};
use diesel::prelude::*;

use crate::schema::{attributeunits, characterunits, combatunits, effects, miscunits, skillunits};

#[derive(Serialize, Deserialize, Summarize)]
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
    misc_effects: Option<String>,
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[table_name = "effects"]
pub struct DBEffect {
    id: Uuid,
    name: String,
    short_description: String,
    long_description: Option<String>,
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[table_name = "attributeunits"]
#[primary_key(effect_id, attr)]
#[belongs_to(DBEffect, foreign_key = "effect_id")]
pub struct DBEffectAttributeUnits {
    effect_id: Uuid,
    attr: Attribute,
    modifier: i16,
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[table_name = "characterunits"]
#[primary_key(effect_id, stat)]
#[belongs_to(DBEffect, foreign_key = "effect_id")]
pub struct DBEffectCharacterUnits {
    effect_id: Uuid,
    stat: CharacterStat,
    modifier: i16,
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[table_name = "combatunits"]
#[primary_key(effect_id, stat)]
#[belongs_to(DBEffect, foreign_key = "effect_id")]
pub struct DBEffectCombatUnits {
    effect_id: Uuid,
    stat: CombatStat,
    modifier: i16,
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[table_name = "miscunits"]
#[primary_key(effect_id)]
#[belongs_to(DBEffect, foreign_key = "effect_id")]
pub struct DBEffectMiscUnits {
    effect_id: Uuid,
    description: String,
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[table_name = "skillunits"]
#[primary_key(effect_id, skill)]
#[belongs_to(DBEffect, foreign_key = "effect_id")]
pub struct DBEffectSkillUnits {
    effect_id: Uuid,
    skill: Skill,
    modifier: i16,
}