use serde::{Serialize,Deserialize};   
use uuid::Uuid;

use crate::Skill;
use crate::Attribute;

#[derive(Serialize,Deserialize)]
pub struct Feat {
    id: Uuid,

    short_description: String,
    long_description: Option<String>,
}

#[derive(Serialize,Deserialize)]
pub struct SkillFeatUnit {
    id: Uuid,

    req_skill: Skill,
    ranks: u8,
}

#[derive(Serialize,Deserialize)]
pub struct AttributeFeatUnit {
    id: Uuid,

    req_attr: Attribute,
    score: u8
}

#[derive(Serialize,Deserialize)]
pub struct RequiredFeat {
    id: Uuid,

    required_feat: Feat,
}
