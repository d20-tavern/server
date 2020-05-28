use crate::item::Weapon;
use crate::spell::Spell;
use crate::summary::Summarize;
use crate::Links;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[cfg( feature = "tavern")]
use tavern_db::{TryFromRow, TryFromUuid};

#[derive(Serialize, Deserialize, Summarize)]
#[cfg_attr(feature = "tavern", derive(TryFromRow, TryFromUuid))]
pub struct Deity {
    #[cfg_attr(feature = "tavern", tavern(
        skip, default = "Links::new()"
    ))]
    links: Links,
    id: Uuid,
    name: String,
    description: String,
    #[cfg_attr(feature = "tavern", tavern())]
    favored_animals: Vec<String>,
    #[cfg_attr(feature = "tavern", tavern(
        references = "Domain",
        column = "ARRAY(SELECT domain_id FROM DeityDomains WHERE DeityDomains.deity_id = $1)",
        is_array
    ))]
    domains: Vec<Domain>,
    #[cfg_attr(feature = "tavern", tavern(
        references = "Weapon",
        column = "ARRAY(SELECT item_id FROM DeityWeapons WHERE DeityWeapons.deity_id = $1)",
        is_array
    ))]
    weapons: Vec<Weapon>,
}

#[derive(Serialize, Deserialize, Summarize)]
#[cfg_attr(feature = "tavern", derive(TryFromRow, TryFromUuid))]
pub struct Domain {
    id: Uuid,
    #[cfg_attr(feature = "tavern", tavern(
        skip, default = "Links::new()"
    ))]
    links: Links,
    name: String,
    description: String,
    power_description: String,

    #[cfg_attr(feature = "tavern", tavern(
        references = "Subdomain",
        column = "ARRAY(SELECT id FROM Subdomains WHERE Subdomains.domain_id = $1)",
        is_array
    ))]
    subdomains: Vec<Subdomain>,
    #[cfg_attr(feature = "tavern", tavern(
        references = "Spell",
        column = "ARRAY(SELECT spell_id FROM DomainSpells WHERE DomainSpells.domain_id = $1)",
        is_array
    ))]
    spells: Vec<Spell>,
}

#[derive(Serialize, Deserialize, Summarize)]
#[cfg_attr(feature = "tavern", derive(TryFromRow, TryFromUuid))]
pub struct Subdomain {
    id: Uuid,
    #[cfg_attr(feature = "tavern", tavern(
        skip, default = "Links::new()"
    ))]
    links: Links,
    name: String,
    description: String,
}
