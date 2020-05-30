use crate::summary::Summarize;
use crate::{Attribute, Attributes};
use crate::{CharacterStat, CharacterStats};
use crate::{CombatStat, CombatStats};
use crate::Links;
use crate::{Skill, Skills};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[cfg( feature = "tavern")]
use tavern_db::{TryFromRow, TryFromUuid};

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

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "Effects")]
pub struct DBEffect {
    id: Uuid,
    name: String,
    short_description: String,
    long_description: Option<String>,
}

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "AttributeUnits")]
#[cfg_attr(feature = "tavern", belongs_to(DBEffect, foreign_key = "effect_id"))]
pub struct DBEffectAttributeUnits {
    effect_id: Uuid,
    attr: Attribute,
    modifier: i16,
}

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "Effects")]
#[cfg_attr(feature = "tavern", belongs_to(DBEffect, foreign_key = "effect_id"))]
pub struct DBEffectCharacterUnits {
    effect_id: Uuid,
    stat: CharacterStat,
    modifier: i16,
}

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "Effects")]
#[cfg_attr(feature = "tavern", belongs_to(DBEffect, foreign_key = "effect_id"))]
pub struct DBEffectCombatUnits {
    effect_id: Uuid,
    stat: CombatStat,
    modifier: i16,
}

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "Effects")]
#[cfg_attr(feature = "tavern", belongs_to(DBEffect, foreign_key = "effect_id"))]
pub struct DBEffectMiscUnits {
    effect_id: Uuid,
    description: String,
}

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "Effects")]
#[cfg_attr(feature = "tavern", belongs_to(DBEffect, foreign_key = "effect_id"))]
pub struct DBEffectSkillUnits {
    effect_id: Uuid,
    attr: Skill,
    modifier: i16,
}