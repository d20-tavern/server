use serde::{Serialize,Deserialize};   
use uuid::Uuid;

#[derive(Serialize,Deserialize)]
pub struct Feat {
    feat_id: Uuid,

    short_description: String,
    long_description: String,
}

#[derive(Serialize,Deserialize)]
pub struct SkillFeatUnit {
    skill_unit_id: Uuid,

    req_skil: crate::Skill,
    ranks: u8,
}

#[derive(Serialize,Deserialize)]
pub struct AttributeFeatUnit {
    attr_unit_id: Uuid,

    req_attr: crate::Attribute,
    score: u8
}

#[derive(Serialize,Deserialize)]
pub struct RequiredFeat {
    feat_id: Uuid,

    required_feat: Feat,
}
