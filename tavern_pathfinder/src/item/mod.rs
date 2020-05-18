use serde::{Serialize,Deserialize};
use crate::character;
use uuid::Uuid;

use crate::EquipmentSlot;
use crate::DamageType;
use crate::WeaponClass;
use crate::ArmorClass;

use crate::effects;

#[derive(Serialize,Deserialize)]
pub struct ItemSummary {
    id: Uuid,
    name: String,
    description: String,
}

#[derive(Serialize,Deserialize)]
pub struct Item {
    id: Uuid,

    name: String,
    description: String,
    cost: u32,
    weight: u32,
    equip_slot: Option<EquipmentSlot>,
    consumed_effects: Option<Vec<effects::Effect>>,
}

#[derive(Serialize,Deserialize)]
pub struct Bag {
    id: Uuid,
    character: character::CharacterSummary,
    contents: Vec<Item>,

    capacity: u32,
}

#[derive(Serialize,Deserialize)]
pub struct Weapon {
    item: Item,
    material: Option<Material>,

    weapon_range: std::ops::Range<u32>,
    crit_range: std::ops::Range<u32>,
    damage: Vec<String>,
    damage_type: Vec<DamageType>,
    weapon_type: WeaponClass,
}

#[derive(Serialize,Deserialize)]
pub struct Armor {
    item: Item,
    material: Option<Material>,

    max_dex_bonus: u32,
    ac: u32,
    spell_failure: u32,
    check_penalty: u32,
    armor_type: ArmorClass,
}

#[derive(Serialize,Deserialize)]
pub struct Material {
    id: Uuid,

    name: String,
    description: String,

    hp_per_inch: Option<u32>,
    hardness: Option<u32>,
    
}
