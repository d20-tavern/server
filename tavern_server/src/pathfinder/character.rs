use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::class::{Feature, Subclass};
use super::feat::Feat;
use super::item::{Bag, DBBag, Item};
use super::religion::Deity;
use super::spell::Spell;
use super::summary::{Summarize, Summary};
use super::Links;
use super::{Alignment, EquipmentSlot, Gender, Size};

use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::connection::Connection as DieselConnection;

//use tavern_derive::Summarize;
use crate::schema::{
    characterequipment, characterfeats, characterfeatures, characters, characterspells,
    charactersubclasses, races, racesubtypes, racetypes,
};
use std::cmp::Ordering;
use crate::db::{TryFromDb, IntoDb, Connection, Error, GetAll, GetById, Delete, DeleteById, Insert, Update, StandaloneDbMarker};
use std::collections::{BTreeSet, BTreeMap};

#[derive(Serialize, Deserialize, Summarize, Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct Character {
    id: Uuid,
    race: Race,
    deity: Option<Summary<Deity>>,
    subclasses: Vec<Summary<Subclass>>,
    feats: Vec<Summary<Feat>>,
    spells: Vec<Summary<Spell>>,
    bags: BTreeSet<Summary<Bag>>,
    equipment: BTreeMap<EquipmentSlot, Summary<Item>>,
    features: Vec<Summary<Feature>>,

    name: String,
    age: i16,
    gender: Gender,
    alignment: Alignment,
    backstory: String,
    height: i16,
    weight: i16,
    size: Size,

    strength: i16,
    dexterity: i16,
    constitution: i16,
    intelligence: i16,
    wisdom: i16,
    charisma: i16,

    max_hp: i16,
    damage: i16,
    nonlethal: i16,

    copper: i16,
    silver: i16,
    gold: i16,
    platinum: i16,

    links: Links,
    #[serde(skip)]
    description: String,
}

impl TryFromDb for Character {
    type DBType = DBCharacter;

    fn try_from_db(other: Self::DBType, conn: &Connection) -> Result<Self, Error> where Self: Sized {
        let race = Race::db_get_by_id(&other.race_id, conn)?;
        let deity = other.deity_id.map(|id| Summary::<Deity>::db_get_by_id(&id, conn)).transpose()?;
        let subclasses = other.get_subclasses(conn)?;
        let feats = other.get_feats(conn)?;
        let spells = other.get_spells(conn)?;
        let bags = other.get_bags(conn)?;
        let equipment = other.get_equipment(conn)?;
        let features = other.get_features(conn)?;
        let links = Links::new();
        let character = Character {
            id: other.id,
            race,
            deity,
            subclasses,
            feats,
            spells,
            bags,
            equipment,
            features,
            name: other.name,
            age: other.age,
            gender: other.gender,
            alignment: other.alignment,
            backstory: other.backstory,
            height: other.height,
            weight: other.weight,
            size: other.size,
            strength: other.strength,
            dexterity: other.dexterity,
            constitution: other.constitution,
            intelligence: other.intelligence,
            wisdom: other.wisdom,
            charisma: other.charisma,
            max_hp: other.max_hp,
            damage: other.damage,
            nonlethal: other.nonlethal,
            copper: other.copper,
            silver: other.silver,
            gold: other.gold,
            platinum: other.platinum,
            links,
            description: Default::default(),
        };

        Ok(character)
    }
}

impl IntoDb for Character {
    type DBType = ();

    fn into_db(self) -> Self::DBType {
        unimplemented!()
    }
}

impl Character {
    fn update_desc(&mut self) {
        let level = self.subclasses.iter().count();
        self.description = format!(
            "Level {} {} {} {}",
            level, &self.gender, &self.alignment, &self.race.name
        );
    }
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(GetAll, GetById, Delete, DeleteById, Insert, Update)]
#[tavern(is_identifiable, is_insertable, is_queryable)]
#[table_name = "characters"]
pub struct DBCharacter {
    id: Uuid,
    user_id: Uuid,
    race_id: Uuid,
    deity_id: Option<Uuid>,

    name: String,
    age: i16,
    gender: Gender,
    alignment: Alignment,
    backstory: String,
    height: i16,
    weight: i16,
    size: Size,

    strength: i16,
    dexterity: i16,
    constitution: i16,
    intelligence: i16,
    wisdom: i16,
    charisma: i16,

    max_hp: i16,
    damage: i16,
    nonlethal: i16,

    copper: i16,
    silver: i16,
    gold: i16,
    platinum: i16,
}

impl DBCharacter {
    fn get_subclasses(&self, conn: &Connection) -> Result<Vec<Summary<Subclass>>, Error> {
        DBCharacterSubclass::belonging_to(self)
            .load::<DBCharacterSubclass>(conn)
            .map_err(Error::RunQuery)?
            .into_iter()
            .map(|sc| Summary::<Subclass>::db_get_by_id(&sc.subclass_id, conn))
            .collect()
    }
    fn get_feats(&self, conn: &Connection) -> Result<Vec<Summary<Feat>>, Error> {
        DBCharacterFeat::belonging_to(self)
            .load::<DBCharacterFeat>(conn)
            .map_err(Error::RunQuery)?
            .into_iter()
            .map(|f| Summary::<Feat>::db_get_by_id(&f.feat_id, conn))
            .collect()
    }
    fn get_spells(&self, conn: &Connection) -> Result<Vec<Summary<Spell>>, Error> {
        DBCharacterSpell::belonging_to(self)
            .load::<DBCharacterSpell>(conn)
            .map_err(Error::RunQuery)?
            .into_iter()
            .map(|s| Summary::<Spell>::db_get_by_id(&s.spell_id, conn))
            .collect()
    }
    fn get_bags(&self, conn: &Connection) -> Result<BTreeSet<Summary<Bag>>, Error> {
        DBBag::belonging_to(self)
            .load::<DBBag>(conn)
            .map_err(Error::RunQuery)?
            .into_iter()
            .map(|b| Summary::<Bag>::try_from_db(b, conn))
            .collect()
    }
    fn get_equipment(&self, conn: &Connection) -> Result<BTreeMap<EquipmentSlot, Summary<Item>>, Error> {
        DBCharacterEquipment::belonging_to(self)
            .load::<DBCharacterEquipment>(conn)
            .map_err(Error::RunQuery)?
            .into_iter()
            .map(|e| {
                // The database should ensure that all items marked as equipment
                // have an equipment slot. So, an unwrap should be safe here. If this
                // is ever not the case, ensure an INSERT/UPDATE trigger is set on the
                // database to catch this.
                Item::db_get_by_id(&e.item_id, conn)
                    .map(|i| (i.equip_slot.unwrap(), Summary::<Item>::from(&i)))
            })
            .collect()
    }
    fn get_features(&self, conn: &Connection) -> Result<Vec<Summary<Feature>>, Error> {
        DBCharacterFeature::belonging_to(self)
            .load::<DBCharacterFeature>(conn)
            .map_err(Error::RunQuery)?
            .into_iter()
            .map(|f| Summary::<Feature>::db_get_by_id(&f.feature_id, conn))
            .collect()
    }
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(GetAll, Delete, Insert, Update)]
#[tavern(is_identifiable, is_insertable, is_queryable)]
#[table_name = "charactersubclasses"]
#[primary_key(char_id, subclass_id)]
#[belongs_to(DBCharacter, foreign_key = "char_id")]
pub struct DBCharacterSubclass {
    char_id: Uuid,
    subclass_id: Uuid,
    levels_taken: i16,
    hp_taken: i16,
    skills_taken: i16,
}

#[derive(Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(GetAll, Delete, Insert)]
#[tavern(is_identifiable, is_insertable, is_queryable)]
#[table_name = "characterfeats"]
#[primary_key(char_id, feat_id)]
#[belongs_to(DBCharacter, foreign_key = "char_id")]
pub struct DBCharacterFeat {
    char_id: Uuid,
    feat_id: Uuid,
}

#[derive(Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(GetAll, Delete, Insert)]
#[tavern(is_identifiable, is_insertable, is_queryable)]
#[table_name = "characterfeatures"]
#[primary_key(char_id, feature_id)]
#[belongs_to(DBCharacter, foreign_key = "char_id")]
pub struct DBCharacterFeature {
    char_id: Uuid,
    feature_id: Uuid,
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[table_name = "characterspells"]
#[derive(GetAll, Delete, Insert, Update)]
#[tavern(is_identifiable, is_insertable, is_queryable)]
#[primary_key(char_id, spell_id)]
#[belongs_to(DBCharacter, foreign_key = "char_id")]
pub struct DBCharacterSpell {
    char_id: Uuid,
    spell_id: Uuid,
    casts_remaining: i16,
}

#[derive(Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(GetAll, Delete, Insert)]
#[tavern(is_identifiable, is_insertable, is_queryable)]
#[table_name = "characterequipment"]
#[primary_key(char_id, item_id)]
#[belongs_to(DBCharacter, foreign_key = "char_id")]
pub struct DBCharacterEquipment {
    char_id: Uuid,
    item_id: Uuid,
}

// TODO: I think this can be implemented better

#[derive(Serialize, Deserialize, Summarize, Clone, Ord, PartialOrd, PartialEq, Eq, StandaloneDbMarker)]
pub struct Race {
    id: Uuid,
    links: Links,
    main_type: RaceType,
    sub_type: Option<RaceSubtype>,
    name: String,
    description: String,
    move_speed: i16,
    size: Size,
    languages: Vec<String>,
}

impl TryFromDb for Race {
    type DBType = DBRace;

    fn try_from_db(other: Self::DBType, conn: &Connection) -> Result<Self, Error> where Self: Sized {
        let main_type = RaceType::db_get_by_id(&other.type_id, conn)?;
        let sub_type = other.subtype_id.map(|id| RaceSubtype::db_get_by_id(&id, conn)).transpose()?;
        let links = Links::new();
        let race = Race {
            id: other.id,
            links,
            main_type,
            sub_type,
            name: other.name,
            description: other.description,
            move_speed: other.move_speed,
            size: other.size,
            languages: other.languages,
        };
        Ok(race)
    }
}

impl IntoDb for Race {
    type DBType = DBRace;

    fn into_db(self) -> Self::DBType {
        DBRace {
            id: self.id,
            type_id: self.main_type.id,
            subtype_id: self.sub_type.map(|sub| sub.id),
            name: self.name,
            description: self.description,
            move_speed: self.move_speed,
            size: self.size,
            languages: self.languages,
        }
    }
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(GetAll, GetById, Delete, DeleteById, Insert, Update)]
#[tavern(is_identifiable, is_insertable, is_queryable)]
#[table_name = "races"]
pub struct DBRace {
    id: Uuid,
    description: String,
    type_id: Uuid,
    subtype_id: Option<Uuid>,
    name: String,
    move_speed: i16,
    size: Size,
    languages: Vec<String>,
}

#[derive(Serialize, Deserialize, AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone)]
#[derive(GetAll, GetById, Delete, DeleteById, Insert, Update)]
#[tavern(is_identifiable, is_insertable, is_queryable)]
#[table_name = "racetypes"]
pub struct RaceType {
    id: Uuid,
    name: String,
    hit_die: String,
    bab_per_hit_die: f32,
}

impl Ord for RaceType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for RaceType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for RaceType {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for RaceType{}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(GetAll, GetById, Delete, DeleteById, Insert, Update)]
#[tavern(is_identifiable, is_insertable, is_queryable)]
#[table_name = "racesubtypes"]
#[derive(Serialize, Deserialize)]
pub struct RaceSubtype {
    id: Uuid,
    name: String,
    description: String,
}
