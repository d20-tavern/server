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
use tavern_derive::Display;

#[derive(Serialize, Deserialize, Summarize)]
pub struct Item {
    links: Links,
    id: Uuid,

    name: String,
    description: String,
    cost: i32,
    weight: f64,
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
    contents: BTreeMap<Item, i32>,
    capacity: i32,
}

#[derive(Serialize, Deserialize, Display, PartialEq, PartialOrd, Eq, Ord)]
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

#[derive(Serialize, Deserialize, Display, PartialEq, PartialOrd, Eq, Ord)]
pub enum ArmorClass {
    Light,
    Medium,
    Heavy,
}

#[derive(Serialize, Deserialize)]
pub struct Weapon {
    #[serde(flatten)]
    item: Item,
    material: Option<Material>,
    weapon_range: std::ops::Range<i32>,
    crit_range: std::ops::Range<i32>,
    damage: Vec<String>,
    damage_type: Vec<DamageType>,
    weapon_type: WeaponClass,
}

impl Summarize<Weapon> for Weapon {
    fn id(&self) -> &Uuid {
        &self.item.id
    }

    fn name(&self) -> &str {
        &self.item.name
    }

    fn description(&self) -> &str {
        &self.item.description
    }

    fn links(&self) -> &Links {
        &self.item.links
    }
}

#[derive(Serialize, Deserialize)]
pub struct Armor {
    #[serde(flatten)]
    item: Item,
    material: Option<Material>,
    max_dex_bonus: i32,
    ac: i32,
    spell_failure: i32,
    check_penalty: i32,
    armor_type: ArmorClass,
}

impl Summarize<Armor> for Armor {
    fn id(&self) -> &Uuid {
        &self.item.id
    }

    fn name(&self) -> &str {
        &self.item.name
    }

    fn description(&self) -> &str {
        &self.item.description
    }

    fn links(&self) -> &Links {
        &self.item.links
    }
}

#[derive(Serialize, Deserialize, Summarize)]
pub struct Material {
    id: Uuid,
    links: Links,
    name: String,
    description: String,
    hp_per_inch: Option<i32>,
    hardness: Option<i32>,
}
