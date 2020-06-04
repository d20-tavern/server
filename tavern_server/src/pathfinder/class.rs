use super::item::{Armor, ArmorClass, Weapon, WeaponClass};
use super::spell::CasterType;
use super::summary::{Summarize, Summary};
use super::Attribute;
use super::Links;

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use uuid::Uuid;

use diesel::prelude::*;
use diesel::connection::Connection as DieselConnection;
use diesel::result::Error as DieselError;

use crate::schema::{
    classes, classnotproficientarmor, classnotproficientweapons, classproficientarmor,
    classproficientarmorclasses, classproficientweaponclasses, classproficientweapons, features,
    subclasses, subclassfeatures,
};
use std::cmp::Ordering;
use crate::db::{Connection, TryFromDb, IntoDb, IntoDbWithId, GetById, GetAll, Delete, DeleteById, Insert, Update, Error as DBError};
use crate::forms::{self, TryFromForm};
use warp::Rejection;
use nebula_form::Form;
use crate::auth::FIELD_EMAIL;
use crate::pathfinder::feat::Feat;
use crate::status::Error;
use nebula_status::{Status, StatusCode};

#[derive(Serialize, Deserialize, Summarize, Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct Subclass {
    links: Links,
    id: Uuid,
    name: String,
    description: String,
    parent_class: Summary<Class>,

    caster_type: Option<CasterType>,
    casting_attr: Option<Attribute>,

    features: Vec<Feature>,
}

impl Subclass {
    const FIELD_NAME: &'static str = "name";
    const FIELD_DESCRIPTION: &'static str = "description";
    const FIELD_CASTER_TYPE: &'static str = "caster-type";
    const FIELD_CASTING_ATTR: &'static str = "casting-attr";
    const FIELD_FEATURES: &'static str = "features";
}

impl TryFromForm for Subclass {
    fn try_from_form(conn: &Connection, form: Form, this_id: Option<Uuid>, parent_id: Option<Uuid>) -> Result<Self, Rejection> where Self: Sized {
        let id = forms::valid_id_or_new::<Subclass>(this_id, conn)?;
        let parent_class: Summary<Class> = parent_id
            .map(|id| forms::value_by_id(id, conn))
            .transpose()?
            .ok_or_else(|| {
                let err = Error::new("invalid URI: expected a parent class ID".to_string());
                Rejection::from(Status::with_data(&StatusCode::BAD_REQUEST, err))
            })?;

        let name = forms::get_required_form_text_field(&form, Subclass::FIELD_NAME)?;
        let description = forms::get_required_form_text_field(&form, Subclass::FIELD_DESCRIPTION)?;
        let caster_type = forms::get_optional_form_text_field(&form, Subclass::FIELD_CASTER_TYPE)?;
        let casting_attr = forms::get_optional_form_text_field(&form, Subclass::FIELD_CASTING_ATTR)?;

        if casting_attr.is_some() != caster_type.is_some() {
            return Err(forms::field_is_invalid_error(Subclass::FIELD_CASTING_ATTR));
        }

        let features: String = forms::get_required_form_text_field(&form, Subclass::FIELD_FEATURES)?;
        let features = serde_json::from_str::<Vec<Uuid>>(&features)
            .map_err(|_| forms::field_is_invalid_error(Subclass::FIELD_FEATURES))?
            .into_iter()
            .map(|id| forms::value_by_id(id, conn))
            .collect::<Result<_, _>>()?;

        let subclass = Subclass {
            links: Default::default(),
            id,
            name,
            description,
            parent_class,
            caster_type,
            casting_attr,
            features,
        };

        Ok(subclass)
    }
}

impl TryFromDb for Subclass {
    type DBType = DBSubclass;

    fn try_from_db(other: Self::DBType, conn: &Connection) -> Result<Self, DBError> {
        let links = Links::new();
        let parent_class = Summary::<Class>::db_get_by_id(&other.class_id, conn)?;
        let features = other.get_features(conn)?;
        let subclass = Subclass {
            links,
            id: other.id,
            name: other.name,
            description: other.description,
            parent_class,
            caster_type: other.caster_type,
            casting_attr: other.casting_attr,
            features,
        };
        Ok(subclass)
    }
}

impl IntoDb for Subclass {
    type DBType = (DBSubclass, Vec<DBSubclassFeature>);

    fn into_db(self) -> Self::DBType {
        let features = self.features.iter()
            .map(|f| DBSubclassFeature {
                subclass_id: self.id.clone(),
                feature_id: f.id.to_owned(),
            }).collect();
        let db_subclass = DBSubclass {
            id: self.id,
            name: self.name,
            description: self.description,
            class_id: self.parent_class.id().to_owned(),
            caster_type: self.caster_type,
            casting_attr: self.casting_attr,
        };
        (db_subclass, features)
    }
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(GetAll, GetById, Insert, Delete, DeleteById, Update)]
#[tavern(is_identifiable, is_insertable, is_queryable)]
#[table_name = "subclasses"]
#[belongs_to(DBClass, foreign_key = "class_id")]
pub struct DBSubclass {
    id: Uuid,
    name: String,
    description: String,
    class_id: Uuid,
    caster_type: Option<CasterType>,
    casting_attr: Option<Attribute>,
}

impl DBSubclass {
    fn get_features(&self, conn: &Connection) -> Result<Vec<Feature>, DBError> {
        DBSubclassFeature::belonging_to(self)
            .load::<DBSubclassFeature>(conn)
            .map_err(DBError::RunQuery)?
            .into_iter()
            .map(|f| Feature::db_get_by_id(&f.feature_id, conn))
            .collect()
    }
}

#[derive(Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(GetAll, Insert, Delete)]
#[tavern(is_identifiable, is_insertable, is_queryable)]
#[table_name = "subclassfeatures"]
#[primary_key(subclass_id, feature_id)]
#[belongs_to(DBSubclass, foreign_key = "subclass_id")]
pub struct DBSubclassFeature {
    subclass_id: Uuid,
    feature_id: Uuid,
}

#[derive(Serialize, Deserialize, Summarize, Clone)]
pub struct Class {
    links: Links,
    id: Uuid,
    //subclasses: Vec<Subclass>,
    weapon_proficiencies: WeaponProficiencies,
    armor_proficiencies: ArmorProficiencies,
    name: String,
    description: String,
    hit_die: String,
    starting_wealth: String,
    bab_per_level: f64,
    skills_per_level: i16,
    skills_attr: Attribute,
}

impl Class {
    const FIELD_NAME: &'static str = "name";
    const FIELD_DESCRIPTION: &'static str = "description";
    const FIELD_HIT_DIE: &'static str = "hit-die";
    const FIELD_STARTING_WEALTH: &'static str = "starting-wealth";
    const FIELD_BAB_PER_LVL: &'static str = "bab-per-level";
    const FIELD_SKILLS_PER_LVL: &'static str = "skills-per-level";
    const FIELD_SKILLS_ATTR: &'static str = "skills-attr";
    const FIELD_PROF_ARMOR_CLASS: &'static str = "prof-armor-class";
    const FIELD_PROF_ARMOR: &'static str = "prof-armor";
    const FIELD_NOT_PROF_ARMOR: &'static str = "not-prof-armor";
    const FIELD_PROF_WEAPON_CLASS: &'static str = "prof-weapon-class";
    const FIELD_PROF_WEAPON: &'static str = "prof-weapon";
    const FIELD_NOT_PROF_WEAPON: &'static str = "not-prof-weapon";
}

impl TryFromForm for Class {
    fn try_from_form(conn: &Connection, form: Form, this_id: Option<Uuid>, parent_id: Option<Uuid>) -> Result<Self, Rejection> where Self: Sized {
        let id = forms::valid_id_or_new::<Class>(this_id, conn)?;
        let name = forms::get_required_form_text_field(&form, Class::FIELD_NAME)?;
        let description = forms::get_required_form_text_field(&form, Class::FIELD_DESCRIPTION)?;
        let hit_die = forms::get_required_form_text_field(&form, Class::FIELD_HIT_DIE)?;
        let starting_wealth = forms::get_required_form_text_field(&form, Class::FIELD_STARTING_WEALTH)?;
        let bab_per_level = forms::get_required_form_text_field(&form, Class::FIELD_BAB_PER_LVL)?;
        let skills_per_level = forms::get_required_form_text_field(&form, Class::FIELD_SKILLS_PER_LVL)?;
        let skills_attr = forms::get_required_form_text_field(&form, Class::FIELD_SKILLS_ATTR)?;

        let prof_armor_class: String = forms::get_required_form_text_field(&form, Class::FIELD_PROF_ARMOR_CLASS)?;
        let prof_armor_class: BTreeSet<ArmorClass> = serde_json::from_str::<Vec<String>>(&prof_armor_class)
            .map_err(|_| forms::field_is_invalid_error(Class::FIELD_PROF_ARMOR_CLASS))?
            .into_iter()
            .map(|val| {
                val.as_str().parse()
                    .map_err(|_| forms::field_is_invalid_error(Class::FIELD_PROF_ARMOR_CLASS))
            })
            .collect::<Result<_, _>>()?;

        let prof_armor: String = forms::get_required_form_text_field(&form, Class::FIELD_PROF_ARMOR)?;
        let prof_armor: BTreeSet<Summary<Armor>> = serde_json::from_str::<Vec<Uuid>>(&prof_armor)
            .map_err(|_| forms::field_is_invalid_error(Class::FIELD_PROF_ARMOR))?
            .into_iter()
            .map(|id| forms::value_by_id(id, conn))
            .collect::<Result<_, _>>()?;

        let not_prof_armor: String = forms::get_required_form_text_field(&form, Class::FIELD_NOT_PROF_ARMOR)?;
        let not_prof_armor: BTreeSet<Summary<Armor>> = serde_json::from_str::<Vec<Uuid>>(&not_prof_armor)
            .map_err(|_| forms::field_is_invalid_error(Class::FIELD_NOT_PROF_ARMOR))?
            .into_iter()
            .map(|id| forms::value_by_id(id, conn))
            .collect::<Result<_, _>>()?;

        let prof_weapon_class: String = forms::get_required_form_text_field(&form, Class::FIELD_PROF_WEAPON_CLASS)?;
        let prof_weapon_class: BTreeSet<WeaponClass> = serde_json::from_str::<Vec<String>>(&prof_weapon_class)
            .map_err(|_| forms::field_is_invalid_error(Class::FIELD_PROF_WEAPON_CLASS))?
            .into_iter()
            .map(|val| {
                val.as_str().parse()
                    .map_err(|_| forms::field_is_invalid_error(Class::FIELD_PROF_WEAPON_CLASS))
            })
            .collect::<Result<_, _>>()?;

        let prof_weapon: String = forms::get_required_form_text_field(&form, Class::FIELD_PROF_WEAPON)?;
        let prof_weapon: BTreeSet<Summary<Weapon>> = serde_json::from_str::<Vec<Uuid>>(&prof_weapon)
            .map_err(|_| forms::field_is_invalid_error(Class::FIELD_PROF_WEAPON))?
            .into_iter()
            .map(|id| forms::value_by_id(id, conn))
            .collect::<Result<_, _>>()?;

        let not_prof_weapon: String = forms::get_required_form_text_field(&form, Class::FIELD_NOT_PROF_WEAPON)?;
        let not_prof_weapon: BTreeSet<Summary<Weapon>> = serde_json::from_str::<Vec<Uuid>>(&not_prof_weapon)
            .map_err(|_| forms::field_is_invalid_error(Class::FIELD_NOT_PROF_WEAPON))?
            .into_iter()
            .map(|id| forms::value_by_id(id, conn))
            .collect::<Result<_, _>>()?;

        let class = Class {
            links: Default::default(),
            id,
            weapon_proficiencies: WeaponProficiencies {
                classes: prof_weapon_class,
                prof: prof_weapon,
                not_prof: not_prof_weapon,
            },
            armor_proficiencies: ArmorProficiencies {
                classes: prof_armor_class,
                prof: prof_armor,
                not_prof: not_prof_armor,
            },
            name,
            description,
            hit_die,
            starting_wealth,
            bab_per_level,
            skills_per_level,
            skills_attr,
        };

        Ok(class)
    }
}

impl TryFromDb for Class {
    type DBType = DBClass;

    fn try_from_db(other: Self::DBType, conn: &Connection) -> Result<Self, DBError> where Self: Sized {
        let links = Links::new();
        let weapon_proficiencies = other.get_weapon_proficiencies(conn)?;
        let armor_proficiencies = other.get_armor_proficiencies(conn)?;
        let class = Class {
            links,
            id: other.id,
            weapon_proficiencies,
            armor_proficiencies,
            name: other.name,
            description: other.description,
            hit_die: other.hit_die,
            starting_wealth: other.starting_wealth,
            bab_per_level: other.bab_per_level,
            skills_per_level: other.skills_per_level,
            skills_attr: other.skills_attr,
        };
        Ok(class)
    }
}

impl IntoDb for Class {
    type DBType = (DBClass, <WeaponProficiencies as IntoDbWithId>::DBType, <ArmorProficiencies as IntoDbWithId>::DBType);

    fn into_db(self) -> Self::DBType {
        let db_weapon_prof = self.weapon_proficiencies.into_db(self.id.clone());
        let db_armor_prof = self.armor_proficiencies.into_db(self.id.clone());
        let db_class = DBClass {
            id: self.id,
            name: self.name,
            description: self.description,
            hit_die: self.hit_die,
            starting_wealth: self.starting_wealth,
            bab_per_level: self.bab_per_level,
            skills_per_level: self.skills_per_level,
            skills_attr: self.skills_attr,
        };

        (db_class, db_weapon_prof, db_armor_prof)
    }
}

impl Ord for Class {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Class {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Class{}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone)]
#[derive(GetAll, GetById, Delete, DeleteById, Insert, Update)]
#[tavern(is_identifiable, is_insertable, is_queryable)]
#[table_name = "classes"]
pub struct DBClass {
    id: Uuid,
    name: String,
    description: String,
    hit_die: String,
    starting_wealth: String,
    bab_per_level: f64,
    skills_per_level: i16,
    skills_attr: Attribute,
}

impl DBClass {
    fn get_weapon_proficiencies(&self, conn: &Connection) -> Result<WeaponProficiencies, DBError> {
        let classes = {
            let result = DBClassProficientWeaponClass::belonging_to(self)
                .first(conn);
            match result {
                Ok(classes) => classes,
                Err(err) => match err {
                    DieselError::NotFound => DBClassProficientWeaponClass {
                        class_id: self.id.clone(),
                        weapon_classes: vec![],
                    },
                    err => return Err(DBError::RunQuery(err)),
                }
            }
        };

        let prof = {
            DBClassProficientWeapon::belonging_to(self)
                .load::<DBClassProficientWeapon>(conn)
                .map_err(DBError::RunQuery)?
        };

        let not_prof = {
            DBClassNotProficientWeapon::belonging_to(self)
                .load::<DBClassNotProficientWeapon>(conn)
                .map_err(DBError::RunQuery)?
        };

        WeaponProficiencies::try_from_db((classes, prof, not_prof), conn)
    }

    fn get_armor_proficiencies(&self, conn: &Connection) -> Result<ArmorProficiencies, DBError> {
        let classes = {
            let result = DBClassProficientArmorClass::belonging_to(self)
                .first(conn);
            match result {
                Ok(classes) => classes,
                Err(err) => match err {
                    DieselError::NotFound => DBClassProficientArmorClass {
                        class_id: self.id.clone(),
                        armor_classes: vec![],
                    },
                    err => return Err(DBError::RunQuery(err)),
                }
            }
        };

        let prof = {
            DBClassProficientArmor::belonging_to(self)
                .load::<DBClassProficientArmor>(conn)
                .map_err(DBError::RunQuery)?
        };

        let not_prof = {
            DBClassNotProficientArmor::belonging_to(self)
                .load::<DBClassNotProficientArmor>(conn)
                .map_err(DBError::RunQuery)?
        };

        ArmorProficiencies::try_from_db((classes, prof, not_prof), conn)
    }
}

impl Ord for DBClass {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for DBClass {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for DBClass {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for DBClass{}

#[derive(
Serialize,
Deserialize,
Summarize,
Clone, Ord, PartialOrd, PartialEq, Eq,
)]
pub struct Feature {
    id: Uuid,
    name: String,
    description: String,
}

impl Feature {
    const FIELD_NAME: &'static str = "name";
    const FIELD_DESCRIPTION: &'static str = "description";
}

impl TryFromForm for Feature {
    fn try_from_form(conn: &Connection, form: Form, this_id: Option<Uuid>, parent_id: Option<Uuid>) -> Result<Self, Rejection> where Self: Sized {
        let id = forms::valid_id_or_new::<Feature>(this_id, conn)?;
        let name = forms::get_required_form_text_field(&form, Feature::FIELD_NAME)?;
        let description = forms::get_required_form_text_field(&form, Feature::FIELD_DESCRIPTION)?;

        Ok(Feature { id, name, description })
    }
}

// This redundancy is due to the use of Summary<Feature> and the automatic
// impls of the traits defined in crate::db. This should make the type system
// happy and still provide those automatic impls.

impl TryFromDb for Feature {
    type DBType = DBFeature;

    fn try_from_db(other: Self::DBType, _conn: &Connection) -> Result<Self, DBError> where Self: Sized {
        let feature = Feature {
            id: other.id,
            name: other.name,
            description: other.description,
        };
        Ok(feature)
    }
}

impl IntoDb for Feature {
    type DBType = DBFeature;

    fn into_db(self) -> Self::DBType {
        DBFeature {
            id: self.id,
            name: self.name,
            description: self.description,
        }
    }
}

#[derive(
    AsChangeset,
    Associations,
    Identifiable,
    Insertable,
    Queryable, Clone, Ord, PartialOrd, PartialEq, Eq,
)]
#[derive(GetById, GetAll, Delete, DeleteById, Insert, Update)]
#[tavern(is_identifiable, is_insertable, is_queryable)]
#[table_name = "features"]
pub struct DBFeature {
    id: Uuid,
    name: String,
    description: String,
}

pub trait Proficiencies<T> {
    type Class;
    fn add_class(&mut self, class: Self::Class);
    fn proficient(&mut self, item: T);
    fn not_proficient(&mut self, item: T);
}

#[derive(Serialize, Deserialize, Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct ArmorProficiencies {
    classes: BTreeSet<ArmorClass>,
    prof: BTreeSet<Summary<Armor>>,
    not_prof: BTreeSet<Summary<Armor>>,
}

impl TryFromDb for ArmorProficiencies {
    type DBType = (DBClassProficientArmorClass, Vec<DBClassProficientArmor>, Vec<DBClassNotProficientArmor>);

    fn try_from_db(other: Self::DBType, conn: &Connection) -> Result<Self, DBError> where Self: Sized {
        let (classes, prof, not_prof) = other;
        let classes = classes.armor_classes.into_iter().collect();
        let prof = prof.into_iter()
            .map(|pa| Summary::<Armor>::db_get_by_id(&pa.armor_id, conn))
            .collect::<Result<_, DBError>>()?;
        let not_prof = not_prof.into_iter()
            .map(|npa| Summary::<Armor>::db_get_by_id(&npa.armor_id, conn))
            .collect::<Result<_, DBError>>()?;

        Ok(ArmorProficiencies{ classes, prof, not_prof })
    }
}

impl IntoDbWithId for ArmorProficiencies {
    type DBType = (DBClassProficientArmorClass, Vec<DBClassProficientArmor>, Vec<DBClassNotProficientArmor>);

    fn into_db(self, id: Uuid) -> Self::DBType {
        let classes = DBClassProficientArmorClass {
            class_id: id.clone(),
            armor_classes: self.classes.into_iter().collect()
        };
        let prof = self.prof.into_iter()
            .map(|pa| DBClassProficientArmor {
                class_id: id.clone(),
                armor_id: pa.id().clone(),
            })
            .collect();
        let not_prof = self.not_prof.into_iter()
            .map(|npa| DBClassNotProficientArmor {
                class_id: id,
                armor_id: npa.id().clone(),
            })
            .collect();

        (classes, prof, not_prof)
    }
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[table_name = "classproficientarmorclasses"]
#[primary_key(class_id)]
#[belongs_to(DBClass, foreign_key = "class_id")]
pub struct DBClassProficientArmorClass {
    class_id: Uuid,
    armor_classes: Vec<ArmorClass>,
}

#[derive(Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[table_name = "classproficientarmor"]
#[primary_key(class_id, armor_id)]
#[belongs_to(DBClass, foreign_key = "class_id")]
pub struct DBClassProficientArmor {
    class_id: Uuid,
    armor_id: Uuid,
}

#[derive(Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[table_name = "classnotproficientarmor"]
#[primary_key(class_id, armor_id)]
#[belongs_to(DBClass, foreign_key = "class_id")]
pub struct DBClassNotProficientArmor {
    class_id: Uuid,
    armor_id: Uuid,
}

impl Proficiencies<Summary<Armor>> for ArmorProficiencies {
    type Class = ArmorClass;

    fn add_class(&mut self, class: Self::Class) {
        self.classes.insert(class);
    }

    fn proficient(&mut self, item: Summary<Armor>) {
        self.not_prof.remove(&item);
        self.prof.insert(item);
    }

    fn not_proficient(&mut self, item: Summary<Armor>) {
        self.prof.remove(&item);
        self.not_prof.insert(item);
    }
}

#[derive(Serialize, Deserialize, Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct WeaponProficiencies {
    classes: BTreeSet<WeaponClass>,
    prof: BTreeSet<Summary<Weapon>>,
    not_prof: BTreeSet<Summary<Weapon>>,
}

impl TryFromDb for WeaponProficiencies {
    type DBType = (DBClassProficientWeaponClass, Vec<DBClassProficientWeapon>, Vec<DBClassNotProficientWeapon>);

    fn try_from_db(other: Self::DBType, conn: &Connection) -> Result<Self, DBError> where Self: Sized {
        let (classes, prof, not_prof) = other;
        let classes = classes.weapon_classes.into_iter().collect();
        let prof = prof.into_iter()
            .map(|pw| Summary::<Weapon>::db_get_by_id(&pw.weapon_id, conn))
            .collect::<Result<_, DBError>>()?;
        let not_prof = not_prof.into_iter()
            .map(|npw| Summary::<Weapon>::db_get_by_id(&npw.weapon_id, conn))
            .collect::<Result<_, DBError>>()?;

        Ok(WeaponProficiencies{ classes, prof, not_prof })
    }
}

impl IntoDbWithId for WeaponProficiencies {
    type DBType = (DBClassProficientWeaponClass, Vec<DBClassProficientWeapon>, Vec<DBClassNotProficientWeapon>);

    fn into_db(self, id: Uuid) -> Self::DBType {
        let classes = DBClassProficientWeaponClass {
            class_id: id.clone(),
            weapon_classes: self.classes.into_iter().collect()
        };
        let prof = self.prof.into_iter()
            .map(|pw| DBClassProficientWeapon {
                class_id: id.clone(),
                weapon_id: pw.id().clone(),
            })
            .collect();
        let not_prof = self.not_prof.into_iter()
            .map(|npw| DBClassNotProficientWeapon {
                class_id: id,
                weapon_id: npw.id().clone(),
            })
            .collect();

        (classes, prof, not_prof)
    }
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(GetById, GetAll, Delete, DeleteById, Insert, Update)]
#[tavern(is_insertable, is_identifiable, is_queryable, id_field = "class_id")]
#[table_name = "classproficientweaponclasses"]
#[primary_key(class_id)]
#[belongs_to(DBClass, foreign_key = "class_id")]
pub struct DBClassProficientWeaponClass {
    class_id: Uuid,
    weapon_classes: Vec<WeaponClass>,
}

#[derive(Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(GetAll, Delete, Insert)]
#[tavern(is_insertable, is_identifiable, is_queryable)]
#[table_name = "classproficientweapons"]
#[primary_key(class_id, weapon_id)]
#[belongs_to(DBClass, foreign_key = "class_id")]
pub struct DBClassProficientWeapon {
    class_id: Uuid,
    weapon_id: Uuid,
}

#[derive(Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(GetAll, Delete, Insert)]
#[tavern(is_insertable, is_identifiable, is_queryable)]
#[table_name = "classnotproficientweapons"]
#[primary_key(class_id, weapon_id)]
#[belongs_to(DBClass, foreign_key = "class_id")]
pub struct DBClassNotProficientWeapon {
    class_id: Uuid,
    weapon_id: Uuid,
}

impl Proficiencies<Summary<Weapon>> for WeaponProficiencies {
    type Class = WeaponClass;

    fn add_class(&mut self, class: Self::Class) {
        self.classes.insert(class);
    }

    fn proficient(&mut self, item: Summary<Weapon>) {
        self.not_prof.remove(&item);
        self.prof.insert(item);
    }

    fn not_proficient(&mut self, item: Summary<Weapon>) {
        self.prof.remove(&item);
        self.not_prof.insert(item);
    }
}
