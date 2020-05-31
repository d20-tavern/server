use super::Links;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::summary::{Summarize, Summary};
use super::{Attributes, Skills};

use super::{Attribute, Skill};
use super::effects::{DBEffect, Effect};

use crate::schema::{attributefeatunits, feats, feateffects, featrequirements, skillfeatunits};

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

#[derive(AsChangeset, Identifiable, Insertable, Queryable)]
#[table_name = "feats"]
pub struct DBFeat {
    id: Uuid,
    name: String,
    short_description: String,
    long_description: Option<String>,
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[table_name = "skillfeatunits"]
#[primary_key(feat_id, skill)]
#[belongs_to(DBFeat, foreign_key = "feat_id")]
pub struct DBFeatRequiredSkill {
    feat_id: Uuid,
    skill: Skill,
    ranks: i16,
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[table_name = "attributefeatunits"]
#[primary_key(feat_id, attr)]
#[belongs_to(DBFeat, foreign_key = "feat_id")]
pub struct DBFeatRequiredAttribute {
    feat_id: Uuid,
    attr: Attribute,
    score: i16,
}

#[derive(Associations, Identifiable, Insertable, Queryable)]
#[table_name = "featrequirements"]
#[primary_key(feat_id, required_feat)]
#[belongs_to(DBFeat, foreign_key = "feat_id")]
pub struct DBFeatRequiredFeat {
    feat_id: Uuid,
    required_feat: Uuid,
}

#[derive(Associations, Identifiable, Insertable, Queryable)]
#[table_name = "feateffects"]
#[primary_key(feat_id, effect_id)]
#[belongs_to(DBFeat, foreign_key = "feat_id")]
pub struct DBFeatEffect {
    feat_id: Uuid,
    effect_id: Uuid,
}