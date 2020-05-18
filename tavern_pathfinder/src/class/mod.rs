use serde::{Serialize,Deserialize};
use uuid::Uuid;

use crate::Attribute;
use crate::CasterType;

#[derive(Serialize,Deserialize)]
pub struct ClassSummary {
    id: Uuid,
    name: String,

    subclasses: Vec<String>,
}

#[derive(Serialize,Deserialize)]
pub struct Class {
    links: HashMap<&str, Link>,

    id: Uuid,

    subclasses: Vec<Subclass>,
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
pub struct Subclass {
    id: Uuid,
    parent_class: Class,

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
