use serde::{Serialize,Deserialize}; 
use uuid::Uuid;

use crate::MagicSchool;
use crate::SpellRange;
use crate::SaveThrow;
use crate::ComponentType;

use crate::item;
use crate::effects;

#[derive(Serialize,Deserialize)]
pub struct SpellSummary {
    id: Uuid,
    name: String,
    level: u32,
    school: MagicSchool
}

#[derive(Serialize,Deserialize)]
pub struct Spell {
    id: Uuid,
    components: Vec<SpellComponent>,
    name: String,
    level: u32,
    school: MagicSchool,
    effects: Vec<effects::Effect>,

    casting_time: u32,
    range: SpellRange,
    area: String,
    duration_per_level: u32,
    saving_throw: SaveThrow,
    spell_resistance: bool,
    description: String,
}

#[derive(Serialize,Deserialize)]
pub struct SpellComponent {
    component_type: ComponentType,
    item: Option<item::Item>,
    item_amount: Option<u32>,
}
