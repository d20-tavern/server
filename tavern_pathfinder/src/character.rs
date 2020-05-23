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

    max_hp: i16,
    damage: i16,
    nonlethal: i16,

    copper: i16,
    silver: i16,
    gold: i16,
    platinum: i16,

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
    move_speed: i16,
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
