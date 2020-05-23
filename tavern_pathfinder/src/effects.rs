use crate::summary::Summarize;
use crate::Attributes;
use crate::CharacterStats;
use crate::CombatStats;
use crate::Links;
use crate::Skills;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Summarize)]
pub struct Effect {
    links: Links,
    id: Uuid,
    name: String,
    #[description]
    short_description: String,
    long_description: Option<String>,
    attr_effects: Attributes,
    skill_effects: Skills,
    char_effects: CharacterStats,
    combat_effects: CombatStats,
    misc_effects: String,
}
