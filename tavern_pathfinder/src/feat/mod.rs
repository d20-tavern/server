use serde::{Serialize,Deserialize};   
use uuid::Uuid;

use crate::Skill;
use crate::Attribute;

#[derive(Serialize,Deserialize)]
pub struct Feat {
    feat_id: Uuid,

    short_description: String,
    long_description: Option<String>,
}

#[derive(Serialize,Deserialize)]
pub struct SkillFeatUnit {
    skill_unit_id: Uuid,

    req_skill: Skill,
    ranks: u8,
}

#[derive(Serialize,Deserialize)]
pub struct AttributeFeatUnit {
    attr_unit_id: Uuid,

    req_attr: Attribute,
    score: u8
}

#[derive(Serialize,Deserialize)]
pub struct RequiredFeat {
    feat_id: Uuid,

    required_feat: Feat,
}
