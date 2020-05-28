use crate::Links;
use serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::collections::{BTreeSet};
use uuid::Uuid;

use crate::SaveThrow;

use crate::effects::Effect;
use crate::item::Item;
use crate::summary::{Summarize, Summary};
#[cfg( feature = "tavern")]
use tavern_db::{TryFromRow, TryFromUuid};

#[derive(Serialize, Deserialize, Summarize)]
#[cfg_attr(feature = "tavern", derive(TryFromRow, TryFromUuid))]
pub struct Spell {
    #[cfg_attr(feature = "tavern", tavern(
        skip, default = "Links::new()"
    ))]
    links: Links,
    id: Uuid,
    #[cfg_attr(feature = "tavern", tavern(
        references = "SpellComponent",
        column = "ARRAY(SELECT id FROM SpellComponents WHERE SpellComponents.spell_id = $1)",
        is_array
    ))]
    components: BTreeSet<SpellComponent>,
    name: String,
    level: i16,
    school: MagicSchool,
    #[cfg_attr(feature = "tavern", tavern(
        references = "Summary<Effect>",
        column = "ARRAY(SELECT effect_id FROM SpellEffects WHERE SpellEffects.spell_id = $1)",
        is_array
    ))]
    effects: Vec<Summary<Effect>>,
    casting_time: i64,
    range: SpellRange,
    area: String,
    duration_per_level: i64,
    #[cfg_attr(feature = "tavern", tavern(
        is_optional
    ))]
    saving_throw: Option<SaveThrow>,
    spell_resistance: bool,
    description: String,
}

#[derive(Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq)]
#[cfg_attr(feature = "tavern", derive(TryFromRow, TryFromUuid))]
pub struct MaterialComponent {
    #[cfg_attr(feature = "tavern", tavern(
        references = "Summary<Item>"
    ))]
    item: Summary<Item>,
    description: String,
    amount: i16,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "tavern", derive(sqlx::Type))]
pub enum CasterType {
    Spontaneous,
    Prepared,
}

// TODO: Special case?
#[derive(Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq)]
pub enum SpellComponent {
    Somatic,
    Material(MaterialComponent),
    Verbal,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "tavern", derive(sqlx::Type))]
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
#[cfg_attr(feature = "tavern", derive(sqlx::Type))]
pub enum SpellRange {
    Personal,
    Touch,
    Close,
    Medium,
    Long,
    Unlimited,
}
