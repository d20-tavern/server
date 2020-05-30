use crate::Links;
use serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::collections::{BTreeSet, BTreeMap};
use uuid::Uuid;

use crate::SaveThrow;

#[cfg( feature = "tavern")]
use crate::effects::DBEffect;
use crate::effects::Effect;
#[cfg( feature = "tavern")]
use crate::item::DBItem;
use crate::item::Item;
use crate::summary::{Summarize, Summary};
#[cfg( feature = "tavern")]
use diesel_derive_enum::DbEnum;
#[cfg( feature = "tavern")]
use tavern_db::{GetAll, GetById, Insert, Update, Delete, Error};
#[cfg( feature = "tavern")]
use diesel::PgConnection;
#[cfg( feature = "tavern")]
use tavern_db::schema::{spells, spelleffects, spellcomponents};
#[cfg( feature = "tavern")]
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Summarize)]
pub struct Spell {
    links: Option<Links>,
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

#[cfg(feature = "tavern")]
impl TryFrom<(&DBSpell, &PgConnection)> for Spell {
    type Error = tavern_db::Error;

    fn try_from((db_spell, conn): (DBSpell, &PgConnection)) -> Result<Self, Self::Error> {
        let spell_effects = DBSpellEffect::belonging_to(&db_spell)
            .load::<DBSpellEffect>(conn)
            .map_err(Error::RunQuery)?;
        let effects = spell_effects.into_iter()
            .map(|spell_effect| Summary::<Effect>::db_get_by_id(spell_effect.effect_id, conn))
            .collect()
            .map_err(Error::RunQuery)?;

        let spell_effects = DBSpellComponent::belonging_to(&db_spell)
            .load::<DBSpellComponent>(conn)
            .map_err(Error::RunQuery)?;
        let effects = spell_effects.into_iter()
            .map(|spell_effect| SpellComponent::db_get_by_id(spell_effect.effect_id, conn))
            .collect()
            .map_err(Error::RunQuery)?;
        
        let spell = Spell {
            links: None,
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

#[cfg_attr(feature = "tavern", derive(GetAll, GetById, Delete, Insert, Update))]

#[cfg(feature = "tavern")]
impl GetAll for Spell {
    fn db_get_all(conn: &PgConnection) -> Result<Vec<Self>, Error> {
        let db_spells = DBSpell::db_get_all(conn)?;
        db.spells.into_iter()
            .map(|spell| Spell::try_from((spell, conn)))
            .collect()
    }
}

#[cfg(feature = "tavern")]
impl GetById for Spell {
    fn db_get_by_id(id: &Uuid, conn: &PgConnection) -> Result<Self, Error> {
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

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", derive(GetAll, GetById, Delete, Insert, Update))]
#[cfg_attr(feature = "tavern", table_name = "Spells")]
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

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", derive(Delete, Insert))]
#[cfg_attr(feature = "tavern", tavern(id_field = "spell_id"))]
#[cfg_attr(feature = "tavern", table_name = "SpellEffects")]
#[cfg_attr(feature = "tavern", belongs_to(DBSpell, foreign_key = "spell_id"))]
#[cfg_attr(feature = "tavern", belongs_to(DBEffect, foreign_key = "effect_id"))]
pub struct DBSpellEffect {
    spell_id: Uuid,
    effect_id: Uuid,
}

#[derive(Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq)]
pub struct SpellComponent {
    item: Option<(Summary<Item>, i16)>,
    description: String,
    amount: i16,
    #[serde(rename = "type")]
    component_type: ComponentType,
}

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", derive(GetAll, GetById, Delete, Insert, Update))]
#[cfg_attr(feature = "tavern", table_name = "SpellComponents")]
#[cfg_attr(feature = "tavern", belongs_to(DBSpell, foreign_key = "spell_id"))]
#[cfg_attr(feature = "tavern", belongs_to(DBItem, foreign_key = "item_id"))]
pub struct DBSpellComponent {
    id: Uuid,
    spell_id: Uuid,
    item_id: Option<Uuid>,
    item_amount: Option<i16>,
    component_type: ComponentType,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "tavern", derive(DbEnum))]
pub enum CasterType {
    Spontaneous,
    Prepared,
}

#[derive(Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq)]
#[cfg_attr(feature = "tavern", derive(DbEnum))]
pub enum ComponentType {
    Somatic,
    Material,
    Verbal,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "tavern", derive(DbEnum))]
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
#[cfg_attr(feature = "tavern", derive(DbEnum))]
pub enum SpellRange {
    Personal,
    Touch,
    Close,
    Medium,
    Long,
    Unlimited,
}
