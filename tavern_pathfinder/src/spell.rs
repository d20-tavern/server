use crate::Links;
use serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::collections::{BTreeSet};
use uuid::Uuid;

use crate::SaveThrow;

use crate::effects::Effect;
use crate::item::Item;
use crate::summary::{Summarize, Summary};

#[derive(Serialize, Deserialize, Summarize)]
pub struct Spell {
    links: Links,
    id: Uuid,
    components: BTreeSet<SpellComponent>,
    name: String,
    level: i16,
    school: MagicSchool,
    effects: Vec<Summary<Effect>>,
    casting_time: i64,
    range: SpellRange,
    area: String,
    duration_per_level: i64,
    saving_throw: Option<SaveThrow>,
    spell_resistance: bool,
    description: String,
}

#[derive(Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq)]
pub struct MaterialComponent {
    item: Summary<Item>,
    description: String,
    amount: i16,
}

#[derive(Serialize, Deserialize)]
pub enum CasterType {
    Spontaneous,
    Prepared,
}

#[derive(Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq)]
pub enum SpellComponent {
    Somatic,
    Material(MaterialComponent),
    Verbal,
}

#[derive(Serialize, Deserialize)]
pub enum MagicSchool {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
}

#[derive(Serialize, Deserialize)]
pub enum SpellRange {
    Personal,
    Touch,
    Close,
    Medium,
    Long,
    Unlimited,
}
