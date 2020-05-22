use crate::item::Weapon;
use crate::spell::Spell;
use crate::summary::Summarize;
use crate::Links;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Summarize)]
pub struct Deity {
    links: Links,
    id: Uuid,
    name: String,
    description: String,
    favored_animals: Vec<String>,
    domains: Vec<Domain>,
    weapons: Vec<Weapon>,
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

#[derive(Serialize, Deserialize)]
pub struct Subdomain {
    name: String,
    description: String,
}
