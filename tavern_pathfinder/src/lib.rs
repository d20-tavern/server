use serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::collections::BTreeMap;
use std::hash::Hash;
use tavern_derive::Display;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[cfg_attr(feature = "tavern", derive(sqlx::Type))]
#[derive(Serialize, Deserialize, Display)]
pub enum Gender {
    Male,
    Female,
    Other,
}

#[cfg_attr(feature = "tavern", derive(sqlx::Type))]
#[derive(Serialize, Deserialize, Display)]
pub enum Size {
    Fine,
    Diminutive,
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
    Colossal,
}

#[cfg_attr(feature = "tavern", derive(sqlx::Type))]
#[derive(Serialize, Deserialize, Display)]
pub enum Alignment {
    #[cfg_attr(feature = "tavern", sqlx(rename = "Lawful Good"))]
    LawfulGood,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Lawful Neutral"))]
    LawfulNeutral,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Lawful Evil"))]
    LawfulEvil,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Neutral Good"))]
    NeutralGood,
    #[cfg_attr(feature = "tavern", sqlx(rename = "True Neutral"))]
    TrueNeutral,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Neutral Evil"))]
    NeutralEvil,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Chaotic Good"))]
    ChaoticGood,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Chaotic Neutral"))]
    ChaoticNeutral,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Chaotic Evil"))]
    ChaoticEvil,
}

#[cfg_attr(feature = "tavern", derive(sqlx::Type))]
#[derive(Serialize, Deserialize, Display, PartialOrd, Ord, Hash, PartialEq, Eq)]
pub enum Attribute {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

pub type Attributes = BTreeMap<Attribute, i8>;

#[cfg_attr(feature = "tavern", derive(sqlx::Type))]
#[derive(Serialize, Deserialize, Display, PartialOrd, Ord, Hash, PartialEq, Eq)]
pub enum Skill {
    Acrobatics,
    Appraise,
    Bluff,
    Climb,
    Craft,
    Diplomacy,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Disable Device"))]
    DisableDevice,
    Disguise,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Escape Artist"))]
    EscapeArtist,
    Fly,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Handle Animal"))]
    HandleAnimal,
    Heal,
    Intimidate,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Knowledge (arcana)"))]
    KnowledgeArcana,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Knowledge (dungeoneering)"))]
    KnowledgeDungeoneering,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Knowledge (engineering)"))]
    KnowledgeEngineering,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Knowledge (geography)"))]
    KnowledgeGeography,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Knowledge (history)"))]
    KnowledgeHistory,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Knowledge (local)"))]
    KnowledgeLocal,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Knowledge (nobility)"))]
    KnowledgeNobility,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Knowledge (planes)"))]
    KnowledgePlanes,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Knowledge (religion)"))]
    KnowledgeReligion,
    Linguistics,
    Perception,
    Perform,
    Profession,
    Ride,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Sense Motive"))]
    SenseMotive,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Sleight of Hand"))]
    SleightOfHand,
    Spellcraft,
    Stealth,
    Survival,
    Swim,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Use Magic Device"))]
    UseMagicDevice,
}

pub type Skills = BTreeMap<Skill, i16>;

#[cfg_attr(feature = "tavern", derive(sqlx::Type))]
// TODO: Revise code to allow for storing data (i16) with the enum instead of separate types
#[derive(Serialize, Deserialize, Display, PartialOrd, Ord, Hash, PartialEq, Eq)]
pub enum SaveThrow {
    Fortitude,
    Reflex,
    Will,
}

#[cfg_attr(feature = "tavern", derive(sqlx::Type))]
#[derive(Serialize, Deserialize, Display, PartialOrd, Ord, Hash, PartialEq, Eq)]
pub enum CharacterStat {
    Name,
    Race,
    Size,
    Height,
    Weight,
    Age,
    Gender,
    Alignment,
    Deity,
    Languages,
    Appearance,
}

pub type CharacterStats = BTreeMap<CharacterStat, i16>;

#[cfg_attr(feature = "tavern", derive(sqlx::Type))]
#[derive(Serialize, Deserialize, Display, PartialOrd, Ord, Hash, PartialEq, Eq)]
pub enum CombatStat {
    #[cfg_attr(feature = "tavern", sqlx(rename = "Melee Attack Bonus"))]
    MeleeAttackBonus,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Ranged Attack Bonus"))]
    RangedAttackBonus,
    CMB,
    CMD,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Armor Class"))]
    ArmorClass,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Touch AC"))]
    TouchAC,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Flat-Footed AC"))]
    FlatFootedAC,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Initiative Bonus"))]
    InitiativeBonus,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Damage Reduction"))]
    DamageReduction,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Spell Resistance"))]
    SpellResistance,
    Speed,
    Fortitude,
    Reflex,
    Will,
}

pub type CombatStats = BTreeMap<CombatStat, i16>;

#[cfg_attr(feature = "tavern", derive(sqlx::Type))]
#[derive(Serialize, Deserialize, Display)]
pub enum EquipmentSlot {
    Armor,
    Belts,
    Body,
    Chest,
    Eyes,
    Feet,
    Hands,
    Head,
    Headband,
    Neck,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Ring (left)"))]
    RingLeft,
    #[cfg_attr(feature = "tavern", sqlx(rename = "Ring (right)"))]
    RingRight,
    Shield,
    Shoulders,
    Wrist,
}

#[cfg_attr(feature = "tavern", derive(sqlx::Type))]
#[derive(Serialize, Deserialize, Display)]
pub enum DamageType {
    Bludgeoning,
    Slashing,
    Piercing,
    Energy,
    Acid,
    Fire,
    Electricity,
    Cold,
    Sonic,
    Positive,
    Negative,
    Nonlethal,
}

pub type Links = BTreeMap<String, String>;

pub mod character;
pub mod class;
pub mod effects;
pub mod feat;
pub mod item;
pub mod religion;
pub mod spell;
pub mod summary;