use serde::{Serialize,Deserialize};	
use uuid::Uuid;

#[derive(Serialize,Deserialize)]
pub struct Effect {
    effect_id: Uuid,

    short_description: String,
    long_description: String,
}

#[derive(Serialize, Deserialize)]
pub struct AttributeUnit {
    attr_unit_id: Uuid,

    base_attr: crate::Attribute,
    modifier: i32,
}

#[derive(Serialize, Deserialize)]
pub struct SkillUnit {
    skill_unit_id: Uuid,

    skill: crate::Skill,
    modifier: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CharacterUnit {
    char_unit_id: Uuid,

    character_stat: crate::CharacterStat,
    modifier: i32
}

#[derive(Serialize, Deserialize)]
pub struct CombatUnit {
    combat_unit_id: Uuid,

    combat_stat: crate::CombatStat,
    modifier: i32,
}

#[derive(Serialize, Deserialize)]
pub struct MiscUnit {
    misc_unit_id: Uuid,

    description: String,
}
