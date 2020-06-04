use super::Links;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::summary::{Summarize, Summary};
use super::{Attributes, Skills};

use super::effects::Effect;
use super::{Attribute, Skill};

use crate::schema::{attributefeatunits, feateffects, featrequirements, feats, skillfeatunits};
use crate::db::{TryFromDb, IntoDb, Connection, Error, GetById, GetAll, Delete, DeleteById, Insert, Update};
use diesel::prelude::*;
use diesel::associations::BelongsTo;
use diesel::Connection as DieselConnection;
use std::collections::{BTreeSet, BTreeMap};
use crate::forms::{self, TryFromForm};
use warp::Rejection;
use nebula_form::Form;
use nebula_status::{Status, StatusCode};

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
    effects: BTreeSet<Summary<Effect>>,
}

impl Feat {
    const FIELD_NAME: &'static str = "name";
    const FIELD_SHORT_DESC: &'static str = "short-description";
    const FIELD_LONG_DESC: &'static str = "long-description";
    const FIELD_REQ_SKILLS: &'static str = "required-skills";
    const FIELD_REQ_ATTRS: &'static str = "required-attrs";
    const FIELD_REQ_FEATS: &'static str = "required-feats";
    const FIELD_EFFECTS: &'static str = "effects";
}

impl TryFromForm for Feat {
    fn try_from_form(conn: &Connection, form: Form, this_id: Option<Uuid>, parent_id: Option<Uuid>) -> Result<Self, Rejection> where Self: Sized {
        let id = forms::valid_id_or_new::<Feat>(this_id, conn)?;
        let name = forms::get_required_form_text_field(&form, Feat::FIELD_NAME)?;
        let short_description = forms::get_required_form_text_field(&form, Feat::FIELD_SHORT_DESC)?;
        let long_description = forms::get_optional_form_text_field(&form, Feat::FIELD_LONG_DESC)?;
        let req_skills: String = forms::get_required_form_text_field(&form, Feat::FIELD_REQ_SKILLS)?;
        let req_skills: Skills = serde_json::from_str::<BTreeMap<String, i16>>(&req_skills)
            .map_err(|_| forms::field_is_invalid_error(Feat::FIELD_REQ_SKILLS))?
            .into_iter()
            .map::<Result<(Skill, i16), Rejection>, _>(|(skill, modifier)| {
                let skill = skill.as_str().parse()
                    .map_err(|e| Rejection::from(Status::with_data(&StatusCode::BAD_REQUEST, e)))?;
                Ok((skill, modifier))
            })
            .collect::<Result<Skills, _>>()?;
        let req_attrs: String = forms::get_required_form_text_field(&form, Feat::FIELD_REQ_ATTRS)?;
        let req_attrs: Attributes = serde_json::from_str::<BTreeMap<String, i16>>(&req_attrs)
            .map_err(|_| forms::field_is_invalid_error(Feat::FIELD_REQ_ATTRS))?
            .into_iter()
            .map::<Result<(Attribute, i16), Rejection>, _>(|(attr, modifier)| {
                let attr = attr.as_str().parse()
                    .map_err(|e| Rejection::from(Status::with_data(&StatusCode::BAD_REQUEST, e)))?;
                Ok((attr, modifier))
            })
            .collect::<Result<Attributes, _>>()?;
        let req_feats: String = forms::get_required_form_text_field(&form, Feat::FIELD_REQ_ATTRS)?;
        let req_feats: Vec<Summary<Feat>> = serde_json::from_str::<Vec<Uuid>>(&req_feats)
            .map_err(|_| forms::field_is_invalid_error(Feat::FIELD_REQ_FEATS))?
            .into_iter()
            .map::<Result<Summary<Feat>, Rejection>, _>(|id| {
                forms::value_by_id(id, conn)
            })
            .collect::<Result<_, _>>()?;
        let effects: String = forms::get_required_form_text_field(&form, Feat::FIELD_EFFECTS)?;
        let effects: BTreeSet<Summary<Effect>> = serde_json::from_str::<Vec<Uuid>>(&effects)
            .map_err(|_| forms::field_is_invalid_error(Feat::FIELD_EFFECTS))?
            .into_iter()
            .map::<Result<Summary<Effect>, Rejection>, _>(|id| {
                forms::value_by_id(id, conn)
            })
            .collect::<Result<_, _>>()?;

        let feat = Feat {
            links: Default::default(),
            id,
            name,
            short_description,
            long_description,
            req_skills,
            req_attrs,
            req_feats,
            effects,
        };

        Ok(feat)
    }
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
        let effects = DBFeatEffect::belonging_to(&other)
            .load::<DBFeatEffect>(conn)
            .map_err(Error::RunQuery)?
            .into_iter()
            .map(|e| Summary::<Effect>::db_get_by_id(&e.effect_id, conn))
            .collect::<Result<_, Error>>()?;

        let feat = Feat {
            id: other.id,
            links,
            name: other.name,
            short_description: other.short_description,
            long_description: other.long_description,
            req_attrs,
            req_feats,
            req_skills,
            effects,
        };

        Ok(feat)
    }
}

impl IntoDb for Feat {
    type DBType = (DBFeat, Vec<DBFeatRequiredAttribute>, Vec<DBFeatRequiredSkill>, Vec<DBFeatRequiredFeat>);

    fn into_db(self) -> Self::DBType {
        let req_attrs = self.req_attrs.iter()
            .map(|(attr, score)| DBFeatRequiredAttribute {
                feat_id: self.id.clone(),
                attr: *attr,
                score: *score,
            })
            .collect();

        let req_skills = self.req_skills.iter()
            .map(|(skill, ranks)| DBFeatRequiredSkill {
                feat_id: self.id.clone(),
                skill: *skill,
                ranks: *ranks,
            })
            .collect();

        let req_feats = self.req_feats.iter()
            .map(|feat| DBFeatRequiredFeat {
                feat_id: self.id.clone(),
                required_feat: feat.id().to_owned(),
            })
            .collect();

        let feat = DBFeat {
            id: self.id.clone(),
            name: self.name,
            short_description: self.short_description,
            long_description: self.long_description,
        };

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
