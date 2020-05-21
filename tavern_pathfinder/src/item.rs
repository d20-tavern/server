use crate::Links;
use serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::collections::BTreeMap;
use uuid::Uuid;

use crate::DamageType;
use crate::EquipmentSlot;

use crate::character::Character;
use crate::effects::Effect;
use crate::summary::{Summarize, Summary};

#[derive(Serialize, Deserialize, Summarize)]
pub struct Item {
    links: Links,
    id: Uuid,

    name: String,
    description: String,
    cost: u32,
    weight: u32,
    equip_slot: Option<EquipmentSlot>,
    consumed_effects: Vec<Summary<Effect>>,
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name
            .cmp(&other.name)
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Item {}

#[derive(Serialize, Deserialize)]
pub struct Bag {
    id: Uuid,
    name: String,
    character: Summary<Character>,
    contents: BTreeMap<Item, u32>,
    capacity: u32,
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
pub enum WeaponClass {
    Axes,
    HeavyBlades,
    LightBlades,
    Bows,
    Close,
    Crossbows,
    Double,
    Firearms,
    Flails,
    Hammers,
    Monk,
    Natural,
    Polearms,
    SiegeEngines,
    Spears,
    Thrown,
    Tribal,
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
pub enum ArmorClass {
    Light,
    Medium,
    Heavy,
}

// TODO: Impl Summarize for Weapon and Armor

#[derive(Serialize, Deserialize)]
pub struct Weapon {
    #[serde(flatten)]
    item: Item,
    material: Option<Material>,
    weapon_range: std::ops::Range<u32>,
    crit_range: std::ops::Range<u32>,
    damage: Vec<String>,
    damage_type: Vec<DamageType>,
    weapon_type: WeaponClass,
}

#[derive(Serialize, Deserialize)]
pub struct Armor {
    #[serde(flatten)]
    item: Item,
    material: Option<Material>,
    max_dex_bonus: u32,
    ac: u32,
    spell_failure: u32,
    check_penalty: u32,
    armor_type: ArmorClass,
}

#[derive(Serialize, Deserialize, Summarize)]
pub struct Material {
    id: Uuid,
    links: Links,
    name: String,
    description: String,
    hp_per_inch: Option<u32>,
    hardness: Option<u32>,
}
