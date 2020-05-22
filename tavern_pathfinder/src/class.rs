use crate::item::{Armor, ArmorClass, Weapon, WeaponClass};
use crate::spell::CasterType;
use crate::summary::{Summarize, Summary};
use crate::Attribute;
use crate::Links;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Subclass {
    id: Uuid,
    parent_class: Class,

    caster_type: CasterType,
    casting_attr: Attribute,
    features: Vec<Feature>,
}

#[derive(Serialize, Deserialize, Summarize)]
pub struct Class {
    links: Links,
    id: Uuid,
    subclasses: Vec<Subclass>,
    weapon_proficiencies: WeaponProficiencies,
    armor_proficiencies: ArmorProficiencies,
    name: String,
    description: String,
    hit_die: String,
    starting_wealth: String,
    bab_per_level: f32,
    skills_per_level: i32,
    skills_attr: Attribute,
}

#[derive(Serialize, Deserialize)]
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
pub struct EquipProficiencies<T, U>
where
    T: Ord + Eq,
    U: Ord + Eq,
{
    classes: BTreeSet<U>,
    prof: BTreeSet<T>,
    not_prof: BTreeSet<T>,
}

impl<T, U> Proficiencies<T> for EquipProficiencies<T, U>
where
    T: Ord + Eq,
    U: Ord + Eq,
{
    type Class = U;

    fn add_class(&mut self, class: Self::Class) {
        self.classes.insert(class);
    }

    fn proficient(&mut self, item: T) {
        self.not_prof.remove(&item);
        self.prof.insert(item);
    }

    fn not_proficient(&mut self, item: T) {
        self.prof.remove(&item);
        self.not_prof.insert(item);
    }
}

pub type WeaponProficiencies = EquipProficiencies<Summary<Weapon>, WeaponClass>;
pub type ArmorProficiencies = EquipProficiencies<Summary<Armor>, ArmorClass>;
