use super::item::Weapon;
use super::spell::Spell;
use super::summary::Summarize;
use super::Links;
use crate::schema::{deities, deitydomains, deityweapons, domains, domainspells, subdomains};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::BTreeSet;
use crate::pathfinder::summary::Summary;
use crate::db::{self, Connection, GetById, GetAll, Error, Delete, DeleteById, Insert, Update, IntoDbWithId, TryFromDb, IntoDb, StandaloneDbMarker};
use diesel::prelude::*;
use diesel::Connection as DieselConnection;

#[derive(Serialize, Deserialize, Summarize, Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct Deity {
    links: Links,
    id: Uuid,
    name: String,
    description: String,
    favored_animals: Option<String>,
    domains: BTreeSet<Summary<Domain>>,
    weapons: BTreeSet<Summary<Weapon>>,
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[derive(GetById, GetAll, Delete, DeleteById, Insert, Update, Ord, PartialOrd, PartialEq, Eq)]
#[tavern(is_insertable, is_identifiable, is_queryable)]
#[table_name = "deities"]
pub struct DBDeity {
    id: Uuid,
    name: String,
    description: String,
    favored_animals: Option<String>,
}

impl TryFromDb for Deity {
    type DBType = DBDeity;
    fn try_from_db(db_deity: DBDeity, conn: &Connection) -> Result<Self, Error> {
        let domains = db_deity.try_get_domains(conn)?;
        let weapons = db_deity.try_get_weapons(conn)?;
        let mut links = Links::new();
        links.insert("self".to_string(), format!("/deities/{}", db_deity.id));

        let deity = Deity {
            id: db_deity.id,
            links,
            name: db_deity.name,
            description: db_deity.description,
            favored_animals: db_deity.favored_animals,
            domains,
            weapons,
        };

        Ok(deity)
    }
}

impl IntoDb for Deity {
    type DBType = (DBDeity, BTreeSet<DBDeityDomain>, BTreeSet<DBDeityWeapon>);
    fn into_db(self) -> (DBDeity, BTreeSet<DBDeityDomain>, BTreeSet<DBDeityWeapon>) {
        let domains = self.domains.iter()
            .map(|domain| domain.to_owned().into_db(self.id.to_owned()))
            .collect();

        let weapons = self.weapons.iter()
            .map(|weapon| weapon.to_owned().into_db(self.id))
            .collect();

        let deity = DBDeity {
            id: self.id.to_owned(),
            name: self.name,
            description: self.description,
            favored_animals: self.favored_animals,
        };

        (deity, domains, weapons)
    }
}

impl DBDeity {
    fn try_get_domains(&self, conn: &Connection) -> Result<BTreeSet<Summary<Domain>>, db::Error> {
        DBDeityDomain::belonging_to(self)
            .load::<DBDeityDomain>(conn)
            .map_err(db::Error::RunQuery)?
            .into_iter()
            .map(|domain| Summary::<Domain>::db_get_by_id(&domain.domain_id, conn))
            .collect::<Result<BTreeSet<Summary<Domain>>, Error>>()
    }

    fn try_get_weapons(&self, conn: &Connection) -> Result<BTreeSet<Summary<Weapon>>, db::Error> {
        DBDeityWeapon::belonging_to(self)
            .load::<DBDeityWeapon>(conn)
            .map_err(db::Error::RunQuery)?
            .into_iter()
            .map(|weapon| Summary::<Weapon>::db_get_by_id(&weapon.item_id, conn))
            .collect::<Result<BTreeSet<Summary<Weapon>>, db::Error>>()
    }
}

#[derive(Associations, Identifiable, Insertable, Queryable, Delete, Insert, Ord, PartialOrd, PartialEq, Eq)]
#[table_name = "deitydomains"]
#[tavern(is_insertable, is_identifiable, is_queryable)]
#[primary_key(deity_id, domain_id)]
#[tavern(id_field = "deity_id")]
#[belongs_to(DBDeity, foreign_key = "deity_id")]
pub struct DBDeityDomain {
    deity_id: Uuid,
    domain_id: Uuid,
}

impl IntoDbWithId for Summary<Domain> {
    type DBType = DBDeityDomain;
    fn into_db(self, deity_id: Uuid) -> DBDeityDomain {
        DBDeityDomain {
            deity_id,
            domain_id: self.id().to_owned(),
        }
    }
}

#[derive(Associations, Identifiable, Insertable, Queryable, DeleteById, Insert, Ord, PartialOrd, PartialEq, Eq)]
#[table_name = "deityweapons"]
#[tavern(is_insertable, is_identifiable, is_queryable, id_field = "deity_id")]
#[primary_key(deity_id, item_id)]
#[belongs_to(DBDeity, foreign_key = "deity_id")]
pub struct DBDeityWeapon {
    deity_id: Uuid,
    item_id: Uuid,
}

//impl TryFromDb for Summary<Weapon> {
//    type DBType = DBDeityWeapon;
//    fn try_from_db(db_weapon: DBDeityWeapon, conn: &Connection) -> Result<Self, Error> {
//        Summary::<Weapon>::db_get_by_id(&db_weapon.item_id, conn)
//    }
//}

impl IntoDbWithId for Summary<Weapon> {
    type DBType = DBDeityWeapon;
    fn into_db(self, deity_id: Uuid) -> DBDeityWeapon {
        DBDeityWeapon {
            deity_id,
            item_id: self.id().to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Summarize, Ord, PartialOrd, PartialEq, Eq, Clone)]
pub struct Domain {
    id: Uuid,
    links: Links,
    name: String,
    description: String,
    power_description: String,
    subdomains: BTreeSet<Subdomain>,
    spells: BTreeSet<(Summary<Spell>, i16)>,
}

impl DeleteById for Domain {
    fn db_delete_by_id(id: &Uuid, conn: &Connection) -> Result<(), Error> {
        conn.transaction(|| {
            DBDomain::db_delete_by_id(id, conn)?;
            diesel::delete({
                    use crate::schema::domainspells::dsl::*;
                    domainspells.filter(domain_id.eq(id))
                })
                .execute(conn)
                .map_err(db::Error::RunQuery)?;
            diesel::delete({
                    use crate::schema::subdomains::dsl::*;
                    subdomains.filter(domain_id.eq(id))
                })
                .execute(conn)
                .map_err(db::Error::RunQuery)
                .map(|_| ())
        })
    }
}

impl Insert for Domain {
    fn db_insert(&self, conn: &Connection) -> Result<(), Error> {
        // Assumes that subdomains have already been inserted
        // or will otherwise be inserted on their own. Expected workflow:
        // 1. Create domain on client and POST to server
        // 2. Domain gets created
        // 3. Add subdomains to the domain
        conn.transaction(|| {
            let (domain, _subdomains, spells) = self.to_owned().into_db();
            domain.db_insert(conn)?;
            for spell in spells {
                spell.db_insert(conn)?;
            }
            Ok(())
        })
    }
}

impl Update for Domain {
    fn db_update(&self, conn: &Connection) -> Result<(), Error> {
        // Subdomains are expected to be managed separately.
        // For more info, see the not on Insert.
        conn.transaction(|| {
            let (domain, _subdomains, spells) = self.to_owned().into_db();
            let old_spells = domain.try_get_spells(conn)?;
            let delete_spells = old_spells.difference(&self.spells);
            let add_spells = self.spells.difference(&old_spells);

            for spell in delete_spells {
                spell.to_owned().into_db(self.id.to_owned()).db_delete(conn)?;
            }
            for spell in add_spells {
                spell.to_owned().into_db(self.id.to_owned()).db_insert(conn)?;
            }

            domain.db_update(conn)
        })
    }
}

impl TryFromDb for Domain {
    type DBType = DBDomain;
    fn try_from_db(db_domain: DBDomain, conn: &Connection) -> Result<Self, Error> {
        let subdomains = db_domain.try_get_subdomains(conn)?;
        let spells = db_domain.try_get_spells(conn)?;
        let mut links = Links::new();
        links.insert("self".to_string(), format!("/domains/{}", db_domain.id));
        let domain = Domain {
            id: db_domain.id,
            links,
            name: db_domain.name,
            description: db_domain.description,
            power_description: db_domain.power_description,
            subdomains,
            spells,
        };

        Ok(domain)
    }
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[derive(GetAll, GetById, Delete, DeleteById, Insert, Update, Ord, PartialOrd, PartialEq, Eq)]
#[tavern(is_insertable, is_identifiable, is_queryable)]
#[table_name = "domains"]
pub struct DBDomain {
    id: Uuid,
    name: String,
    description: String,
    power_description: String,
}

impl IntoDb for Domain {
    type DBType = (DBDomain, BTreeSet<DBSubdomain>, BTreeSet<DBDomainSpell>);
    fn into_db(self) -> (DBDomain, BTreeSet<DBSubdomain>, BTreeSet<DBDomainSpell>) {
        let db_subdomains = self.subdomains.iter()
            .map(|sd| sd.to_owned().into_db(self.id.to_owned()))
            .collect();

        let db_spells = self.spells.iter()
            .map(|(spell, casts)| DBDomainSpell {
                domain_id: self.id.to_owned(),
                spell_id: spell.id().to_owned(),
                casts: *casts,
            })
            .collect();

        let db_domain = DBDomain {
            id: self.id.to_owned(),
            name: self.name,
            description: self.description,
            power_description: self.power_description,
        };

        (db_domain, db_subdomains, db_spells)
    }
}

impl DBDomain {
    fn try_get_spells(&self, conn: &Connection) -> Result<BTreeSet<(Summary<Spell>, i16)>, db::Error> {
        let spells = DBDomainSpell::belonging_to(self)
            .load::<DBDomainSpell>(conn)
            .map_err(db::Error::RunQuery)?;
        let spells = spells
            .into_iter()
            .map(|spell| <(Summary::<Spell>, i16)>::try_from_db(spell, conn))
            .collect::<Result<BTreeSet<(Summary<Spell>, i16)>, db::Error>>()?;
        Ok(spells)
    }

    fn try_get_subdomains(&self, conn: &Connection) -> Result<BTreeSet<Subdomain>, db::Error> {
        let subdomains = DBSubdomain::belonging_to(self)
            .load::<DBSubdomain>(conn)
            .map_err(db::Error::RunQuery)?;
        let subdomains = subdomains
            .into_iter()
            .map(Subdomain::from)
            .collect();
        Ok(subdomains)
    }
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[derive(GetAll, Insert, Ord, PartialOrd, PartialEq, Eq)]
#[tavern(is_insertable, is_identifiable, is_queryable)]
#[table_name = "domainspells"]
#[primary_key(domain_id, spell_id)]
#[belongs_to(DBDomain, foreign_key = "domain_id")]
pub struct DBDomainSpell {
    domain_id: Uuid,
    spell_id: Uuid,
    casts: i16,
}

impl Delete for DBDomainSpell {
    fn db_delete(&self, conn: &Connection) -> Result<(), Error> {
        use crate::schema::domainspells::dsl::*;
        diesel::delete(domainspells.filter(domain_id.eq(self.domain_id)).filter(spell_id.eq(self.spell_id)))
            .execute(conn)
            .map_err(db::Error::RunQuery)
            .map(|_| ())
    }
}

impl Update for DBDomainSpell {
    fn db_update(&self, conn: &Connection) -> Result<(), Error> {
        use crate::schema::domainspells::dsl::*;
        diesel::update(domainspells.filter(domain_id.eq(self.domain_id)).filter(spell_id.eq(self.spell_id)))
            .set(self)
            .execute(conn)
            .map_err(db::Error::RunQuery)
            .map(|_| ())
    }
}

impl TryFromDb for (Summary<Spell>, i16) {
    type DBType = DBDomainSpell;
    fn try_from_db(ds: DBDomainSpell, conn: &Connection) -> Result<Self, Error> {
        Ok((Summary::<Spell>::db_get_by_id(&ds.spell_id, conn)?, ds.casts))
    }
}

impl IntoDbWithId for (Summary<Spell>, i16) {
    type DBType = DBDomainSpell;
    fn into_db(self, domain_id: Uuid) -> DBDomainSpell {
        let (spell, casts) = self;
        DBDomainSpell {
            domain_id,
            spell_id: spell.id().to_owned(),
            casts,
        }
    }
}

#[derive(Serialize, Deserialize, Summarize, Ord, PartialOrd, PartialEq, Eq, StandaloneDbMarker, Clone)]
pub struct Subdomain {
    id: Uuid,
    links: Links,
    name: String,
    description: String,
}

impl GetById for Subdomain {
    fn db_get_by_id(id: &Uuid, conn: &Connection) -> Result<Self, Error> where
        Self: Sized {
        DBSubdomain::db_get_by_id(id, conn)
            .map(Subdomain::from)
    }
}

impl GetAll for Subdomain {
    fn db_get_all(conn: &Connection) -> Result<Vec<Self>, Error> where
        Self: Sized {
        DBSubdomain::db_get_all(conn)
            .map(|sds| {
                sds.into_iter()
                    .map(Subdomain::from)
                    .collect()
            })
    }
}

impl From<DBSubdomain> for Subdomain {
    fn from(other: DBSubdomain) -> Subdomain {
        let mut links = Links::new();
        links.insert("domain".to_string(), format!("/domains/{}", other.domain_id));
        links.insert("self".to_string(), format!("/domains/{}/subdomains/{}", other.domain_id, other.id));

        Subdomain {
            id: other.id,
            links,
            name: other.name,
            description: other.description,
        }
    }
}

impl IntoDbWithId for Subdomain {
    type DBType = DBSubdomain;
    fn into_db(self, domain_id: Uuid) -> DBSubdomain {
        DBSubdomain {
            id: self.id,
            domain_id,
            name: self.name,
            description: self.description,
        }
    }
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Ord, PartialOrd, PartialEq, Eq)]
#[derive(Summarize, GetAll, GetById, Delete, DeleteById, Insert, Update)]
#[tavern(is_insertable, is_identifiable, is_queryable)]
#[table_name = "subdomains"]
#[belongs_to(DBDomain, foreign_key = "domain_id")]
pub struct DBSubdomain {
    id: Uuid,
    domain_id: Uuid,
    name: String,
    description: String,
}
