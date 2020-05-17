use serde::{Serialize,Deserialize};
use uuid::Uuid;

#[derive(Serialize,Deserialize)]
pub struct CharacterSummary {
    char_id: Uuid,
    name: String,
    race: Race,
    level: u8,
}

#[derive(Serialize,Deserialize)]
pub struct Character {
    char_id: Uuid,
    race: Race,
    deity: crate::religion::Deity,

    name: String,
    age: u32,
    gender: crate::Gender,
    alignment: crate::Alignment,
    backstory: String,
    height: u32,
    weight: u32,
    size: crate::Size,

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
    race_id: Uuid,
    type_id: RaceType,
    subtype_id: RaceSubtype,

    name: String,
    move_speed: u32,
    size: crate::Size,
    languages: Vec<String>,
}

#[derive(Serialize,Deserialize)]
pub struct RaceType {
    type_id: Uuid,

    name: String,
    hit_die: String,
    bab_per_hit_die: f32,
}

#[derive(Serialize,Deserialize)]
pub struct RaceSubtype {
    subtype_id: Uuid,
    name: String,
}
