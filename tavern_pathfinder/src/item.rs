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

#[derive(Serialize, Deserialize, Summarize)]
#[cfg_attr(feature = "tavern", derive(TryFromRow, TryFromUuid))]
pub struct Item {
    #[cfg_attr(feature = "tavern", tavern(
        skip, default = "Links::new()"
    ))]
    links: Links,
    id: Uuid,

    name: String,
    description: String,
    cost: i32,
    weight: f64,

    #[cfg_attr(feature = "tavern", tavern(is_optional))]
    equip_slot: Option<EquipmentSlot>,
    #[cfg_attr(feature = "tavern", tavern(
        references = "Summary<Effect>",
        column = "ARRAY(SELECT ROW(effect_id) FROM ItemEffects WHERE ItemEffects.item_id = $1)",
        is_array
    ))]
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

#[cfg_attr(feature = "tavern", tavern(
select_post_op = "instance.update_desc();",
table_name = "Bags",
))]
#[cfg_attr(feature = "tavern", derive(TryFromRow, TryFromUuid))]
#[derive(Serialize, Deserialize, Summarize)]
pub struct Bag {
    id: Uuid,
    #[cfg_attr(feature = "tavern", tavern(
        skip, default = "Links::new()",
    ))]
    links: Links,
    name: String,
    #[cfg_attr(feature = "tavern", tavern(
        references = "Summary<Character>",
        column_name = "char_id",
    ))]
    character: Summary<Character>,
    #[cfg_attr(feature = "tavern", tavern(
        references = "Summary<Item>",
        column_name = "item_id",
    ))]
    item: Summary<Item>,
    #[cfg_attr(feature = "tavern", tavern(
        references = "BTreeMap<Summary<Item>, i32>",
        column = "ARRAY(SELECT ROW(item_id, count) FROM ItemsInBags WHERE ItemsInBags.bag_id = $1)",
        key_references = "Summary<Item>",
        is_map,
    ))]
    contents: BTreeMap<Summary<Item>, i32>,
    capacity: i32,
    #[serde(skip)]
    #[cfg_attr(feature = "tavern", tavern(skip, default = "String::new()"))]
    description: String,
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
#[cfg_attr(feature = "tavern", derive(sqlx::Type))]
pub enum WeaponClass {
    Axes,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Heavy Blades"))]
    HeavyBlades,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Light Blades"))]
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
    #[cfg_attr(feature = "tavern", sqlx(rename = "Siege Engines"))]
    SiegeEngines,
    Spears,
    Thrown,
    Tribal,
}

#[derive(Serialize, Deserialize, Display, PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
#[cfg_attr(feature = "tavern", derive(sqlx::Type))]
pub enum ArmorClass {
    Light,
    Medium,
    Heavy,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "tavern", derive(TryFromRow, TryFromUuid))]
pub struct Weapon {
    #[serde(flatten)]
    #[cfg_attr(feature = "tavern", tavern(
        references = "Item",
        column_name = "id"
    ))]
    item: Item,
    #[cfg_attr(feature = "tavern", tavern(
        references = "Material",
        column_name = "material_id",
        is_optional,
    ))]
    material: Option<Material>,
    weapon_range: std::ops::Range<i32>,
    crit_range: std::ops::Range<i32>,
    damage: Vec<String>,

    #[cfg_attr(feature = "tavern", tavern(
        tuple_hack = "DamageType",
        is_array,
    ))]
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
#[cfg_attr(feature = "tavern", derive(TryFromRow, TryFromUuid))]
pub struct Armor {
    #[serde(flatten)]
    #[cfg_attr(feature = "tavern", tavern(
        references = "Item",
        column_name = "id"
    ))]
    item: Item,
    #[cfg_attr(feature = "tavern", tavern(
        references = "Material",
        column_name = "material_id",
        is_optional,
    ))]
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
#[cfg_attr(feature = "tavern", derive(TryFromRow, TryFromUuid))]
pub struct Material {
    id: Uuid,
    #[cfg_attr(feature = "tavern", tavern(
        skip, default = "Links::new()"
    ))]
    links: Links,
    name: String,
    description: String,
    #[cfg_attr(feature = "tavern", tavern(is_optional))]
    hp_per_inch: Option<i32>,
    #[cfg_attr(feature = "tavern", tavern(is_optional))]
    hardness: Option<i32>,
}
