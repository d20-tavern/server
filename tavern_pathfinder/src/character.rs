use crate::Links;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::cell::RefCell;
use std::sync::Arc;

//Additional modules
use crate::class::Class;
use crate::feat::Feat;
use crate::item::Bag;
use crate::religion::Deity;
use crate::spell::Spell;
use crate::summary::{Summarize, Summary};

//Enums
use crate::Alignment;
use crate::Gender;
use crate::Size;

// TODO: Impl Summarize for Character

#[derive(Serialize, Deserialize, Summarize)]
pub struct Character {
    links: Links,

    id: Uuid,
    race: Race,
    deity: Summary<Deity>,

    classes: Vec<Summary<Class>>,
    feats: Vec<Summary<Feat>>,
    spells: Vec<Summary<Spell>>,
    bags: Vec<Bag>,

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

    #[serde(skip)]
    description: String,
}

impl Character {
    fn update_desc(&mut self) {
        let level = self.classes.iter().count();
        self.description = format!("Level {} {} {} {}", level, &self.gender, &self.alignment, &self.race.name);
    }
}

// TODO: I think this can be implemented better

#[derive(Serialize, Deserialize, Summarize)]
pub struct Race {
    id: Uuid,
    links: Links,
    main_type: RaceType,
    subtype: RaceSubtype,
    name: String,
    description: String,
    move_speed: u32,
    size: Size,
    languages: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RaceType {
    id: Uuid,
    name: String,
    hit_die: String,
    bab_per_hit_die: f32,
}

#[derive(Serialize, Deserialize)]
pub struct RaceSubtype {
    id: Uuid,
    parent_type: RaceType,
    name: String,
}
