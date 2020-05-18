use serde::{Serialize,Deserialize};
use uuid::Uuid;
use std::collections::HashMap;
use crate::Link;

//Additional modules
use crate::religion;
use crate::class;
use crate::feat;
use crate::spell;
use crate::item;
use crate::effects;

//Enums
use crate::Gender;
use crate::Alignment;
use crate::Size;

#[derive(Serialize,Deserialize)]
pub struct CharacterSummary {
	char_id: Uuid,
	name: String,
	race: Race,
	level: u8,
}

#[derive(Serialize,Deserialize)]
pub struct Character<'a> {
	links: HashMap<&'b str, Link>,

	id: Uuid,
	race: Race,
	deity: religion::DeitySummary,

	classes: Vec<class::ClassSummary>,
	feats: Vec<feat::FeatSummary>,
	spells: Vec<spell::SpellSummary>,
	bags: Vec<item::Bag<'b>>,
	active_effects: Vec<effects::EffectSummary>,

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
}

#[derive(Serialize,Deserialize)]
pub struct Race {
	id: Uuid,
	main_type: RaceType,
	subtype: RaceSubtype,

	name: String,
	move_speed: u32,
	size: Size,
	languages: Vec<String>,
}

#[derive(Serialize,Deserialize)]
pub struct RaceType {
	id: Uuid,

	name: String,
	hit_die: String,
	bab_per_hit_die: f32,
}

#[derive(Serialize,Deserialize)]
pub struct RaceSubtype {
	id: Uuid,
	parent_type: RaceType,

	name: String,
}
