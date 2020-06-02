use super::item::{Armor, ArmorClass, DBArmor, DBWeapon, Weapon, WeaponClass};
use super::spell::CasterType;
use super::summary::{Summarize, Summary};
use super::Attribute;
use super::Links;

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use uuid::Uuid;

use crate::schema::{
    classes, classnotproficientarmor, classnotproficientweapons, classproficientarmor,
    classproficientarmorclasses, classproficientweaponclasses, classproficientweapons, features,
    subclasses, subclassfeatures,
};
use std::cmp::Ordering;

#[derive(Serialize, Deserialize, Summarize, Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct Subclass {
    links: Links,
    id: Uuid,
    name: String,
    description: String,
    parent_class: Summary<Class>,

    caster_type: CasterType,
    casting_attr: Attribute,

    features: Vec<Feature>,
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[table_name = "subclasses"]
#[belongs_to(DBClass, foreign_key = "class_id")]
pub struct DBSubclass {
    id: Uuid,
    name: String,
    description: String,
    class_id: Uuid,
    caster_type: CasterType,
    casting_attr: Attribute,
}

#[derive(Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[table_name = "subclassfeatures"]
#[primary_key(subclass_id, feature_id)]
#[belongs_to(DBSubclass, foreign_key = "subclass_id")]
pub struct DBSubclassFeatures {
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
    AsChangeset,
    Associations,
    Identifiable,
    Insertable,
    Queryable, Clone, Ord, PartialOrd, PartialEq, Eq,
)]
#[table_name = "features"]
pub struct Feature {
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

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[table_name = "classproficientweaponclasses"]
#[primary_key(class_id)]
#[belongs_to(DBClass, foreign_key = "class_id")]
pub struct DBClassProficientWeaponClass {
    class_id: Uuid,
    weapon_classes: Vec<WeaponClass>,
}

#[derive(Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[table_name = "classproficientweapons"]
#[primary_key(class_id, weapon_id)]
#[belongs_to(DBClass, foreign_key = "class_id")]
pub struct DBClassProficientWeapon {
    class_id: Uuid,
    weapon_id: Uuid,
}

#[derive(Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
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
