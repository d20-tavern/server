use super::Links;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::summary::{Summarize, Summary};
use super::{Attributes, Skills};

use super::effects::{DBEffect, Effect};
use super::{Attribute, Skill};

use crate::schema::{attributefeatunits, feateffects, featrequirements, feats, skillfeatunits};
use crate::db::{TryFromDb, IntoDb, Connection, Error, GetById, GetAll, Delete, DeleteById, Insert, Update};
use diesel::prelude::*;
use diesel::associations::BelongsTo;
use diesel::Connection as DieselConnection;

#[derive(Serialize, Deserialize, Summarize, Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct Feat {
    links: Links,
    id: Uuid,
    name: String,
    #[description]
    short_description: String,
    long_description: Option<String>,
    req_skills: Skills,
    req_attrs: Attributes,
    req_feats: Vec<Summary<Feat>>,
}

impl TryFromDb for Feat {
    type DBType = DBFeat;

    fn try_from_db(other: Self::DBType, conn: &Connection) -> Result<Self, Error> where Self: Sized {
        let links = Links::new();
        let req_skills = DBFeatRequiredSkill::belonging_to(&other)
            .load::<DBFeatRequiredSkill>(conn)
            .map_err(Error::RunQuery)?
            .into_iter()
            .map(|skill| (skill.skill, skill.ranks))
            .collect();
        let req_attrs = DBFeatRequiredAttribute::belonging_to(&other)
            .load::<DBFeatRequiredAttribute>(conn)
            .map_err(Error::RunQuery)?
            .into_iter()
            .map(|attr| (attr.attr, attr.score))
            .collect();
        let req_feats = DBFeatRequiredFeat::belonging_to(&other)
            .load::<DBFeatRequiredFeat>(conn)
            .map_err(Error::RunQuery)?
            .into_iter()
            .map(|feat| Summary::<Feat>::db_get_by_id(&feat.required_feat, conn))
            .collect::<Result<Vec<Summary<Feat>>, Error>>()?;

        let feat = Feat {
            id: other.id,
            links,
            name: other.name,
            short_description: other.short_description,
            long_description: other.long_description,
            req_attrs,
            req_feats,
            req_skills,
        };

        Ok(feat)
    }
}

impl IntoDb for Feat {
    type DBType = (DBFeat, Vec<DBFeatRequiredAttribute>, Vec<DBFeatRequiredSkill>, Vec<DBFeatRequiredFeat>);

    fn into_db(self) -> Self::DBType {
        let feat = DBFeat {
            id: self.id.clone(),
            name: self.name,
            short_description: self.short_description,
            long_description: self.long_description,
        };

        let req_attrs = self.req_attrs.into_iter()
            .map(|(attr, score)| DBFeatRequiredAttribute {
                feat_id: self.id.clone(),
                attr,
                score,
            })
            .collect();

        let req_skills = self.req_skills.into_iter()
            .map(|(skill, ranks)| DBFeatRequiredSkill {
                feat_id: self.id.clone(),
                skill,
                ranks,
            })
            .collect();

        let req_feats = self.req_feats.into_iter()
            .map(|feat| DBFeatRequiredFeat {
                feat_id: self.id.clone(),
                required_feat: self.id.clone(),
            })
            .collect();

        (feat, req_attrs, req_skills, req_feats)
    }
}

#[derive(AsChangeset, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(GetAll, GetById, Insert, Update, Delete, DeleteById)]
#[tavern(is_insertable, is_identifiable, is_queryable)]
#[table_name = "feats"]
pub struct DBFeat {
    id: Uuid,
    name: String,
    short_description: String,
    long_description: Option<String>,
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(Insert, Update, Delete)]
#[tavern(is_insertable, is_identifiable, is_queryable)]
#[table_name = "skillfeatunits"]
#[primary_key(feat_id, skill)]
#[belongs_to(DBFeat, foreign_key = "feat_id")]
pub struct DBFeatRequiredSkill {
    feat_id: Uuid,
    skill: Skill,
    ranks: i16,
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(Insert, Update, Delete)]
#[tavern(is_insertable, is_identifiable, is_queryable)]
#[table_name = "attributefeatunits"]
#[primary_key(feat_id, attr)]
#[belongs_to(DBFeat, foreign_key = "feat_id")]
pub struct DBFeatRequiredAttribute {
    feat_id: Uuid,
    attr: Attribute,
    score: i16,
}

#[derive(Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(Insert, Delete)]
#[tavern(is_insertable, is_identifiable, is_queryable)]
#[table_name = "featrequirements"]
#[primary_key(feat_id, required_feat)]
#[belongs_to(DBFeat, foreign_key = "feat_id")]
pub struct DBFeatRequiredFeat {
    feat_id: Uuid,
    required_feat: Uuid,
}

#[derive(Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(Insert, Delete)]
#[tavern(is_insertable, is_identifiable, is_queryable)]
#[table_name = "feateffects"]
#[primary_key(feat_id, effect_id)]
#[belongs_to(DBFeat, foreign_key = "feat_id")]
pub struct DBFeatEffect {
    feat_id: Uuid,
    effect_id: Uuid,
}
