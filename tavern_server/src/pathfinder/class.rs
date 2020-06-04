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
use crate::db::{Connection, TryFromDb, IntoDb, IntoDbWithId, GetById, GetAll, Delete, DeleteById, Insert, Update, Error};

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

impl TryFromDb for Subclass {
    type DBType = DBSubclass;

    fn try_from_db(other: Self::DBType, conn: &Connection) -> Result<Self, Error> {
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
    fn get_features(&self, conn: &Connection) -> Result<Vec<Feature>, Error> {
        DBSubclassFeature::belonging_to(self)
            .load::<DBSubclassFeature>(conn)
            .map_err(Error::RunQuery)?
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

impl TryFromDb for Class {
    type DBType = DBClass;

    fn try_from_db(other: Self::DBType, conn: &Connection) -> Result<Self, Error> where Self: Sized {
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
    fn get_weapon_proficiencies(&self, conn: &Connection) -> Result<WeaponProficiencies, Error> {
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
                    err => return Err(Error::RunQuery(err)),
                }
            }
        };

        let prof = {
            DBClassProficientWeapon::belonging_to(self)
                .load::<DBClassProficientWeapon>(conn)
                .map_err(Error::RunQuery)?
        };

        let not_prof = {
            DBClassNotProficientWeapon::belonging_to(self)
                .load::<DBClassNotProficientWeapon>(conn)
                .map_err(Error::RunQuery)?
        };

        WeaponProficiencies::try_from_db((classes, prof, not_prof), conn)
    }

    fn get_armor_proficiencies(&self, conn: &Connection) -> Result<ArmorProficiencies, Error> {
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
                    err => return Err(Error::RunQuery(err)),
                }
            }
        };

        let prof = {
            DBClassProficientArmor::belonging_to(self)
                .load::<DBClassProficientArmor>(conn)
                .map_err(Error::RunQuery)?
        };

        let not_prof = {
            DBClassNotProficientArmor::belonging_to(self)
                .load::<DBClassNotProficientArmor>(conn)
                .map_err(Error::RunQuery)?
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

// This redundancy is due to the use of Summary<Feature> and the automatic
// impls of the traits defined in crate::db. This should make the type system
// happy and still provide those automatic impls.

impl TryFromDb for Feature {
    type DBType = DBFeature;

    fn try_from_db(other: Self::DBType, _conn: &Connection) -> Result<Self, Error> where Self: Sized {
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

    fn try_from_db(other: Self::DBType, conn: &Connection) -> Result<Self, Error> where Self: Sized {
        let (classes, prof, not_prof) = other;
        let classes = classes.armor_classes.into_iter().collect();
        let prof = prof.into_iter()
            .map(|pa| Summary::<Armor>::db_get_by_id(&pa.armor_id, conn))
            .collect::<Result<_, Error>>()?;
        let not_prof = not_prof.into_iter()
            .map(|npa| Summary::<Armor>::db_get_by_id(&npa.armor_id, conn))
            .collect::<Result<_, Error>>()?;

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

    fn try_from_db(other: Self::DBType, conn: &Connection) -> Result<Self, Error> where Self: Sized {
        let (classes, prof, not_prof) = other;
        let classes = classes.weapon_classes.into_iter().collect();
        let prof = prof.into_iter()
            .map(|pw| Summary::<Weapon>::db_get_by_id(&pw.weapon_id, conn))
            .collect::<Result<_, Error>>()?;
        let not_prof = not_prof.into_iter()
            .map(|npw| Summary::<Weapon>::db_get_by_id(&npw.weapon_id, conn))
            .collect::<Result<_, Error>>()?;

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
