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
#[cfg( feature = "tavern")]
use tavern_db::{TryFromRow, TryFromUuid};
#[cfg( feature = "tavern")]
use crate::effects::DBEffect;
#[cfg( feature = "tavern")]
use crate::character::DBCharacter;
#[cfg(feature = "tavern")]
use diesel_derive_enum::DbEnum;

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

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "Items")]
#[cfg_attr(feature = "tavern", belongs_to(DBItem, foreign_key = "item_id"))]
pub struct DBItem {
    id: Uuid,
    name: String,
    description: String,
    cost: i32,
    weight: f64,
    equip_slot: Option<EquipmentSlot>,
    consumed_effects: Vec<Summary<Effect>>,
}

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "Bags")]
#[cfg_attr(feature = "tavern", belongs_to(DBItem, foreign_key = "item_id"))]
#[cfg_attr(feature = "tavern", belongs_to(DBEffect, foreign_key = "effect_id"))]
pub struct DBItemEffects {
    item_id: Uuid,
    effect_id: Uuid,
    is_permanent: bool,
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

#[derive(Serialize, Deserialize, Summarize)]
pub struct Bag {
    id: Uuid,
    links: Links,
    name: String,
    character: Summary<Character>,
    item: Summary<Item>,
    contents: BTreeMap<Summary<Item>, i32>,
    capacity: i32,
    #[serde(skip)]
    description: String,
}

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "Bags")]
#[cfg_attr(feature = "tavern", belongs_to(DBItem, foreign_key = "item_id"))]
#[cfg_attr(feature = "tavern", belongs_to(DBCharacter, foreign_key = "character_id"))]
pub struct DBBag {
    id: Uuid,
    name: String,
    character_id: Uuid,
    item_id: Uuid,
    capacity: i32,
    #[serde(skip)]
    description: String,
}

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "ItemsInBags")]
#[cfg_attr(feature = "tavern", belongs_to(DBItem, foreign_key = "item_id"))]
#[cfg_attr(feature = "tavern", belongs_to(DBBag, foreign_key = "bag_id"))]
pub struct DBItemInBag {
    item_id: Uuid,
    bag_id: Uuid,
    count: i32,
}

impl Bag {
    fn update_desc(&mut self) {
        let size: i32 = self.contents.iter()
            .map(|(_, count)| count)
            .sum();
        self.description = format!("{} {}/{}", self.name, size, self.capacity);
    }
}

#[derive(Serialize, Deserialize, Display, PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
#[cfg_attr(feature = "tavern", derive(DbEnum))]
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

#[derive(Serialize, Deserialize, Display, PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
#[cfg_attr(feature = "tavern", derive(DbEnum))]
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

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "Weapons")]
#[cfg_attr(feature = "tavern", belongs_to(DBItem, foreign_key = "item_id"))]
#[cfg_attr(feature = "tavern", belongs_to(DBMaterial, foreign_key = "material_id"))]
struct DBWeapon {
    item_id: Uuid,
    material_id: Option<Uuid>,
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

    fn links(&self) -> &Links {
        &self.item.links
    }

    fn name(&self) -> &str {
        &self.item.name
    }

    fn description(&self) -> &str {
        &self.item.description
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

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "Armor")]
#[cfg_attr(feature = "tavern", belongs_to(DBItem, foreign_key = "item_id"))]
#[cfg_attr(feature = "tavern", belongs_to(DBMaterial, foreign_key = "material_id"))]
struct DBArmor {
    item_id: Uuid,
    material_id: Option<Uuid>,
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

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "Materials")]
struct DBMaterial {
    id: Uuid,
    name: String,
    description: String,
    hp_per_inch: Option<i32>,
    hardness: Option<i32>,
}