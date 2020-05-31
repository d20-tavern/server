use serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::collections::BTreeMap;
use std::ops::Bound;
use uuid::Uuid;

use super::character::{Character, DBCharacter};
use super::effects::{DBEffect, Effect};
use super::summary::{Summarize, Summary};
use super::{DamageType, EquipmentSlot, Links};

use crate::schema::{armor, bags, itemeffects, items, itemsinbags, materials, weapons};
use diesel::pg::types::sql_types::Range;
use diesel::sql_types::SmallInt;
use diesel_derive_enum::DbEnum;
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

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[table_name = "items"]
#[belongs_to(DBItem, foreign_key = "id")]
pub struct DBItem {
    id: Uuid,
    name: String,
    description: String,
    cost: i32,
    weight: f64,
    equip_slot: Option<EquipmentSlot>,
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[table_name = "itemeffects"]
#[primary_key(item_id, effect_id)]
#[belongs_to(DBItem, foreign_key = "item_id")]
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
        Some(self.cmp(&other))
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

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[table_name = "bags"]
#[belongs_to(DBCharacter, foreign_key = "char_id")]
pub struct DBBag {
    id: Uuid,
    name: String,
    char_id: Uuid,
    item_id: Uuid,
    capacity: i32,
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[table_name = "itemsinbags"]
#[primary_key(item_id, bag_id)]
#[belongs_to(DBBag, foreign_key = "bag_id")]
pub struct DBItemInBag {
    item_id: Uuid,
    bag_id: Uuid,
    count: i32,
}

impl Bag {
    fn update_desc(&mut self) {
        let size: i32 = self.contents.iter().map(|(_, count)| count).sum();
        self.description = format!("{} {}/{}", self.name, size, self.capacity);
    }
}

#[derive(
    Serialize, Deserialize, Display, PartialEq, PartialOrd, Eq, Ord, Copy, Clone, DbEnum, Debug,
)]
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

#[derive(
    Serialize, Deserialize, Display, PartialEq, PartialOrd, Eq, Ord, Copy, Clone, DbEnum, Debug,
)]
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
    crit_range: std::ops::Range<i32>,
    damage: Vec<String>,
    damage_type: Vec<DamageType>,
    weapon_type: WeaponClass,
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[table_name = "weapons"]
#[belongs_to(DBItem, foreign_key = "id")]
pub(crate) struct DBWeapon {
    id: Uuid,
    material_id: Option<Uuid>,
    crit_range: (Bound<i16>, Bound<i16>),
    damage: Vec<String>,
    damage_type: Vec<DamageType>,
    weapon_type: WeaponClass,
}

impl Summarize<Weapon> for Weapon {
    fn id(&self) -> &Uuid {
        &self.item.id
    }

    fn links(&self) -> Option<&Links> {
        Some(&self.item.links)
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

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[table_name = "armor"]
#[belongs_to(DBItem, foreign_key = "id")]
pub(crate) struct DBArmor {
    id: Uuid,
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

    fn links(&self) -> Option<&Links> {
        Some(&self.item.links)
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

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[table_name = "materials"]
struct DBMaterial {
    id: Uuid,
    name: String,
    description: String,
    hp_per_inch: Option<i32>,
    hardness: Option<i32>,
}