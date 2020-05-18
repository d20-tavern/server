use serde::{Serialize,Deserialize};
use uuid::Uuid;

use crate::Attribute;
use crate::CasterType;
use proficiencies;

#[derive(Serialize,Deserialize)]
pub struct Class {
    id: Uuid,
    subclasses: Subclass[],

    name: String,
    hit_die: String,
    starting_wealth: String,
    bab_per_level: f32,
    skills_per_level: i32,
    skills_attr: Attribute,
}

#[derive(Serialize,Deserialize)]
pub struct Subclass {
    id: Uuid,
    parent_class: Class,

    caster_type: CasterType,
    casting_attr: Attribute,
}

#[derive(Serialize,Deserialize)]
pub struct Feature {
    id: Uuid,
    description: String,
}

pub mod proficiencies;
