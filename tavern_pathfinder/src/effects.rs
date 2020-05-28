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
#[cfg_attr(feature = "tavern", derive(TryFromRow, TryFromUuid))]
pub struct Effect {
    #[cfg_attr(feature = "tavern", tavern(
        skip,
        default = "Links::new()",
    ))]
    links: Links,
    id: Uuid,
    name: String,
    #[description]
    short_description: String,
    #[cfg_attr(feature = "tavern", tavern(is_optional))]
    long_description: Option<String>,
    #[cfg_attr(feature = "tavern", tavern(
        references = "Attributes",
        column = "ARRAY(SELECT ROW(attr, modifier) FROM AttributeUnits WHERE AttributeUnits.id == $1)",
        key_type = "Attribute",
        val_type = "i16",
        is_map,
    ))]
    attr_effects: Attributes,
    #[cfg_attr(feature = "tavern", tavern(
        references = "Skills",
        column = "ARRAY(SELECT ROW(attr, modifier) FROM SkillUnits WHERE SkillUnits.id == $1)",
        key_type = "Skill",
        val_type = "i16",
        is_map,
    ))]
    skill_effects: Skills,
    #[cfg_attr(feature = "tavern", tavern(
        references = "CharacterStats",
        column = "ARRAY(SELECT ROW(attr, modifier) FROM CharacterUnits WHERE CharacterUnits.id == $1)",
        key_type = "CharacterStat",
        val_type = "i16",
        is_map,
    ))]
    char_effects: CharacterStats,
    #[cfg_attr(feature = "tavern", tavern(
        references = "CombatStats",
        column = "ARRAY(SELECT ROW(attr, modifier) FROM CombatUnits WHERE CombatUnits.id == $1)",
        key_type = "CharacterStat",
        val_type = "i16",
        is_map,
    ))]
    combat_effects: CombatStats,
    misc_effects: String,
}
