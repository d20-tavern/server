use serde::{Serialize,Deserialize}; 
use uuid::Uuid;
use std::collections::HashMap;
use crate::Link;

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
pub struct Spell<'a> {
	links: HashMap<&'b str, Link>,

	id: Uuid,
	components: Vec<SpellComponent>,
	name: String,
	level: u32,
	school: MagicSchool,
	effects: Vec<effects::EffectSummary>,

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
	item: Option<item::ItemSummary>,
	item_amount: Option<u32>,
}
