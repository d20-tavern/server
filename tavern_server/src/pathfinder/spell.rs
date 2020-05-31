use serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::collections::BTreeSet;
use std::convert::TryFrom;
use uuid::Uuid;

use super::effects::{DBEffect, Effect};
use super::item::{DBItem, Item};
use super::summary::{Summarize, Summary};
use super::{Links, SaveThrow};
use crate::schema::{spellcomponents, spelleffects, spells};

use crate::db;
use crate::db::{Connection, Delete, Error, GetAll, GetById, Insert, Update};
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;

#[derive(Serialize, Deserialize, Summarize)]
pub struct Spell {
    links: Links,
    id: Uuid,
    components: BTreeSet<SpellComponent>,
    name: String,
    level: i16,
    school: MagicSchool,
    effects: BTreeSet<Summary<Effect>>,
    casting_time: i64,
    range: SpellRange,
    area: String,
    duration_per_level: i64,
    saving_throw: Option<SaveThrow>,
    spell_resistance: bool,
    description: String,
}

impl TryFrom<(DBSpell, &Connection)> for Spell {
    type Error = db::Error;

    fn try_from((db_spell, conn): (DBSpell, &Connection)) -> Result<Self, Self::Error> {
        let spell_effects = DBSpellEffect::belonging_to(&db_spell)
            .load::<DBSpellEffect>(conn)
            .map_err(Error::RunQuery)?;
        let effects = spell_effects
            .into_iter()
            .map(|spell_effect| Summary::<Effect>::db_get_by_id(spell_effect.effect_id, conn))
            .collect()
            .map_err(Error::RunQuery)?;

        let spell_components = DBSpellComponent::belonging_to(&db_spell)
            .load::<DBSpellComponent>(conn)
            .map_err(Error::RunQuery)?;
        let components = spell_components
            .into_iter()
            .map(|spell_component| SpellComponent::db_get_by_id(spell_component.item_id, conn))
            .collect()
            .map_err(Error::RunQuery)?;

        let spell = Spell {
            links: Default::default(),
            id: db_spell.id,
            components,
            name: db_spell.name,
            level: db_spell.level,
            school: db_spell.school,
            effects,
            casting_time: db_spell.casting_time,
            range: db_spell.range,
            area: db_spell.area,
            duration_per_level: db_spell.duration_per_level,
            saving_throw: db_spell.saving_throw,
            spell_resistance: db_spell.spell_resistance,
            description: db_spell.description,
        };

        Ok(spell)
    }
}

impl GetAll for Spell {
    fn db_get_all(conn: &Connection) -> Result<Vec<Self>, Error> {
        let db_spells = DBSpell::db_get_all(conn)?;
        db_spells
            .into_iter()
            .map(|db_spell| Spell::try_from((db_spell, conn)))
            .collect()
    }
}

impl GetById for Spell {
    fn db_get_by_id(id: &Uuid, conn: &Connection) -> Result<Self, Error> {
        let db_spell = DBSpell::db_get_by_id(id, conn)?;
        Spell::try_from((db_spell, conn))
    }
}

/*#[cfg(feature = "tavern")]
impl Insert for Spell {
    fn db_insert(&self, conn: &PgConnection) -> Result<(), Error> {
        let (db_spell, db_effects, db_components) = self
    }
}*/

#[derive(
    AsChangeset,
    Associations,
    Identifiable,
    Insertable,
    Queryable,
    GetAll,
    GetById,
    Delete,
    Insert,
    Update,
)]
#[table_name = "spells"]
pub struct DBSpell {
    id: Uuid,
    name: String,
    level: i16,
    school: MagicSchool,
    casting_time: i64,
    range: SpellRange,
    area: String,
    duration_per_level: i64,
    saving_throw: Option<SaveThrow>,
    spell_resistance: bool,
    description: String,
}

#[derive(Associations, Identifiable, Insertable, Queryable, GetById, Delete, Insert)]
#[tavern(id_field = "spell_id")]
#[table_name = "spelleffects"]
#[primary_key(spell_id, effect_id)]
#[belongs_to(DBSpell, foreign_key = "spell_id")]
pub struct DBSpellEffect {
    spell_id: Uuid,
    effect_id: Uuid,
}

#[derive(Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq)]
pub struct SpellComponent {
    id: Uuid,
    item: Option<(Summary<Item>, i16)>,
    description: String,
    amount: i16,
    #[serde(rename = "type")]
    component_type: ComponentType,
}

#[derive(
    AsChangeset,
    Associations,
    Identifiable,
    Insertable,
    Queryable,
    GetAll,
    GetById,
    Delete,
    Insert,
    Update,
)]
#[table_name = "spellcomponents"]
#[belongs_to(DBSpell, foreign_key = "spell_id")]
pub struct DBSpellComponent {
    id: Uuid,
    spell_id: Uuid,
    item_id: Option<Uuid>,
    item_amount: Option<i16>,
    component_type: ComponentType,
}

#[derive(Serialize, Deserialize, DbEnum, Debug)]
pub enum CasterType {
    Spontaneous,
    Prepared,
}

#[derive(Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq, DbEnum, Debug)]
pub enum ComponentType {
    Somatic,
    Material,
    Verbal,
}

#[derive(Serialize, Deserialize, DbEnum, Debug)]
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

#[derive(Serialize, Deserialize, DbEnum, Debug)]
pub enum SpellRange {
    Personal,
    Touch,
    Close,
    Medium,
    Long,
    Unlimited,
}
