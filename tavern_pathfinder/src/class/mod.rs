use serde::{Serialize,Deserialize};
use uuid::Uuid;

#[derive(Serialize,Deserialize)]
pub struct Class {
    class_id: Uuid,

    name: String,
    hit_die: String,
    starting_wealth: String,
    bab_per_level: f32,
    skills_per_level: i32,
    skills_attr: crate::Attribute,
}

#[derive(Serialize,Deserialize)]
pub struct Subclass {
    subclass_id: Uuid,
    class: Class,

    caster_type: crate::CasterType,
    casting_attr: crate::Attribute,
}

#[derive(Serialize,Deserialize)]
pub struct Feature {
    feature_id: Uuid,
    description: String,
}

pub mod proficiencies;
