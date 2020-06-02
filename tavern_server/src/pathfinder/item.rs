use serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::collections::{BTreeMap, BTreeSet};
use std::ops::{Bound, Range};
use uuid::Uuid;

use super::character::{Character, DBCharacter};
use super::effects::{DBEffect, Effect};
use super::summary::{Summarize, Summary};
use super::{DamageType, EquipmentSlot, Links};

use crate::schema::{armor, bags, itemeffects, items, itemsinbags, materials, weapons};
use diesel::prelude::*;
use diesel::sql_types::SmallInt;
use diesel::associations::BelongsTo;
use diesel_derive_enum::DbEnum;
use tavern_derive::Display;
use crate::db::{GetById, GetAll, DeleteById, Delete, Insert, Update, Connection, Error, StandaloneDbMarker, IntoDb, TryFromDb, IntoDbWithId};
use diesel::Connection as DieselConnection;

#[derive(Serialize, Deserialize, Summarize, Clone)]
pub struct Item {
    links: Links,
    id: Uuid,

    name: String,
    description: String,
    cost: i32,
    weight: f64,

    equip_slot: Option<EquipmentSlot>,
    consumed_effects: BTreeSet<ItemEffect>,
}

impl TryFromDb for Item {
    type DBType = DBItem;

    fn try_from_db(other: Self::DBType, conn: &Connection) -> Result<Self, Error> where Self: Sized {
        let mut links = Links::new();
        let consumed_effects = other.get_effects(conn)?;
        let item = Item {
            id: other.id,
            links,
            name: other.name,
            description: other.description,
            cost: other.cost,
            weight: other.weight,
            equip_slot: other.equip_slot,
            consumed_effects,
        };
        Ok(item)
    }
}

impl IntoDb for Item {
    type DBType = (DBItem, BTreeSet<DBItemEffect>);

    fn into_db(self) -> Self::DBType {
        let item = DBItem {
            id: self.id.clone(),
            name: self.name,
            description: self.description,
            cost: self.cost,
            weight: self.weight,
            equip_slot: self.equip_slot
        };

        let effects = self.consumed_effects.into_iter()
            .map(|item| ItemEffect::into_db(item, self.id))
            .collect();

        (item, effects)
    }
}

impl Delete for Item {
    fn db_delete(&self, conn: &Connection) -> Result<(), Error> {
        Self::db_delete_by_id(&self.id, conn)
    }
}

impl DeleteById for Item {
    fn db_delete_by_id(id: &Uuid, conn: &Connection) -> Result<(), Error> {
        conn.transaction::<_, Error, _>(|| {
            use crate::schema::itemeffects::dsl::*;
            DBItem::db_delete_by_id(id, conn)?;
            diesel::delete(itemeffects.filter(item_id.eq(id)))
                .execute(conn)
                .map_err(Error::RunQuery)
                .map(|_| ())
        })
    }
}

impl Insert for Item {
    fn db_insert(&self, conn: &Connection) -> Result<(), Error> {
        conn.transaction::<_, Error, _>(|| {
            let (item, effects) = self.to_owned().into_db();
            item.db_insert(conn)?;
            for effect in effects.into_iter() {
                effect.db_insert(conn)?;
            }
            Ok(())
        })
    }
}

impl Update for Item {
    fn db_update(&self, conn: &Connection) -> Result<(), Error> {
        conn.transaction::<_, Error, _>(|| {
            let (item, effects) = self.to_owned().into_db();
            item.db_update(conn)?;

            let old_effects = item.get_effects(conn)?;
            let delete_effects = old_effects.difference(&self.consumed_effects)
                .map(|item_effect| item_effect.into_db(self.id.to_owned()));
            let add_effects = self.consumed_effects.difference(&old_effects)
                .map(|item_effect| item_effect.into_db(self.id.to_owned()));

            for effect in delete_effects {
                effect.db_delete(conn)?;
            }
            for effect in add_effects {
                effect.db_insert(conn)?;
            }

            Ok(())
        })
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
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

impl Eq for Item{}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone)]
#[derive(GetAll, GetById, Delete, DeleteById, Insert, Update)]
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

impl DBItem {
    fn get_effects(&self, conn: &Connection) -> Result<BTreeSet<ItemEffect>, Error> {
        DBItemEffect::belonging_to(self)
            .load::<DBItemEffect>(conn)
            .map_err(Error::RunQuery)?
            .into_iter()
            .map(|item_effect| ItemEffect::try_from_db(item_effect, conn))
            .collect()
    }
}

impl Ord for DBItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for DBItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for DBItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for DBItem{}

#[derive(Serialize, Deserialize, Clone, Ord, PartialOrd, PartialEq, Eq, StandaloneDbMarker)]
pub struct ItemEffect {
    effect: Summary<Effect>,
    is_permanent: bool,
}

impl TryFromDb for ItemEffect {
    type DBType = DBItemEffect;

    fn try_from_db(other: Self::DBType, conn: &Connection) -> Result<Self, Error> where Self: Sized {
        let effect = Summary::<Effect>::db_get_by_id(&other.effect_id, conn)?;
        let item_effect = ItemEffect {
            effect,
            is_permanent: other.is_permanent,
        };
        Ok(item_effect)
    }
}

impl IntoDbWithId for ItemEffect {
    type DBType = DBItemEffect;

    fn into_db(self, item_id: Uuid) -> Self::DBType {
        DBItemEffect {
            item_id,
            effect_id: self.effect.id().to_owned(),
            is_permanent: self.is_permanent,
        }
    }
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(Delete, Insert)]
#[tavern(is_identifiable, is_insertable, is_queryable)]
#[table_name = "itemeffects"]
#[primary_key(item_id, effect_id)]
#[belongs_to(DBItem, foreign_key = "item_id")]
pub struct DBItemEffect {
    item_id: Uuid,
    effect_id: Uuid,
    is_permanent: bool,
}

#[derive(Serialize, Deserialize, Summarize, Clone)]
pub struct Bag {
    id: Uuid,
    links: Links,
    name: String,
    character: Summary<Character>,
    item: Summary<Item>,
    contents: BTreeSet<ItemInBag>,
    capacity: i32,
    #[serde(skip)]
    description: String,
}

impl Bag {
    fn update_desc(&mut self) {
        let size: i32 = self.contents.iter().map(|item| item.count).sum();
        self.description = format!("{} {}/{}", self.name, size, self.capacity);
    }
}

impl TryFromDb for Bag {
    type DBType = DBBag;

    fn try_from_db(other: Self::DBType, conn: &Connection) -> Result<Self, Error> where Self: Sized {
        let item = Summary::<Item>::db_get_by_id(&other.item_id, conn)?;
        let character = Summary::<Character>::db_get_by_id(&other.char_id, conn)?;
        let contents = DBItemInBag::belonging_to(&other)
            .load::<DBItemInBag>(conn)
            .map_err(Error::RunQuery)?
            .into_iter()
            .map(|item_in_bag| ItemInBag::try_from_db(item_in_bag, conn))
            .collect()?;
        let mut links = Links::new();
        links.insert("character".to_string(), format!("/characters/{}", other.char_id));

        let mut bag = Bag {
            id: other.id,
            links,
            name: other.name,
            character,
            item,
            contents,
            capacity: other.capacity,
            description: String::new(),
        };
        bag.update_desc();
        Ok(bag)
    }
}

impl IntoDb for Bag {
    type DBType = (DBBag, BTreeSet<DBItemInBag>);

    fn into_db(self) -> Self::DBType {
        let db_bag = DBBag {
            id: self.id.clone(),
            name: self.name,
            char_id: self.character.id().to_owned(),
            item_id: self.item.id().to_owned(),
            capacity: self.capacity,
        };

        let contents = self.contents.into_iter()
            .map(|item| item.into_db(self.id))
            .collect();

        (db_bag, contents)
    }
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[table_name = "bags"]
#[belongs_to(DBCharacter, foreign_key = "char_id")]
pub struct DBBag {
    id: Uuid,
    name: String,
    char_id: Uuid,
    item_id: Uuid,
    capacity: i32,
}

#[derive(Serialize, Deserialize, StandaloneDbMarker, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ItemInBag {
    pub item: Summary<Item>,
    pub count: i32,
}

impl TryFromDb for ItemInBag {
    type DBType = DBItemInBag;

    fn try_from_db(other: Self::DBType, conn: &Connection) -> Result<Self, Error> where Self: Sized {
        let count = other.count;
        let item = Summary::<Item>::db_get_by_id(&other.item_id, conn)?;
        let bag_item = ItemInBag {
            item,
            count,
        };
        Ok(bag_item)
    }
}

impl IntoDbWithId for ItemInBag {
    type DBType = DBItemInBag;

    fn into_db(self, bag_id: Uuid) -> Self::DBType {
        DBItemInBag {
            item_id: self.item.id().to_owned(),
            bag_id,
            count: self.count,
        }
    }
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[table_name = "itemsinbags"]
#[primary_key(item_id, bag_id)]
#[belongs_to(DBBag, foreign_key = "bag_id")]
pub struct DBItemInBag {
    item_id: Uuid,
    bag_id: Uuid,
    count: i32,
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

#[derive(Serialize, Deserialize, Clone, StandaloneDbMarker)]
pub struct Weapon {
    #[serde(flatten)]
    item: Item,
    material: Option<Material>,
    crit_range: std::ops::Range<i32>,
    damage: Vec<String>,
    damage_type: Vec<DamageType>,
    weapon_type: WeaponClass,
}

impl TryFromDb for Weapon {
    type DBType = DBWeapon;

    fn try_from_db(other: Self::DBType, conn: &Connection) -> Result<Self, Error> where Self: Sized {
        let item = Item::db_get_by_id(&other.id, conn)?;
        let material = other.material_id.map(|id| Material::db_get_by_id(&id, conn)).transpose()?;
        let crit_range = {
            let (start, end) = other.crit_range;
            let start = match start {
                Bound::Included(val) => val,
                Bound::Excluded(val) => val - 1,
                Bound::Unbounded => return Err(Error::Other("start cannot be unbounded".to_string()))
            };
            let end = match end {
                Bound::Included(val) => val,
                Bound::Excluded(val) => val - 1,
                Bound::Unbounded => return Err(Error::Other("end cannot be unbounded".to_string()))
            };
            Range { start, end }
        };
        let weapon = Weapon {
            item,
            material,
            crit_range,
            damage: other.damage,
            damage_type: other.damage_type,
            weapon_type: other.weapon_type,
        };
        Ok(weapon)
    }
}

impl IntoDb for Weapon {
    type DBType = DBWeapon;

    fn into_db(self) -> Self::DBType {
        DBWeapon {
            id: self.item.id,
            material_id: self.material.map(|mat| mat.id),
            crit_range: (Bound::Included(self.crit_range.start), Bound::Included(self.crit_range.end - 1)),
            damage_type: self.damage_type,
            damage: self.damage,
            weapon_type: self.weapon_type,
        }
    }
}

impl Ord for Weapon {
    fn cmp(&self, other: &Self) -> Ordering {
        self.item.cmp(&other.item)
    }
}

impl PartialOrd for Weapon {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Weapon {
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item
    }
}

impl Eq for Weapon{}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone)]
#[derive(GetAll, GetById, Delete, DeleteById, Insert, Update)]
#[table_name = "weapons"]
#[belongs_to(DBItem, foreign_key = "id")]
pub(crate) struct DBWeapon {
    id: Uuid,
    material_id: Option<Uuid>,
    crit_range: (Bound<i32>, Bound<i32>),
    damage: Vec<String>,
    damage_type: Vec<DamageType>,
    weapon_type: WeaponClass,
}

impl Ord for DBWeapon {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for DBWeapon {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for DBWeapon {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for DBWeapon{}

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

#[derive(Serialize, Deserialize, Clone, Ord, PartialOrd, PartialEq, Eq, StandaloneDbMarker)]
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

impl TryFromDb for Armor {
    type DBType = DBArmor;

    fn try_from_db(other: Self::DBType, conn: &Connection) -> Result<Self, Error> where Self: Sized {
        let item = Item::db_get_by_id(&other.id, conn)?;
        let material = other.material_id.map(|id| Material::db_get_by_id(&id, conn)).transpose()?;
        let armor = Armor {
            item,
            material,
            max_dex_bonus: other.max_dex_bonus,
            ac: other.ac,
            spell_failure: other.spell_failure,
            check_penalty: other.check_penalty,
            armor_type: other.armor_type,
        };
        Ok(armor)
    }
}

impl IntoDb for Armor {
    type DBType = DBArmor;

    fn into_db(self) -> Self::DBType {
        DBArmor {
            id: self.item.id,
            material_id: self.material.map(|mat| mat.id),
            max_dex_bonus: self.max_dex_bonus,
            ac: self.ac,
            spell_failure: self.spell_failure,
            check_penalty: self.check_penalty,
            armor_type: self.armor_type,
        }
    }
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(GetAll, GetById, Delete, DeleteById, Insert, Update)]
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

#[derive(Serialize, Deserialize, Summarize, Clone, Ord, PartialOrd, PartialEq, Eq, StandaloneDbMarker)]
pub struct Material {
    id: Uuid,
    links: Links,
    name: String,
    description: String,
    hp_per_inch: Option<i32>,
    hardness: Option<i32>,
}

impl TryFromDb for Material {
    type DBType = DBMaterial;

    fn try_from_db(other: Self::DBType, _conn: &Connection) -> Result<Self, Error> where Self: Sized {
        let mut links = Links::new();
        links.insert("self".to_string(), format!("/materials/{}", other.id));
        let material = Material {
            id: other.id,
            links,
            name: other.name,
            description: other.description,
            hp_per_inch: other.hp_per_inch,
            hardness: other.hardness,
        };
        Ok(material)
    }
}

impl IntoDb for Material {
    type DBType = DBMaterial;

    fn into_db(self) -> Self::DBType {
        DBMaterial {
            id: self.id,
            name: self.name,
            description: self.description,
            hp_per_inch: self.hp_per_inch,
            hardness: self.hardness,
        }
    }
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(GetById, GetAll, Delete, DeleteById, Insert, Update)]
#[tavern(is_insertable, is_identifiable, is_queryable)]
#[table_name = "materials"]
struct DBMaterial {
    id: Uuid,
    name: String,
    description: String,
    hp_per_inch: Option<i32>,
    hardness: Option<i32>,
}
