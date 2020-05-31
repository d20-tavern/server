use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Links;
use super::class::{DBSubclass, Subclass, Feature};
use super::feat::{DBFeat, Feat};
use super::item::{Bag, DBBag, DBItem, Item};
use super::religion::{DBDeity, Deity};
use super::spell::{DBSpell, Spell};
use super::summary::{Summarize, Summary};
use super::{Alignment, Gender, Size};

//use tavern_derive::Summarize;
use crate::schema::{characters, characterequipment, characterfeats, characterfeatures, characterspells, charactersubclasses, races, racesubtypes, racetypes};

#[derive(Serialize, Deserialize, Summarize)]
pub struct Character {
    id: Uuid,
    race: Race,
    deity: Summary<Deity>,
    subclasses: Vec<Summary<Subclass>>,
    feats: Vec<Summary<Feat>>,
    spells: Vec<Summary<Spell>>,
    bags: Vec<Summary<Bag>>,
    equipment: Vec<Summary<Item>>,
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

impl Character {
    fn update_desc(&mut self) {
        let level = self.subclasses.iter().count();
        self.description = format!("Level {} {} {} {}", level, &self.gender, &self.alignment, &self.race.name);
    }
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable)]
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

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable)]
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

#[derive(Associations, Identifiable, Insertable, Queryable)]
#[table_name = "characterfeats"]
#[primary_key(char_id, feat_id)]
#[belongs_to(DBCharacter, foreign_key = "char_id")]
pub struct DBCharacterFeat {
    char_id: Uuid,
    feat_id: Uuid,
}

#[derive(Associations, Identifiable, Insertable, Queryable)]
#[table_name = "characterfeatures"]
#[primary_key(char_id, feature_id)]
#[belongs_to(DBCharacter, foreign_key = "char_id")]
pub struct DBCharacterFeature {
    char_id: Uuid,
    feature_id: Uuid,
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[table_name = "characterspells"]
#[primary_key(char_id, spell_id)]
#[belongs_to(DBCharacter, foreign_key = "char_id")]
pub struct DBCharacterSpell {
    char_id: Uuid,
    spell_id: Uuid,
    casts_remaining: i16,
}

#[derive(Associations, Identifiable, Insertable, Queryable)]
#[table_name = "characterequipment"]
#[primary_key(char_id, item_id)]
#[belongs_to(DBCharacter, foreign_key = "char_id")]
pub struct DBCharacterEquipment {
    char_id: Uuid,
    item_id: Uuid,
}

// TODO: I think this can be implemented better

#[derive(Serialize, Deserialize, Summarize)]
pub struct Race {
    id: Uuid,
    links: Links,
    main_type: RaceType,
    sub_type: RaceSubtype,
    name: String,
    description: String,
    move_speed: i16,
    size: Size,
    languages: Vec<String>,
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[table_name = "races"]
pub struct DBRace {
    id: Uuid,
    type_id: Uuid,
    subtype_id: Uuid,
    name: String,
    description: String,
    move_speed: i16,
    size: Size,
    languages: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[table_name = "racetypes"]
pub struct RaceType {
    id: Uuid,
    name: String,
    hit_die: String,
    bab_per_hit_die: f32,
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[table_name = "racesubtypes"]
#[derive(Serialize, Deserialize)]
pub struct RaceSubtype {
    id: Uuid,
    name: String,
    description: String,
}
