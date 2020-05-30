use crate::item::{Armor, ArmorClass, Weapon, WeaponClass};
use crate::spell::CasterType;
use crate::summary::{Summarize, Summary};
use crate::Attribute;
use crate::Links;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use uuid::Uuid;
#[cfg(feature = "tavern")]
use tavern_db::{TryFromRow, TryFromUuid};

#[derive(Serialize, Deserialize, Summarize)]
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

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "Subclasses")]
#[cfg_attr(feature = "tavern", belongs_to(DBClass, foreign_key = "class_id"))]
pub struct DBSubclass {
    id: Uuid,
    name: String,
    description: String,
    class_id: Uuid,
    caster_type: CasterType,
    casting_attr: Attribute,
}

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "SubclassFeatures")]
#[cfg_attr(feature = "tavern", belongs_to(DBSubclass, foreign_key = "subclass_id"))]
#[cfg_attr(feature = "tavern", belongs_to(DBFeature, foreign_key = "feature_id"))]
pub struct DBSubclassFeatures {
    subclass_id: Uuid,
    feature_id: Uuid,
}

#[derive(Serialize, Deserialize, Summarize)]
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

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "Classes")]
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

#[derive(Serialize, Deserialize, Summarize)]
#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "Features")]
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

#[derive(Serialize, Deserialize)]
pub struct ArmorProficiencies {
    classes: BTreeSet<ArmorClass>,
    prof: BTreeSet<Summary<Armor>>,
    not_prof: BTreeSet<Summary<Armor>>,
}

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "ClassProficientArmorClasses")]
#[cfg_attr(feature = "tavern", belongs_to(DBClass, foreign_key = "class_id"))]
pub struct DBClassProficientArmorClass {
    class_id: Uuid,
    armor_classes: Vec<ArmorClass>,
}

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "ClassProficientArmor")]
#[cfg_attr(feature = "tavern", belongs_to(DBClass, foreign_key = "class_id"))]
#[cfg_attr(feature = "tavern", belongs_to(DBArmor, foreign_key = "armor_id"))]
pub struct DBClassProficientArmor {
    class_id: Uuid,
    armor_id: Uuid,
}

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "ClassProficientArmor")]
#[cfg_attr(feature = "tavern", belongs_to(DBClass, foreign_key = "class_id"))]
#[cfg_attr(feature = "tavern", belongs_to(DBArmor, foreign_key = "armor_id"))]
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

#[derive(Serialize, Deserialize)]
pub struct WeaponProficiencies {
    classes: BTreeSet<WeaponClass>,
    prof: BTreeSet<Summary<Weapon>>,
    not_prof: BTreeSet<Summary<Weapon>>,
}

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "ClassProficientWeaponClasses")]
#[cfg_attr(feature = "tavern", belongs_to(DBClass, foreign_key = "class_id"))]
pub struct DBClassProficientWeaponClass {
    class_id: Uuid,
    weapon_classes: Vec<WeaponClass>,
}

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "ClassProficientWeapons")]
#[cfg_attr(feature = "tavern", belongs_to(DBClass, foreign_key = "class_id"))]
#[cfg_attr(feature = "tavern", belongs_to(DBWeapon, foreign_key = "weapon_id"))]
pub struct DBClassProficientWeapon {
    class_id: Uuid,
    weapon_id: Uuid,
}

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "ClassProficientWeapons")]
#[cfg_attr(feature = "tavern", belongs_to(DBClass, foreign_key = "class_id"))]
#[cfg_attr(feature = "tavern", belongs_to(DBWeapon, foreign_key = "weapon_id"))]
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