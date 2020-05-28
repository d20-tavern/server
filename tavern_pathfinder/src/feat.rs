use crate::Links;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::summary::{Summarize, Summary};
use crate::Attributes;
use crate::Skills;
#[cfg( feature = "tavern")]
use tavern_db::{TryFromRow, TryFromUuid};

#[derive(Serialize, Deserialize, Summarize)]
#[cfg_attr(feature = "tavern", derive(TryFromRow, TryFromUuid))]
pub struct Feat {
    #[cfg_attr(feature = "tavern", tavern(
        skip, default = "Links::new()"
    ))]
    links: Links,
    id: Uuid,
    name: String,
    #[description]
    short_description: String,
    #[cfg_attr(feature = "tavern", tavern(is_optional))]
    long_description: Option<String>,
    #[cfg_attr(feature = "tavern", tavern(
        references = "Skills",
        column = "ARRAY(SELECT ROW(skill, ranks) FROM SkillFeatUnits WHERE SkillFeatUnits.id = $1)",
        is_map
    ))]
    req_skills: Skills,
    #[cfg_attr(feature = "tavern", tavern(
        references = "Attributes",
        column = "ARRAY(SELECT ROW(attr, score) FROM AttributeFeatUnits WHERE AttributeFeatUnits.id = $1)",
        is_map
    ))]
    req_attr: Attributes,
    #[cfg_attr(feature = "tavern", tavern(
        references = "Feat",
        column = "ARRAY(SELECT required_feat FROM FeatRequirements WHERE FeatRequirements.feat_id = $1)",
        is_array
    ))]
    req_feats: Vec<Summary<Feat>>,
}
