use super::item::Weapon;
use super::spell::Spell;
use super::summary::Summarize;
use super::Links;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::schema::{deities, deitydomains, deityweapons, domains, subdomains};

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

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[table_name = "deities"]
pub struct DBDeity {
    id: Uuid,
    name: String,
    description: String,
    favored_animals: Option<String>,
}

#[derive(Associations, Identifiable, Insertable, Queryable)]
#[table_name = "deitydomains"]
#[primary_key(deity_id, domain_id)]
#[belongs_to(DBDeity, foreign_key = "deity_id")]
pub struct DBDeityDomain {
    deity_id: Uuid,
    domain_id: Uuid,
}

#[derive(Associations, Identifiable, Insertable, Queryable)]
#[table_name = "deityweapons"]
#[primary_key(deity_id, item_id)]
#[belongs_to(DBDeity, foreign_key = "deity_id")]
pub struct DBDeityWeapon {
    deity_id: Uuid,
    item_id: Uuid,
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

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[table_name = "domains"]
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

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[table_name = "subdomains"]
#[belongs_to(DBDomain, foreign_key = "domain_id")]
pub struct DBSubdomain {
    id: Uuid,
    domain_id: Uuid,
    name: String,
    description: String,
}