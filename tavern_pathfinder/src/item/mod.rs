use serde::{Serialize,Deserialize};
use crate::character;
use uuid::Uuid;

#[derive(Serialize,Deserialize)]
pub struct Item {
    item_id: Uuid,

    name: String,
    description: String,
    cost: u32,
    weight: u32,
    equip_slot: crate::EquipmentSlot,
}

#[derive(Serialize,Deserialize)]
pub struct Bag {
    bag_id: Uuid,
    character: character::Character,
    item: Item,

    capacity: u32,
}

#[derive(Serialize,Deserialize)]
pub struct Weapon {
    item: Item,
    material: Material,

    weapon_range: std::ops::Range<u32>,
    crit_range: std::ops::Range<u32>,
    damage: Vec<String>,
    damage_type: Vec<crate::DamageType>,
    weapon_type: crate::WeaponClass,
}

#[derive(Serialize,Deserialize)]
pub struct Armor {
    item: Item,
    material: Material,

    max_dex_bonus: u32,
    ac: u32,
    spell_failure: u32,
    check_penalty: u32,
    armor_type: crate::ArmorClass,
}

#[derive(Serialize,Deserialize)]
pub struct Material {
    material: Uuid,

    name: String,
    description: String,

    hp_per_inch: u32,
    hardness: u32,
    
}
