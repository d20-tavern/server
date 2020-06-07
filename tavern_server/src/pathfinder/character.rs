use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::class::{Feature, Subclass};
use super::feat::Feat;
use super::item::{Bag, DBBag, Item};
use super::religion::Deity;
use super::spell::Spell;
use super::summary::{Summarize, Summary};
use super::Links;
use super::{Alignment, EquipmentSlot, Gender, Size};

use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::connection::Connection as DieselConnection;

//use tavern_derive::Summarize;
use crate::schema::{
    characterequipment, characterfeats, characterfeatures, characters, characterspells,
    charactersubclasses, races, racesubtypes, racetypes,
};
use std::cmp::Ordering;
use crate::db::{TryFromDb, IntoDb, Connection, Error, GetAll, GetById, Delete, DeleteById, Insert, Update, StandaloneDbMarker, IntoDbWithId};
use std::collections::{BTreeSet, BTreeMap};
use crate::forms::{self, TryFromForm};
use warp::{Rejection, Reply, Filter};
use nebula_form::Form;
use crate::api::{GetById as APIGetById, GetAll as APIGetAll, Insert as APIInsert, Update as APIUpdate, DeleteById as APIDeleteById, Filters, APIPath};
use warp::filters::BoxedFilter;
use nebula_status::{Status, StatusCode};

#[derive(Serialize, Deserialize, Summarize, Clone, Ord, PartialOrd, PartialEq, Eq, Debug)]
pub struct Character {
    id: Uuid,
    race: Race,
    deity: Option<Summary<Deity>>,
    subclasses: Vec<CharacterSubclass>,
    feats: Vec<Summary<Feat>>,
    spells: BTreeMap<Summary<Spell>, i16>,
    bags: BTreeSet<Summary<Bag>>,
    equipment: BTreeMap<EquipmentSlot, Summary<Item>>,
    features: Vec<Summary<Feature>>,

    name: String,
    age: i16,
    gender: Gender,
    alignment: Alignment,
    backstory: String,
    height: i16,
    weight: i16,
    size: Size,

    strength: i16,
    dexterity: i16,
    constitution: i16,
    intelligence: i16,
    wisdom: i16,
    charisma: i16,

    max_hp: i16,
    damage: i16,
    nonlethal: i16,

    copper: i16,
    silver: i16,
    gold: i16,
    platinum: i16,

    links: Links,
    #[serde(skip)]
    description: String,
}

//impl Filters for Character {
//    fn filters(parent_id: Option<Uuid>) -> BoxedFilter<(Box<dyn Reply>,)> {
//        let with_id = warp::path::param()
//                .and_then(|id| {
//                    Self::get_by_id(None, id)
//                        .or(Self::update(None, id))
//                        .or(Self::delete_by_id(None, id))
//                        //.or(Bags::filters(Some(id)))
//                        .unify()
//                });
//        warp::path("characters")
//            .and(
//                    with_id.or(
//                        Self::insert(None)
//                            .or(Self::get_all(None))
//                            .unify()
//                    ).unify()
//            )
//            .boxed()
//    }
//}

impl Character {
    const FIELD_NAME: &'static str = "name";
    const FIELD_RACE: &'static str = "race";
    const FIELD_AGE: &'static str = "age";
    const FIELD_GENDER: &'static str = "gender";
    const FIELD_ALIGNMENT: &'static str = "alignment";
    const FIELD_BACKSTORY: &'static str = "backstory";
    const FIELD_HEIGHT: &'static str = "height";
    const FIELD_WEIGHT: &'static str = "weight";
    const FIELD_SIZE: &'static str = "size";

    const FIELD_STRENGTH: &'static str = "strength";
    const FIELD_DEXTERITY: &'static str = "dexterity";
    const FIELD_CONSTITUTION: &'static str = "constitution";
    const FIELD_INTELLIGENCE: &'static str = "intelligence";
    const FIELD_WISDOM: &'static str = "wisdom";
    const FIELD_CHARISMA: &'static str = "charisma";

    const FIELD_MAX_HP: &'static str = "max-hp";
    const FIELD_DAMAGE: &'static str = "lethal-damage";
    const FIELD_NONLETHAL: &'static str = "nonlethal-damage";

    const FIELD_COPPER: &'static str = "copper";
    const FIELD_SILVER: &'static str = "silver";
    const FIELD_GOLD: &'static str = "gold";
    const FIELD_PLATINUM: &'static str = "platinum";

    const FIELD_DEITY: &'static str = "deity-id";
    const FIELD_SUBCLASSES: &'static str = "subclasses";
    const FIELD_FEATS: &'static str = "feats";
    const FIELD_SPELLS: &'static str = "spells";
    const FIELD_BAGS: &'static str = "bags";
    const FIELD_EQUIPMENT: &'static str = "equipment";
    const FIELD_FEATURES: &'static str = "features";
}

impl TryFromForm for Character {
    fn try_from_form(conn: &Connection, form: Form, this_id: Option<Uuid>, parent_id: Option<Uuid>) -> Result<Self, Rejection> where Self: Sized {
        let id = forms::valid_id_or_new::<Character>(this_id, conn)?;
        let name = forms::get_required_form_text_field(&form, Character::FIELD_NAME)?;
        let race = forms::get_required_form_text_field(&form, Character::FIELD_RACE)?;
        let race = forms::value_by_id(race, conn)?;
        let age = forms::get_required_form_text_field(&form, Character::FIELD_AGE)?;
        let gender = forms::get_required_form_text_field(&form, Character::FIELD_GENDER)?;
        let alignment = forms::get_required_form_text_field(&form, Character::FIELD_ALIGNMENT)?;
        let backstory = forms::get_required_form_text_field(&form, Character::FIELD_BACKSTORY)?;
        let height = forms::get_required_form_text_field(&form, Character::FIELD_HEIGHT)?;
        let weight = forms::get_required_form_text_field(&form, Character::FIELD_WEIGHT)?;
        let size = forms::get_required_form_text_field(&form, Character::FIELD_SIZE)?;

        let strength = forms::get_required_form_text_field(&form, Character::FIELD_STRENGTH)?;
        let dexterity = forms::get_required_form_text_field(&form, Character::FIELD_DEXTERITY)?;
        let constitution = forms::get_required_form_text_field(&form, Character::FIELD_CONSTITUTION)?;
        let intelligence = forms::get_required_form_text_field(&form, Character::FIELD_INTELLIGENCE)?;
        let wisdom = forms::get_required_form_text_field(&form, Character::FIELD_WISDOM)?;
        let charisma = forms::get_required_form_text_field(&form, Character::FIELD_CHARISMA)?;

        let max_hp = forms::get_required_form_text_field(&form, Character::FIELD_MAX_HP)?;
        let damage = forms::get_required_form_text_field(&form, Character::FIELD_DAMAGE)?;
        let nonlethal = forms::get_required_form_text_field(&form, Character::FIELD_NONLETHAL)?;

        let copper = forms::get_required_form_text_field(&form, Character::FIELD_COPPER)?;
        let silver = forms::get_required_form_text_field(&form, Character::FIELD_SILVER)?;
        let gold = forms::get_required_form_text_field(&form, Character::FIELD_GOLD)?;
        let platinum = forms::get_required_form_text_field(&form, Character::FIELD_PLATINUM)?;

        let deity = forms::get_optional_form_text_field(&form, Character::FIELD_DEITY)?
            .map(|id| forms::value_by_id(id, conn))
            .transpose()?;

        let subclasses = {
            use crate::schema::charactersubclasses::dsl::*;
            charactersubclasses.filter(char_id.eq(id))
                .load::<DBCharacterSubclass>(conn)
                .map_err(Error::RunQuery)
                .map_err(Rejection::from)?
                .into_iter()
                .map(|csc| CharacterSubclass::try_from_db(csc, conn))
                .collect::<Result<_, _>>()
        }?;

        let feats: String = forms::get_required_form_text_field(&form, Character::FIELD_FEATS)?;
        let feats = serde_json::from_str::<Vec<Uuid>>(&feats)
            .map_err(|_| forms::field_is_invalid_error(Character::FIELD_FEATS))?
            .into_iter()
            .map(|id| forms::value_by_id(id, conn))
            .collect::<Result<_, _>>()?;

        let spells: String = forms::get_required_form_text_field(&form, Character::FIELD_SPELLS)?;
        let spells = serde_json::from_str::<BTreeMap<Uuid, i16>>(&spells)
            .map_err(|_| forms::field_is_invalid_error(Character::FIELD_SPELLS))?
            .into_iter()
            .map::<Result<(Summary<Spell>, i16), Rejection>, _>(|(id, casts)| Ok((forms::value_by_id::<Summary<Spell>>(id, conn)?, casts)))
            .collect::<Result<BTreeMap<Summary<Spell>, i16>, _>>()?;

        let bags: String = forms::get_required_form_text_field(&form, Character::FIELD_BAGS)?;
        let bags = serde_json::from_str::<Vec<Uuid>>(&bags)
            .map_err(|_| forms::field_is_invalid_error(Character::FIELD_BAGS))?
            .into_iter()
            .map(|id| forms::value_by_id(id, conn))
            .collect::<Result<BTreeSet<Summary<Bag>>, _>>()?;

        let features: String = forms::get_required_form_text_field(&form, Character::FIELD_FEATURES)?;
        let features = serde_json::from_str::<Vec<Uuid>>(&features)
            .map_err(|_| forms::field_is_invalid_error(Character::FIELD_FEATURES))?
            .into_iter()
            .map(|id| forms::value_by_id(id, conn))
            .collect::<Result<_, _>>()?;

        let equipment: String = forms::get_required_form_text_field(&form, Character::FIELD_EQUIPMENT)?;
        let equipment = serde_json::from_str::<BTreeMap<String, Uuid>>(&equipment)
            .map_err(|_| forms::field_is_invalid_error(Character::FIELD_EQUIPMENT))?
            .into_iter()
            .map::<Result<(EquipmentSlot, Summary<Item>), Rejection>, _>(|(slot, id)| {
                let slot = slot.as_str().parse()
                    .map_err(|_| forms::field_is_invalid_error(Character::FIELD_EQUIPMENT))?;
                let item = forms::value_by_id(id, conn)?;
                Ok((slot, item))
            })
            .collect::<Result<_, _>>()?;

        let character = Character {
            id,
            race,
            deity,
            subclasses,
            feats,
            spells,
            bags,
            equipment,
            features,
            name,
            age,
            gender,
            alignment,
            backstory,
            height,
            weight,
            size,
            strength,
            dexterity,
            constitution,
            intelligence,
            wisdom,
            charisma,
            max_hp,
            damage,
            nonlethal,
            copper,
            silver,
            gold,
            platinum,
            links: Default::default(),
            description: Default::default(),
        };

        Ok(character)
    }
}

impl TryFromDb for Character {
    type DBType = DBCharacter;

    fn try_from_db(other: Self::DBType, conn: &Connection) -> Result<Self, Error> where Self: Sized {
        let race = Race::db_get_by_id(&other.race_id, conn)?;
        let deity = other.deity_id.map(|id| Summary::<Deity>::db_get_by_id(&id, conn)).transpose()?;
        let subclasses = other.get_subclasses(conn)?;
        let feats = other.get_feats(conn)?;
        let spells = other.get_spells(conn)?;
        let bags = other.get_bags(conn)?;
        let equipment = other.get_equipment(conn)?;
        let features = other.get_features(conn)?;
        let links = Links::new();
        let character = Character {
            id: other.id,
            race,
            deity,
            subclasses,
            feats,
            spells,
            bags,
            equipment,
            features,
            name: other.name,
            age: other.age,
            gender: other.gender,
            alignment: other.alignment,
            backstory: other.backstory,
            height: other.height,
            weight: other.weight,
            size: other.size,
            strength: other.strength,
            dexterity: other.dexterity,
            constitution: other.constitution,
            intelligence: other.intelligence,
            wisdom: other.wisdom,
            charisma: other.charisma,
            max_hp: other.max_hp,
            damage: other.damage,
            nonlethal: other.nonlethal,
            copper: other.copper,
            silver: other.silver,
            gold: other.gold,
            platinum: other.platinum,
            links,
            description: Default::default(),
        };

        Ok(character)
    }
}

impl IntoDb for Character {
    type DBType = (DBCharacter, Vec<DBCharacterSubclass>, Vec<DBCharacterFeat>, Vec<DBCharacterSpell>, Vec<DBCharacterEquipment>, Vec<DBCharacterFeature>);

    fn into_db(self) -> Self::DBType {
        let subclasses = self.subclasses.iter()
            .map(|sc| sc.to_owned().into_db(self.id.clone()))
            .collect();
        let feats = self.feats.iter()
            .map(|feat| DBCharacterFeat{
                char_id: self.id.clone(),
                feat_id: feat.id().to_owned(),
            })
            .collect();
        let spells = self.spells.iter()
            .map(|(spell, casts)| DBCharacterSpell {
                char_id: Default::default(),
                spell_id: Default::default(),
                casts_remaining: *casts,
            })
            .collect();
        let equipment = self.equipment.iter()
            .map(|(slot, item)| DBCharacterEquipment {
                char_id: self.id.clone(),
                item_id: item.id().to_owned(),
            })
            .collect();
        let features = self.features.iter()
            .map(|feature| DBCharacterFeature {
                char_id: self.id.clone(),
                feature_id: feature.id().to_owned(),
            })
            .collect();

        let character = DBCharacter {
            id: self.id,
            race_id: self.race.id,
            deity_id: self.deity.map(|deity| deity.id().to_owned()),
            name: self.name,
            age: self.age,
            gender: self.gender,
            alignment: self.alignment,
            backstory: self.backstory,
            height: self.height,
            weight: self.weight,
            size: self.size,
            strength: self.strength,
            dexterity: self.dexterity,
            constitution: self.constitution,
            intelligence: self.intelligence,
            wisdom: self.wisdom,
            charisma: self.charisma,
            max_hp: self.max_hp,
            damage: self.damage,
            nonlethal: self.nonlethal,
            copper: self.copper,
            silver: self.silver,
            gold: self.gold,
            platinum: self.platinum,
        };

        (character, subclasses, feats, spells, equipment, features)
    }
}

//impl Insert for Character {
//    fn db_insert(&self, conn: &Connection) -> Result<(), Error> {
//        conn.transaction::<_, Error, _>(|| {
//            let (character, subclasses, feats, spells, equipment, features) = self.into_db();
//            character.db_insert(conn)?;
//            for s in subclasses.into_iter() {
//                s.db_insert(conn)?;
//            }
//            for f in feats.into_iter() {
//                f.db_insert(conn)?;
//            }
//            for s in spells.into_iter() {
//                s.db_insert(conn)?;
//            }
//            for e in equipment.into_iter() {
//                e.db_insert(conn)?;
//            }
//            for f in features.into_iter() {
//                f.db_insert(conn)?;
//            }
//
//            Ok(())
//        })
//    }
//}
//
//impl Update for Character {
//    fn db_update(&self, conn: &Connection) -> Result<(), Error> {
//        conn.transaction::<_, Error, _>(|| {
//            let (character, subclasses, feats, spells, equipment, features) = self.into_db();
//            character.db_update(conn)?;
//            for s in subclasses.into_iter() {
//                s.db_update(conn)?;
//            }
//            for f in feats.into_iter() {
//                f.db_update(conn)?;
//            }
//            for s in spells.into_iter() {
//                s.db_update(conn)?;
//            }
//            for e in equipment.into_iter() {
//                e.db_update(conn)?;
//            }
//            for f in features.into_iter() {
//                f.db_update(conn)?;
//            }
//
//            Ok(())
//        })
//    }
//}
//
//impl DeleteById for Character {
//    fn db_delete_by_id(id: &Uuid, conn: &Connection) -> Result<(), Error> {
//        conn.transaction::<_, Error, _>(|| {
//            DBCharacter::db_delete_by_id(id, conn)?;
//
//            {
//                use crate::schema::charactersubclasses::dsl::*;
//                diesel::delete(charactersubclasses.filter(char_id.eq(id)))
//                    .execute(conn)
//                    .map_err(Error::RunQuery)
//            }?;
//
//            {
//                use crate::schema::characterfeats::dsl::*;
//                diesel::delete(characterfeats.filter(char_id.eq(id)))
//                    .execute(conn)
//                    .map_err(Error::RunQuery)
//            }?;
//
//            {
//                use crate::schema::characterspells::dsl::*;
//                diesel::delete(characterspells.filter(char_id.eq(id)))
//                    .execute(conn)
//                    .map_err(Error::RunQuery)
//            }?;
//
//            {
//                use crate::schema::characterequipment::dsl::*;
//                diesel::delete(characterequipment.filter(char_id.eq(id)))
//                    .execute(conn)
//                    .map_err(Error::RunQuery)
//            }?;
//
//            {
//                use crate::schema::characterfeatures::dsl::*;
//                diesel::delete(characterfeatures.filter(char_id.eq(id)))
//                    .execute(conn)
//                    .map_err(Error::RunQuery)
//            }?;
//
//            Ok(())
//        })
//    }
//}

impl Character {
    fn update_desc(&mut self) {
        let level = self.subclasses.iter().count();
        self.description = format!(
            "Level {} {} {} {}",
            level, &self.gender, &self.alignment, &self.race.name
        );
    }
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(GetAll, GetById, Delete, DeleteById, Insert, Update)]
#[tavern(is_identifiable, is_insertable, is_queryable)]
#[table_name = "characters"]
pub struct DBCharacter {
    id: Uuid,
    race_id: Uuid,
    deity_id: Option<Uuid>,

    name: String,
    age: i16,
    gender: Gender,
    alignment: Alignment,
    backstory: String,
    height: i16,
    weight: i16,
    size: Size,

    strength: i16,
    dexterity: i16,
    constitution: i16,
    intelligence: i16,
    wisdom: i16,
    charisma: i16,

    max_hp: i16,
    damage: i16,
    nonlethal: i16,

    copper: i16,
    silver: i16,
    gold: i16,
    platinum: i16,
}

impl DBCharacter {
    fn get_subclasses(&self, conn: &Connection) -> Result<Vec<CharacterSubclass>, Error> {
        DBCharacterSubclass::belonging_to(self)
            .load::<DBCharacterSubclass>(conn)
            .map_err(Error::RunQuery)?
            .into_iter()
            .map(|sc| CharacterSubclass::try_from_db(sc, conn))
            .collect()
    }
    fn get_feats(&self, conn: &Connection) -> Result<Vec<Summary<Feat>>, Error> {
        DBCharacterFeat::belonging_to(self)
            .load::<DBCharacterFeat>(conn)
            .map_err(Error::RunQuery)?
            .into_iter()
            .map(|f| Summary::<Feat>::db_get_by_id(&f.feat_id, conn))
            .collect()
    }
    fn get_spells(&self, conn: &Connection) -> Result<BTreeMap<Summary<Spell>, i16>, Error> {
        DBCharacterSpell::belonging_to(self)
            .load::<DBCharacterSpell>(conn)
            .map_err(Error::RunQuery)?
            .into_iter()
            .map(|s| Ok((Summary::<Spell>::db_get_by_id(&s.spell_id, conn)?, s.casts_remaining)))
            .collect()
    }
    fn get_bags(&self, conn: &Connection) -> Result<BTreeSet<Summary<Bag>>, Error> {
        DBBag::belonging_to(self)
            .load::<DBBag>(conn)
            .map_err(Error::RunQuery)?
            .into_iter()
            .map(|b| Summary::<Bag>::try_from_db(b, conn))
            .collect()
    }
    fn get_equipment(&self, conn: &Connection) -> Result<BTreeMap<EquipmentSlot, Summary<Item>>, Error> {
        DBCharacterEquipment::belonging_to(self)
            .load::<DBCharacterEquipment>(conn)
            .map_err(Error::RunQuery)?
            .into_iter()
            .map(|e| {
                // The database should ensure that all items marked as equipment
                // have an equipment slot. So, an unwrap should be safe here. If this
                // is ever not the case, ensure an INSERT/UPDATE trigger is set on the
                // database to catch this.
                Item::db_get_by_id(&e.item_id, conn)
                    .map(|i| (i.equip_slot.unwrap(), Summary::<Item>::from(&i)))
            })
            .collect()
    }
    fn get_features(&self, conn: &Connection) -> Result<Vec<Summary<Feature>>, Error> {
        DBCharacterFeature::belonging_to(self)
            .load::<DBCharacterFeature>(conn)
            .map_err(Error::RunQuery)?
            .into_iter()
            .map(|f| Summary::<Feature>::db_get_by_id(&f.feature_id, conn))
            .collect()
    }
}

#[derive(Serialize, Deserialize, Clone, Ord, PartialOrd, PartialEq, Eq, Debug)]
pub struct CharacterSubclass {
    pub subclass: Summary<Subclass>,
    pub levels_taken: i16,
    pub hp_taken: i16,
    pub skills_taken: i16,
}

impl TryFromDb for CharacterSubclass {
    type DBType = DBCharacterSubclass;

    fn try_from_db(other: Self::DBType, conn: &Connection) -> Result<Self, Error> where Self: Sized {
        let subclass = Summary::<Subclass>::db_get_by_id(&other.subclass_id, conn)?;
        let char_subclass = CharacterSubclass {
            subclass,
            levels_taken: other.levels_taken,
            hp_taken: other.hp_taken,
            skills_taken: other.skills_taken,
        };
        Ok(char_subclass)
    }
}

impl IntoDbWithId for CharacterSubclass {
    type DBType = DBCharacterSubclass;

    fn into_db(self, char_id: Uuid) -> Self::DBType {
        DBCharacterSubclass {
            subclass_id: self.subclass.id().to_owned(),
            char_id,
            levels_taken: self.levels_taken,
            hp_taken: self.hp_taken,
            skills_taken: self.skills_taken,
        }
    }
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(GetAll, Delete, Insert, Update)]
#[tavern(is_identifiable, is_insertable, is_queryable)]
#[table_name = "charactersubclasses"]
#[primary_key(char_id, subclass_id)]
#[belongs_to(DBCharacter, foreign_key = "char_id")]
pub struct DBCharacterSubclass {
    char_id: Uuid,
    subclass_id: Uuid,
    levels_taken: i16,
    hp_taken: i16,
    skills_taken: i16,
}

#[derive(Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(GetAll, Delete, Insert)]
#[tavern(is_identifiable, is_insertable, is_queryable)]
#[table_name = "characterfeats"]
#[primary_key(char_id, feat_id)]
#[belongs_to(DBCharacter, foreign_key = "char_id")]
pub struct DBCharacterFeat {
    char_id: Uuid,
    feat_id: Uuid,
}

#[derive(Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(GetAll, Delete, Insert)]
#[tavern(is_identifiable, is_insertable, is_queryable)]
#[table_name = "characterfeatures"]
#[primary_key(char_id, feature_id)]
#[belongs_to(DBCharacter, foreign_key = "char_id")]
pub struct DBCharacterFeature {
    char_id: Uuid,
    feature_id: Uuid,
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[table_name = "characterspells"]
#[derive(GetAll, Delete, Insert, Update)]
#[tavern(is_identifiable, is_insertable, is_queryable)]
#[primary_key(char_id, spell_id)]
#[belongs_to(DBCharacter, foreign_key = "char_id")]
pub struct DBCharacterSpell {
    char_id: Uuid,
    spell_id: Uuid,
    casts_remaining: i16,
}

#[derive(Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(GetAll, Delete, Insert)]
#[tavern(is_identifiable, is_insertable, is_queryable)]
#[table_name = "characterequipment"]
#[primary_key(char_id, item_id)]
#[belongs_to(DBCharacter, foreign_key = "char_id")]
pub struct DBCharacterEquipment {
    char_id: Uuid,
    item_id: Uuid,
}

// TODO: I think this can be implemented better

#[derive(Serialize, Deserialize, Summarize, Clone, Ord, PartialOrd, PartialEq, Eq, StandaloneDbMarker, Debug)]
pub struct Race {
    id: Uuid,
    links: Links,
    main_type: RaceType,
    sub_type: Option<RaceSubtype>,
    name: String,
    description: String,
    move_speed: i16,
    size: Size,
    languages: Vec<String>,
}

impl Race {
    const FIELD_NAME: &'static str = "name";
    const FIELD_DESCRIPTION: &'static str = "description";
    const FIELD_MAIN_TYPE: &'static str = "main-type";
    const FIELD_SUB_TYPE: &'static str = "sub-type";
    const FIELD_MOVE_SPEED: &'static str = "move-speed";
    const FIELD_SIZE: &'static str = "size";
    const FIELD_LANGUAGES: &'static str = "languages";
}

impl TryFromForm for Race {
    fn try_from_form(conn: &Connection, form: Form, this_id: Option<Uuid>, parent_id: Option<Uuid>) -> Result<Self, Rejection> where Self: Sized {
        let id = forms::valid_id_or_new::<Race>(this_id, conn)?;
        let name = forms::get_required_form_text_field(&form, Race::FIELD_NAME)?;
        let description = forms::get_required_form_text_field(&form, Race::FIELD_DESCRIPTION)?;
        let main_type = forms::get_required_form_text_field(&form, Race::FIELD_MAIN_TYPE)?;
        let main_type = forms::value_by_id(main_type, conn)?;
        let sub_type = forms::get_optional_form_text_field(&form, Race::FIELD_SUB_TYPE)?
            .map(|id| forms::value_by_id(id, conn))
            .transpose()?;
        let move_speed = forms::get_required_form_text_field(&form, Race::FIELD_MOVE_SPEED)?;
        let size = forms::get_required_form_text_field(&form, Race::FIELD_SIZE)?;
        let languages: String = forms::get_required_form_text_field(&form, Race::FIELD_LANGUAGES)?;
        let languages = serde_json::from_str(&languages)
            .map_err(|_| forms::field_is_invalid_error(Race::FIELD_LANGUAGES))?;

        let race = Race {
            id,
            links: Default::default(),
            name,
            description,
            main_type,
            sub_type,
            move_speed,
            size,
            languages
        };

        Ok(race)
    }
}

impl TryFromDb for Race {
    type DBType = DBRace;

    fn try_from_db(other: Self::DBType, conn: &Connection) -> Result<Self, Error> where Self: Sized {
        let main_type = RaceType::db_get_by_id(&other.type_id, conn)?;
        let sub_type = other.subtype_id.map(|id| RaceSubtype::db_get_by_id(&id, conn)).transpose()?;
        let links = Links::new();
        let race = Race {
            id: other.id,
            links,
            main_type,
            sub_type,
            name: other.name,
            description: other.description,
            move_speed: other.move_speed,
            size: other.size,
            languages: other.languages,
        };
        Ok(race)
    }
}

impl IntoDb for Race {
    type DBType = DBRace;

    fn into_db(self) -> Self::DBType {
        DBRace {
            id: self.id,
            type_id: self.main_type.id,
            subtype_id: self.sub_type.map(|sub| sub.id),
            name: self.name,
            description: self.description,
            move_speed: self.move_speed,
            size: self.size,
            languages: self.languages,
        }
    }
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[derive(GetAll, GetById, Delete, DeleteById, Insert, Update)]
#[tavern(is_identifiable, is_insertable, is_queryable)]
#[table_name = "races"]
pub struct DBRace {
    id: Uuid,
    description: String,
    type_id: Uuid,
    subtype_id: Option<Uuid>,
    name: String,
    move_speed: i16,
    size: Size,
    languages: Vec<String>,
}

#[derive(Serialize, Deserialize, AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone, Debug)]
#[derive(GetAll, GetById, Delete, DeleteById, Insert, Update)]
#[tavern(is_identifiable, is_insertable, is_queryable)]
#[table_name = "racetypes"]
pub struct RaceType {
    id: Uuid,
    name: String,
    hit_die: String,
    bab_per_hit_die: f32,
}

impl RaceType {
    const FIELD_NAME: &'static str = "name";
    const FIELD_HIT_DIE: &'static str = "hit-die";
    const FIELD_BAB_PER_HIT_DIE: &'static str = "bab-per-hit-die";
}

impl TryFromForm for RaceType {
    fn try_from_form(conn: &Connection, form: Form, this_id: Option<Uuid>, parent_id: Option<Uuid>) -> Result<Self, Rejection> where Self: Sized {
        let id = forms::valid_id_or_new::<RaceType>(this_id, conn)?;
        let name = forms::get_required_form_text_field(&form, RaceType::FIELD_NAME)?;
        let hit_die = forms::get_required_form_text_field(&form, RaceType::FIELD_HIT_DIE)?;
        let bab_per_hit_die = forms::get_required_form_text_field(&form, RaceType::FIELD_BAB_PER_HIT_DIE)?;

        Ok(RaceType{ id, name, hit_die, bab_per_hit_die })
    }
}

impl Ord for RaceType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for RaceType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for RaceType {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for RaceType{}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable, Clone, Ord, PartialOrd, PartialEq, Eq, Debug)]
#[derive(GetAll, GetById, Delete, DeleteById, Insert, Update, Filters)]
#[tavern(is_identifiable, is_insertable, is_queryable, api_path = "race-subtypes")]
#[table_name = "racesubtypes"]
#[derive(Serialize, Deserialize)]
pub struct RaceSubtype {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

impl RaceSubtype {
    pub const FIELD_NAME: &'static str = "name";
    pub const FIELD_DESCRIPTION: &'static str = "description";
}

impl TryFromForm for RaceSubtype {
    fn try_from_form(conn: &Connection, form: Form, this_id: Option<Uuid>, parent_id: Option<Uuid>) -> Result<Self, Rejection> where Self: Sized {
        let id = forms::valid_id_or_new::<RaceSubtype>(this_id, conn)?;
        let name: String = forms::get_required_form_text_field(&form, RaceSubtype::FIELD_NAME)?;
        let description = forms::get_required_form_text_field(&form, RaceSubtype::FIELD_DESCRIPTION)?;

        if name.is_empty() {
            return Err(forms::field_is_invalid_error(RaceSubtype::FIELD_NAME));
        }

        Ok(RaceSubtype{ id, name, description })
    }
}