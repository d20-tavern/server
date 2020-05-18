use serde::{Serialize,Deserialize}; 
use uuid::Uuid;

#[derive(Serialize,Deserialize)]
pub struct Spell {
    spell_id: Uuid,

    name: String,
    level: u32,
    school: crate::MagicSchool,

    casting_time: u32,
    range: crate::SpellRange,
    area: String,
    duration_per_level: u32,
    saving_throw: crate::SaveThrow,
    spell_resistance: bool,
    description: String,
}

#[derive(Serialize,Deserialize)]
pub struct SpellComponent {
    spell: Spell,
}
