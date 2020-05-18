use serde::{Serialize,Deserialize};	
use uuid::Uuid;

use crate::Attribute;
use crate::Skill;
use crate::CharacterStat;
use crate::CombatStat;

#[derive(Serialize,Deserialize)]
pub struct Effect {
    id: Uuid,

    short_description: String,
    long_description: String,

    attr_effects: Vec<AttributeUnit>,
    skill_effects: Vec<SkillUnit>,
    char_effects: Vec<CharacterUnit>,
    combat_effects: Vec<CombatUnit>,
    misc_effects: Vec<MiscUnit>,
}

#[derive(Serialize, Deserialize)]
pub struct AttributeUnit {
    id: Uuid,

    base_attr: Attribute,
    modifier: i32,
}

#[derive(Serialize, Deserialize)]
pub struct SkillUnit {
    id: Uuid,

    skill: Skill,
    modifier: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CharacterUnit {
    id: Uuid,

    character_stat: CharacterStat,
    modifier: i32
}

#[derive(Serialize, Deserialize)]
pub struct CombatUnit {
    id: Uuid,

    combat_stat: CombatStat,
    modifier: i32,
}

#[derive(Serialize, Deserialize)]
pub struct MiscUnit {
    id: Uuid,

    description: String,
}
