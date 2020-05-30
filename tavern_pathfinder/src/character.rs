use crate::Links;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[cfg( feature = "tavern")]
use tavern_db::{TryFromRow, TryFromUuid};

//Additional modules
use crate::class::Subclass;
use crate::feat::Feat;
use crate::item::{Bag, Item};
use crate::religion::Deity;
use crate::spell::Spell;
use crate::summary::{Summarize, Summary};

//Enums
use crate::Alignment;
use crate::Gender;
use crate::Size;

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

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "Characters")]
#[cfg_attr(feature = "tavern", belongs_to(DBRace, foreign_key = "race_id"))]
#[cfg_attr(feature = "tavern", belongs_to(DBDeity, foreign_key = "subtype_id"))]
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

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "CharacterSubclasses")]
#[cfg_attr(feature = "tavern", belongs_to(DBCharacter, foreign_key = "char_id"))]
#[cfg_attr(feature = "tavern", belongs_to(DBSubclass, foreign_key = "subclass_id"))]
pub struct DBCharacterSubclass {
    char_id: Uuid,
    subclass_id: Uuid,
    levels_taken: i16,
    hp_taken: i16,
    skills_taken: i16,
}

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "CharacterFeats")]
#[cfg_attr(feature = "tavern", belongs_to(DBCharacter, foreign_key = "char_id"))]
#[cfg_attr(feature = "tavern", belongs_to(DBFeat, foreign_key = "feat_id"))]
pub struct DBCharacterFeat {
    char_id: Uuid,
    feat_id: Uuid,
}

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "CharacterFeatures")]
#[cfg_attr(feature = "tavern", belongs_to(DBCharacter, foreign_key = "char_id"))]
#[cfg_attr(feature = "tavern", belongs_to(Feature, foreign_key = "feature_id"))]
pub struct DBCharacterFeature {
    char_id: Uuid,
    feature_id: Uuid,
}

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "CharacterFeatures")]
#[cfg_attr(feature = "tavern", belongs_to(DBCharacter, foreign_key = "char_id"))]
#[cfg_attr(feature = "tavern", belongs_to(DBSpell, foreign_key = "spell_id"))]
pub struct DBCharacterSpell {
    char_id: Uuid,
    spell_id: Uuid,
    casts_remaining: i16,
}

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "CharacterEquipment")]
#[cfg_attr(feature = "tavern", belongs_to(DBCharacter, foreign_key = "char_id"))]
#[cfg_attr(feature = "tavern", belongs_to(DBItem, foreign_key = "item_id"))]
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

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "Races")]
#[cfg_attr(feature = "tavern", belongs_to(DBRaceType, foreign_key = "type_id"))]
#[cfg_attr(feature = "tavern", belongs_to(DBRaceSubtype, foreign_key = "subtype_id"))]
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
#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "RaceTypes")]
pub struct RaceType {
    id: Uuid,
    name: String,
    hit_die: String,
    bab_per_hit_die: f32,
}

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "RaceSubtypes")]
#[derive(Serialize, Deserialize)]
pub struct RaceSubtype {
    id: Uuid,
    name: String,
    description: String,
}
