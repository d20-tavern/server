use crate::item::{Armor, ArmorClass, Weapon, WeaponClass};
use crate::spell::CasterType;
use crate::summary::{Summarize, Summary};
use crate::Attribute;
use crate::Links;
use futures::executor::block_on;

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use uuid::Uuid;
#[cfg(feature = "tavern")]
use tavern_db::{TryFromRow, TryFromUuid};

#[derive(Serialize, Deserialize, Summarize)]
#[cfg_attr(feature = "tavern", derive(TryFromRow, TryFromUuid))]
pub struct Subclass {
    #[cfg_attr(feature = "tavern", tavern(skip, default = "Links::new()"))]
    links: Links,
    id: Uuid,
    name: String,
    description: String,
    #[cfg_attr(feature = "tavern", tavern(
        references = "Summary<Class>",
        column_name = "class_id",
    ))]
    parent_class: Summary<Class>,

    caster_type: CasterType,
    casting_attr: Attribute,

    #[cfg_attr(feature = "tavern", tavern(
        references = "Feature",
        column = "ARRAY(SELECT ROW(feature_id) FROM SubclassFeatures WHERE SubclassFeatures.subclass_id = $1)",
        is_array,
    ))]
    features: Vec<Feature>,
}

#[derive(Serialize, Deserialize, Summarize)]
#[cfg_attr(feature = "tavern", derive(TryFromRow, TryFromUuid))]
pub struct Class {
    #[cfg_attr(feature = "tavern", tavern(skip, default = "Links::new()"))]
    links: Links,
    id: Uuid,
    //subclasses: Vec<Subclass>,
    #[cfg_attr(feature = "tavern", tavern(
        references = "WeaponProficiencies",
        column = "id"
    ))]
    weapon_proficiencies: WeaponProficiencies,
    #[cfg_attr(feature = "tavern", tavern(
        references = "ArmorProficiencies",
        column = "id"
    ))]
    armor_proficiencies: ArmorProficiencies,
    name: String,
    description: String,
    hit_die: String,
    starting_wealth: String,
    bab_per_level: f64,
    skills_per_level: i16,
    skills_attr: Attribute,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "tavern", derive(TryFromRow, TryFromUuid))]
pub struct Feature {
    id: Uuid,
    description: String,
}

pub trait Proficiencies<T> {
    type Class;
    fn add_class(&mut self, class: Self::Class);
    fn proficient(&mut self, item: T);
    fn not_proficient(&mut self, item: T);
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "tavern", derive(TryFromRow, TryFromUuid))]
#[cfg_attr(feature = "tavern", tavern(
    table_name = "ClassProficientArmorClasses",
    id_column = "class_id",
))]
pub struct ArmorProficiencies {
    #[cfg_attr(feature = "tavern", tavern(
        is_set,
        tuple_hack = "ArmorClass",
        column_name = "armor_classes",
    ))]
    classes: BTreeSet<ArmorClass>,
    #[cfg_attr(feature = "tavern", tavern(
        is_set,
        references = "Summary<Armor>",
        column_name = "prof",
        column = "ARRAY(SELECT armor_id FROM ClassProficientArmor WHERE ClassProficientArmor.class_id = $1)",
    ))]
    prof: BTreeSet<Summary<Armor>>,
    #[cfg_attr(feature = "tavern", tavern(
        is_set,
        references = "Summary<Armor>",
        column_name = "not_prof",
        column = "ARRAY(SELECT armor_id FROM ClassNotProficientArmor WHERE ClassNotProficientArmor.class_id = $1)",
    ))]
    not_prof: BTreeSet<Summary<Armor>>,
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
#[cfg_attr(feature = "tavern", derive(TryFromRow, TryFromUuid))]
#[cfg_attr(feature = "tavern", tavern(
    table_name = "ClassProficientWeaponClasses",
    id_column = "class_id",
))]
pub struct WeaponProficiencies {
    #[cfg_attr(feature = "tavern", tavern(
        is_set,
        tuple_hack = "WeaponClass",
        column_name = "weapon_classes",
    ))]
    classes: BTreeSet<WeaponClass>,
    #[cfg_attr(feature = "tavern", tavern(
        is_set,
        references = "Summary<Weapon>",
        column_name = "prof",
        column = "ARRAY(SELECT weapon_id FROM ClassProficientWeapons WHERE ClassProficientWeapons.class_id = $1)",
    ))]
    prof: BTreeSet<Summary<Weapon>>,
    #[cfg_attr(feature = "tavern", tavern(
        is_set,
        references = "Summary<Weapon>",
        column_name = "not_prof",
        column = "ARRAY(SELECT weapon_id FROM ClassNotProficientWeapons WHERE ClassNotProficientWeapons.class_id = $1)",
    ))]
    not_prof: BTreeSet<Summary<Weapon>>,
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