use serde::{Serialize,Deserialize};
use uuid::Uuid;
use std::collections::HashMap;
use crate::Link;

use crate::Attribute;
use crate::CasterType;

#[derive(Serialize,Deserialize)]
pub struct ClassSummary {
    id: Uuid,
    name: String,

    subclasses: Vec<String>,
}

#[derive(Serialize,Deserialize)]
pub struct Class<'a> {
    links: HashMap<&'b str, Link>,

    id: Uuid,

    subclasses: Vec<Subclass<'b>>,
    weapon_proficiencies: proficiencies::WeaponsNotProficient,
    armor_proficiencies: proficiencies::ArmorNotProficient,

    name: String,
    hit_die: String,
    starting_wealth: String,
    bab_per_level: f32,
    skills_per_level: i32,
    skills_attr: Attribute,
}

#[derive(Serialize,Deserialize)]
pub struct Subclass<'b> {
    id: Uuid,
    parent_class: Class<'b>,

    caster_type: CasterType,
    casting_attr: Attribute,
    features: Vec<Feature>
}

#[derive(Serialize,Deserialize)]
pub struct Feature {
    id: Uuid,
    description: String,
}

pub mod proficiencies;
