use crate::item::Weapon;
use crate::spell::Spell;
use crate::summary::Summarize;
use crate::Links;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[cfg( feature = "tavern")]
use tavern_db::{TryFromRow, TryFromUuid};

#[derive(Serialize, Deserialize, Summarize)]
pub struct Deity {
    links: Links,
    id: Uuid,
    name: String,
    description: String,
    favored_animals: Option<String>,
    domains: Vec<Domain>,
    weapons: Vec<Weapon>,
}

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "Deities")]
pub struct DBDeity {
    id: Uuid,
    name: String,
    description: String,
    favored_animals: Option<String>,
}

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "DeityDomains")]
#[cfg_attr(feature = "tavern", belongs_to(DBDeity, foreign_key = "deity_id"))]
#[cfg_attr(feature = "tavern", belongs_to(DBDomain, foreign_key = "domain_id"))]
pub struct DBDeityDomain {
    deity_id: Uuid,
    domain_id: Uuid,
}

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "DeityWeapons")]
#[cfg_attr(feature = "tavern", belongs_to(DBDeity, foreign_key = "deity_id"))]
#[cfg_attr(feature = "tavern", belongs_to(DBDomain, foreign_key = "domain_id"))]
pub struct DBDeityWeapon {
    deity_id: Uuid,
    domain_id: Uuid,
}

#[derive(Serialize, Deserialize, Summarize)]
pub struct Domain {
    id: Uuid,
    links: Links,
    name: String,
    description: String,
    power_description: String,
    subdomains: Vec<Subdomain>,
    spells: Vec<Spell>,
}

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "Domains")]
pub struct DBDomain {
    id: Uuid,
    name: String,
    description: String,
    power_description: String,
}

#[derive(Serialize, Deserialize, Summarize)]
pub struct Subdomain {
    id: Uuid,
    links: Links,
    name: String,
    description: String,
}

#[cfg(feature = "tavern")]
#[cfg_attr(feature = "tavern", derive(AsChangeSet, Associations, Identifiable, Insertable, Queryable))]
#[cfg_attr(feature = "tavern", table_name = "Domains")]
#[cfg_attr(feature = "tavern", belongs_to(DBDomain, foreign_key = "domain_id"))]
pub struct DBSubdomain {
    id: Uuid,
    domain_id: Uuid,
    name: String,
    description: String,
}