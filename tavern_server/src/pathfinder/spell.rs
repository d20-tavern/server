use serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd, Ordering};
use std::collections::BTreeSet;
use std::convert::TryFrom;
use uuid::Uuid;

use super::effects::{DBEffect, Effect};
use super::item::{DBItem, Item};
use super::summary::{Summarize, Summary};
use super::{Links, SaveThrow};

use crate::schema::{spells, spellcomponents, spelleffects};
use crate::db::{self, Connection, Delete, DeleteById, Error, GetAll, GetById, Insert, Update, IntoDbWithId, TryFromDb, IntoDb, StandaloneDbMarker};
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use diesel::associations::BelongsTo;
use diesel::Connection as DieselConnection;

#[derive(Clone, Serialize, Deserialize, Summarize, Ord, PartialOrd, PartialEq, Eq,)]
pub struct Spell {
    pub links: Links,
    pub id: Uuid,
    pub components: BTreeSet<SpellComponent>,
    pub name: String,
    pub level: i16,
    pub school: MagicSchool,
    pub effects: BTreeSet<Summary<Effect>>,
    pub casting_time: i64,
    pub range: SpellRange,
    pub area: String,
    pub duration_per_level: i64,
    pub saving_throw: Option<SaveThrow>,
    pub spell_resistance: bool,
    pub description: String,
}

impl TryFromDb for Spell {
    type DBType = DBSpell;
    fn try_from_db(db_spell: DBSpell, conn: &Connection) -> Result<Self, Error> {
        let effects = db_spell.get_effects(conn)?;

        let components = db_spell.get_components(conn)?;

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

impl Insert for Spell {
    fn db_insert(&self, conn: &Connection) -> Result<(), Error> {
        conn.transaction::<_, Error, _>(|| {
            let (spell, components, effects) = Spell::to_owned(self).into_db();
            spell.db_insert(conn)?;
            for component in components {
                component.db_insert(conn)?;
            }
            for effect in effects {
                effect.db_insert(conn)?;
            }
            Ok(())
        })
    }
}

impl Update for Spell {
    fn db_update(&self, conn: &Connection) -> Result<(), Error> {
        conn.transaction::<_, Error, _>(|| {
            let (spell, components, effects) = Spell::to_owned(self).into_db();
            spell.db_update(conn)?;

            // Get difference in current and updated effects
            let old_effects = spell.get_effects(conn)?;
            let delete_effects = old_effects.difference(&self.effects)
                .map(|effect| DBSpellEffect {
                    spell_id: self.id.to_owned(),
                    effect_id: effect.id().to_owned(),
                });
            let add_effects = self.effects.difference(&old_effects)
                .map(|effect| DBSpellEffect {
                    spell_id: self.id.to_owned(),
                    effect_id: effect.id().to_owned(),
                });

            // Get difference in current and updated components
            let old_components = spell.get_components(conn)?;
            let delete_components = old_components.difference(&self.components)
                .map(|comp| comp.into_db(self.id.to_owned()));
            let add_components = self.components.difference(&old_components)
                .map(|comp| comp.into_db(self.id.to_owned()));

            for effect in delete_effects {
                effect.db_delete(conn)?;
            }
            for effect in add_effects {
                effect.db_insert(conn)?;
            }

            for component in delete_components {
                component.db_delete(conn)?;
            }
            for component in add_components {
                component.db_insert(conn)?;
            }

            Ok(())
        })
    }
}

impl DeleteById for Spell {
    fn db_delete_by_id(del_id: &Uuid, conn: &Connection) -> Result<(), db::Error> {
        conn.transaction::<_, db::Error, _>(|| {
            use crate::schema::spellcomponents::dsl::*;
            DBSpell::db_delete_by_id(del_id, conn)?;
            DBSpellEffect::db_delete_by_id(del_id, conn)?;
            diesel::delete(spellcomponents.filter(spell_id.eq(del_id)))
                .execute(conn)
                .map_err(db::Error::RunQuery)
                .map(|_| ())
        })
    }
}

//impl Delete for Spell {
//    fn db_delete(&self, conn: &Connection) -> Result<(), Error> {
//        self.into().db_delete(conn)
//    }
//}

#[derive(
    AsChangeset,
    Associations,
    Identifiable,
    Insertable,
    Queryable,
    GetAll,
    GetById,
    Delete,
    DeleteById,
    Insert,
    Update,
Ord,
PartialOrd,
PartialEq,
Eq,
)]
#[tavern(is_insertable, is_identifiable, is_queryable)]
#[table_name = "spells"]
pub struct DBSpell {
    pub id: Uuid,
    pub name: String,
    pub level: i16,
    pub school: MagicSchool,
    pub casting_time: i64,
    pub range: SpellRange,
    pub area: String,
    pub duration_per_level: i64,
    pub saving_throw: Option<SaveThrow>,
    pub spell_resistance: bool,
    pub description: String,
}

impl DBSpell {
    fn get_components(&self, conn: &Connection) -> Result<BTreeSet<SpellComponent>, db::Error> {
        let spell_components = DBSpellComponent::belonging_to(self)
            .load::<DBSpellComponent>(conn)
            .map_err(Error::RunQuery)?;
        spell_components.into_iter()
            .map(|spell_component| SpellComponent::db_get_by_id(&spell_component.id, conn))
            .collect()
    }

    fn get_effects(&self, conn: &Connection) -> Result<BTreeSet<Summary<Effect>>, db::Error> {
        let spell_effects = DBSpellEffect::belonging_to(self)
            .load::<DBSpellEffect>(conn)
            .map_err(Error::RunQuery)?;
        spell_effects.into_iter()
            .map(|spell_effect| Summary::<Effect>::db_get_by_id(&spell_effect.effect_id, conn))
            .collect()
    }
}

impl IntoDb for Spell {
    type DBType = (DBSpell, BTreeSet<DBSpellComponent>, BTreeSet<DBSpellEffect>);
    fn into_db(self) -> (DBSpell, BTreeSet<DBSpellComponent>, BTreeSet<DBSpellEffect>) {
        let effects = self.effects.into_iter()
            .map(|effect| {
                DBSpellEffect {
                    spell_id: self.id.to_owned(),
                    effect_id: effect.id().to_owned(),
                }
            }).collect();

        let components = self.components.into_iter()
            .map(|comp| comp.into_db(self.id.to_owned())).collect();

        let dbspell = DBSpell {
            id: self.id,
            name: self.name,
            level: self.level,
            school: self.school,
            casting_time: self.casting_time,
            range: self.range,
            area: self.area,
            duration_per_level: self.duration_per_level,
            saving_throw: self.saving_throw,
            spell_resistance: self.spell_resistance,
            description: self.description,
        };

        (dbspell, components, effects)
    }
}

#[derive(Associations, Identifiable, Insertable, Queryable, Delete, DeleteById, Insert, Ord, PartialOrd, PartialEq, Eq,)]
#[tavern(is_insertable, is_identifiable, is_queryable)]
#[tavern(id_field = "spell_id")]
#[table_name = "spelleffects"]
#[primary_key(spell_id, effect_id)]
#[belongs_to(DBSpell, foreign_key = "spell_id")]
pub struct DBSpellEffect {
    pub spell_id: Uuid,
    pub effect_id: Uuid,
}

#[derive(Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq, Clone, StandaloneDbMarker)]
pub struct SpellComponent {
    pub id: Uuid,
    pub item: Option<(Summary<Item>, i16)>,
    #[serde(rename = "type")]
    pub component_type: ComponentType,
}

impl Delete for SpellComponent {
    fn db_delete(&self, conn: &Connection) -> Result<(), Error> {
        SpellComponent::db_delete_by_id(&self.id, conn)
    }
}

impl Insert for SpellComponent {
    fn db_insert(&self, conn: &Connection) -> Result<(), Error> {
        unimplemented!()
    }
}

impl Update for SpellComponent {
    fn db_update(&self, conn: &Connection) -> Result<(), Error> {
        unimplemented!()
    }
}

impl IntoDbWithId for SpellComponent {
    type DBType = DBSpellComponent;
    fn into_db(self, spell_id: Uuid) -> DBSpellComponent {
        DBSpellComponent {
            id: self.id,
            spell_id,
            item_id: self.item.as_ref().map(|(item, _)| item.id().to_owned()),
            item_amount: self.item.as_ref().map(|(_, amount)| *amount),
            component_type: self.component_type,
        }
    }
}

impl TryFromDb for SpellComponent {
    type DBType = DBSpellComponent;
    fn try_from_db(db_sc: Self::DBType, conn: &Connection) -> Result<Self, Error> {
        // Database *should* be enforcing both item columns to be (not) null together
        // If code panics here, check the database design.
        let item = match db_sc.item_id {
            None => return Err(db::Error::InvalidValues(vec!["item_id".to_string(), "item_amount".to_string()])),
            Some(id) => {
                match db_sc.item_amount {
                    None => return Err(db::Error::InvalidValues(vec!["item_id".to_string(), "item_amount".to_string()])),
                    Some(amt) => Some((Summary::<Item>::db_get_by_id(&id, conn)?, amt))
                }
            }
        };

        let result = SpellComponent {
            id: db_sc.id,
            item,
            component_type: db_sc.component_type,
        };

        Ok(result)
    }
}

#[derive(
    AsChangeset,
    Associations,
    Identifiable,
    Insertable,
    Queryable,
    GetById,
    Delete,
    DeleteById,
    Insert,
    Update,
    Ord,
    PartialOrd,
    PartialEq,
    Eq,
)]
#[tavern(is_insertable, is_identifiable, is_queryable)]
#[table_name = "spellcomponents"]
#[belongs_to(DBSpell, foreign_key = "spell_id")]
pub struct DBSpellComponent {
    pub id: Uuid,
    pub spell_id: Uuid,
    pub item_id: Option<Uuid>,
    pub item_amount: Option<i16>,
    pub component_type: ComponentType,
}

#[derive(Serialize, Deserialize, DbEnum, Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum CasterType {
    Spontaneous,
    Prepared,
}

#[derive(Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq, DbEnum, Debug, Copy, Clone)]
pub enum ComponentType {
    Somatic,
    Material,
    Verbal,
}

#[derive(Serialize, Deserialize, DbEnum, Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, DbEnum, Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum SpellRange {
    Personal,
    Touch,
    Close,
    Medium,
    Long,
    Unlimited,
}
