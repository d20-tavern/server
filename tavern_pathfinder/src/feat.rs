use crate::Links;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::summary::{Summarize, Summary};
use crate::{Attribute, Attributes};
use crate::{Skill, Skills};
#[cfg(feature = "tavern")]
use tavern_db::{TryFromRow, TryFromUuid};
#[cfg(feature = "tavern")]
use crate::effects::Effect;

#[derive(Serialize, Deserialize, Summarize)]
pub struct Feat {
    links: Links,
    id: Uuid,
    name: String,
    #[description]
    short_description: String,
    long_description: Option<String>,
    req_skills: Skills,
    req_attr: Attributes,
    req_feats: Vec<Summary<Feat>>,
}

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "Feats")]
pub struct DBFeat {
    id: Uuid,
    name: String,
    short_description: String,
    long_description: Option<String>,
}

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "SkillFeatUnits")]
#[cfg_attr(feature = "tavern", belongs_to(DBFeat, foreign_key = "feat_id"))]
pub struct DBFeatRequiredSkill {
    feat_id: Uuid,
    skill: Skill,
    ranks: i16,
}

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "AttributeFeatUnits")]
#[cfg_attr(feature = "tavern", belongs_to(DBFeat, foreign_key = "feat_id"))]
pub struct DBFeatRequiredAttribute {
    feat_id: Uuid,
    attr: Attribute,
    score: i16,
}

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "FeatRequirements")]
#[cfg_attr(feature = "tavern", belongs_to(DBFeat, foreign_key = "feat_id"))]
#[cfg_attr(feature = "tavern", belongs_to(DBFeat, foreign_key = "required_feat"))]
pub struct DBFeatRequiredFeat {
    feat_id: Uuid,
    required_feat: Uuid,
}

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "FeatEffects")]
#[cfg_attr(feature = "tavern", belongs_to(DBFeat, foreign_key = "feat_id"))]
#[cfg_attr(feature = "tavern", belongs_to(DBEffect, foreign_key = "effect_id"))]
pub struct DBFeatEffect {
    feat_id: Uuid,
    effect_id: Uuid,
}