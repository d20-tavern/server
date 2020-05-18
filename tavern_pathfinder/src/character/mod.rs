use serde::{Serialize,Deserialize};
use uuid::Uuid;

//Additional modules
use crate::religion;
use crate::class;
use crate::feat;
use crate::spell;
use crate::item;
use crate::effects;

//Enums
use crate::Gender;
use crate::Alignment;
use crate::Size;

#[derive(Serialize,Deserialize)]
pub struct CharacterSummary {
    char_id: Uuid,
    name: String,
    race: Race,
    level: u8,
}

#[derive(Serialize,Deserialize)]
pub struct Character {
    id: Uuid,
    race: Race,
    deity: religion::Deity,

    classes: Vec<class::Classes>,
    feats: Vec<feat::Feat>,
    spells: Vec<spell::Spell>,
    bags: Vec<item::Bag>,
    active_effects: Vec<effects::Effect>,

    name: String,
    age: u32,
    gender: Gender,
    alignment: Alignment,
    backstory: String,
    height: u32,
    weight: u32,
    size: Size,

    strength: u32,
    dexterity: u32,
    constitution: u32,
    intelligence: u32,
    wisdom: u32,

    max_hp: u32,
    damage: u32,
    nonlethal: u32,

    copper: u32,
    silver: u32,
    gold: u32,
    platinum: u32,
}

#[derive(Serialize,Deserialize)]
pub struct Race {
    id: Uuid,
    type: RaceType,
    subtype: RaceSubtype,

    name: String,
    move_speed: u32,
    size: Size,
    languages: Vec<String>,
}

#[derive(Serialize,Deserialize)]
pub struct RaceType {
    id: Uuid,
    parent_race: Race,

    name: String,
    hit_die: String,
    bab_per_hit_die: f32,
}

#[derive(Serialize,Deserialize)]
pub struct RaceSubtype {
    id: Uuid,
    parent_type: RaceType,

    name: String,
}
