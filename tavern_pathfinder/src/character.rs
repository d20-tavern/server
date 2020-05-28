use crate::Links;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[cfg( feature = "tavern")]
use tavern_db::{TryFromRow, TryFromUuid};

//Additional modules
use crate::class::Subclass;
use crate::feat::Feat;
use crate::item::{Bag, Item};
use crate::religion::Deity;
use crate::spell::Spell;
use crate::summary::{Summarize, Summary};

//Enums
use crate::Alignment;
use crate::Gender;
use crate::Size;

#[derive(Serialize, Deserialize, Summarize)]
#[cfg_attr(feature = "tavern", derive(TryFromRow, TryFromUuid))]
#[cfg_attr(feature = "tavern", tavern(
    select_post_op = "instance.update_desc();",
    verify_user = "user_id",
))]
pub struct Character {
    #[cfg_attr(feature = "tavern", tavern(skip, default = "Links::new()"))]
    links: Links,

    id: Uuid,
    #[cfg_attr(feature = "tavern", tavern(references = "Race", column_name = "race_id"))]
    race: Race,
    #[cfg_attr(feature = "tavern", tavern(references = "Summary<Deity>", column_name = "deity_id"))]
    deity: Summary<Deity>,

    #[cfg_attr(feature = "tavern", tavern(
        references = "Summary<Subclass>",
        column = "ARRAY(SELECT subclass_id FROM CharacterSubclasses WHERE characterSubclasses.char_id = $1)",
        column_name = "subclasses",
        is_array,
    ))]
    subclasses: Vec<Summary<Subclass>>,
    #[cfg_attr(feature = "tavern", tavern(
        references = "Summary<Feat>",
        column = "ARRAY(SELECT feat_id FROM CharacterFeats WHERE CharacterFeats.char_id = $1)",
        column_name = "feats",
        is_array,
    ))]
    feats: Vec<Summary<Feat>>,
    #[cfg_attr(feature = "tavern", tavern(
        references = "Summary<Spell>",
        column = "ARRAY(SELECT spell_id FROM CharacterSpells WHERE CharacterSpells.char_id = $1)",
        column_name = "spells",
        is_array,
    ))]
    spells: Vec<Summary<Spell>>,
    #[cfg_attr(feature = "tavern", tavern(
        references = "Summary<Bag>",
        column = "ARRAY(SELECT id FROM Bags WHERE Bags.char_id = $1)",
        column_name = "bags",
        is_array,
    ))]
    bags: Vec<Summary<Bag>>,
    #[cfg_attr(feature = "tavern", tavern(
        references = "Summary<Item>",
        column = "ARRAY(SELECT item_id FROM CharacterEquipment WHERE CharacterEquipment.char_id = $1)",
        column_name = "equipment",
        is_array,
    ))]
    equipment: Vec<Summary<Item>>,

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

    #[serde(skip)]
    #[cfg_attr(feature = "tavern", tavern(skip, default = "String::new()"))]
    description: String,
}

impl Character {
    fn update_desc(&mut self) {
        let level = self.subclasses.iter().count();
        self.description = format!("Level {} {} {} {}", level, &self.gender, &self.alignment, &self.race.name);
    }
}

// TODO: I think this can be implemented better

#[derive(Serialize, Deserialize, Summarize)]
#[cfg_attr(feature = "tavern", derive(TryFromRow, TryFromUuid))]
pub struct Race {
    id: Uuid,
    #[cfg_attr(feature = "tavern", tavern(skip, default = "Links::new()"))]
    links: Links,
    #[cfg_attr(feature = "tavern", tavern(
        references = "RaceType",
        column_name = "type_id",
    ))]
    main_type: RaceType,
    #[cfg_attr(feature = "tavern", tavern(
        references = "RaceSubtype",
        column_name = "subtype_id",
    ))]
    sub_type: RaceSubtype,
    name: String,
    description: String,
    move_speed: i16,
    size: Size,
    languages: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "tavern", derive(TryFromRow, TryFromUuid))]
pub struct RaceType {
    id: Uuid,
    name: String,
    hit_die: String,
    bab_per_hit_die: f32,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "tavern", derive(TryFromRow, TryFromUuid))]
pub struct RaceSubtype {
    id: Uuid,
    name: String,
    description: String,
}
