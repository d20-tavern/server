use crate::Links;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::summary::Summarize;
use crate::Attributes;
use crate::Skills;

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
    req_feats: Vec<Feat>,
}
