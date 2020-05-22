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

#[derive(Serialize, Deserialize, Display)]
pub enum Gender {
    Male,
    Female,
    Other,
}

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

#[derive(Serialize, Deserialize, Display)]
pub enum Alignment {
    LawfulGood,
    LawfulNeutral,
    LawfulEvil,
    TrueNeutral,
    NeutralEvil,
    ChaoticGood,
    ChaoticNeutral,
    ChaoticEvil,
}

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

#[derive(Serialize, Deserialize, Display, PartialOrd, Ord, Hash, PartialEq, Eq)]
pub enum Skill {
    Acrobatics,
    Appraise,
    Bluff,
    Climb,
    Craft,
    Diplomacy,
    DisableDevice,
    Disguise,
    EscapeArtist,
    Fly,
    HandleAnimal,
    Heal,
    Intimidate,
    KnowledgeArcana,
    KnowledgeDungeoneering,
    KnowledgeEngineering,
    KnowledgeGeography,
    KnowledgeHistory,
    KnowledgeLocal,
    KnowledgeNobility,
    KnowledgePlanes,
    KnowledgeReligion,
    Linguistics,
    Perception,
    Perform,
    Profession,
    Ride,
    SenseMotive,
    SleightOfHand,
    Spellcraft,
    Stealth,
    Survival,
    Swim,
    UseMagicDevice,
}

pub type Skills = BTreeMap<Skill, i16>;

#[derive(Serialize, Deserialize)]
pub enum SaveThrow {
    Fortitude(i16),
    Reflex(i16),
    Will(i16),
}

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

#[derive(Serialize, Deserialize, Display, PartialOrd, Ord, Hash, PartialEq, Eq)]
pub enum CombatStat {
    MeleeAttackBonus,
    RangedAttackBonus,
    CMB,
    CMD,
    AC,
    TouchAC,
    FlatFootedAC,
    InitiativeBonus,
    DamageReduction,
    SpellResistance,
    Speed,
    Fortitude,
    Reflex,
    Will,
}

pub type CombatStats = BTreeMap<CombatStat, i16>;

#[derive(Serialize, Deserialize, Display)]
pub enum EquipmentSlot {
    NoSlot,
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
    RingLeft,
    RingRight,
    Shield,
    Shoulders,
    Wrist,
}

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

//REST API link struct
#[derive(Serialize, Deserialize, Clone)]
pub struct Link {
    rel: String,
    url: String,
}

pub type Links = BTreeMap<String, Link>;

pub mod character;
pub mod class;
pub mod effects;
pub mod feat;
pub mod item;
pub mod religion;
pub mod spell;
pub mod summary;
