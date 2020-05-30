use serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::collections::BTreeMap;
use std::hash::Hash;
use tavern_derive::Display;
#[cfg(feature = "tavern")]
use diesel_derive_enum::DbEnum;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[cfg_attr(feature = "tavern", derive(DbEnum))]
#[derive(Serialize, Deserialize, Display)]
pub enum Gender {
    Male,
    Female,
    Other,
}

#[cfg_attr(feature = "tavern", derive(DbEnum))]
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

#[cfg_attr(feature = "tavern", derive(DbEnum))]
#[derive(Serialize, Deserialize, Display)]
pub enum Alignment {
    LawfulGood,
    LawfulNeutral,
    LawfulEvil,
    NeutralGood,
    TrueNeutral,
    NeutralEvil,
    ChaoticGood,
    ChaoticNeutral,
    ChaoticEvil,
}

#[cfg_attr(feature = "tavern", derive(DbEnum))]
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

#[cfg_attr(feature = "tavern", derive(DbEnum))]
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

#[cfg_attr(feature = "tavern", derive(DbEnum))]
// TODO: Revise code to allow for storing data (i16) with the enum instead of separate types
#[derive(Serialize, Deserialize, Display, PartialOrd, Ord, Hash, PartialEq, Eq)]
pub enum SaveThrow {
    Fortitude,
    Reflex,
    Will,
}

#[cfg_attr(feature = "tavern", derive(DbEnum))]
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

#[cfg_attr(feature = "tavern", derive(DbEnum))]
#[derive(Serialize, Deserialize, Display, PartialOrd, Ord, Hash, PartialEq, Eq)]
pub enum CombatStat {
    MeleeAttackBonus,
    RangedAttackBonus,
    CMB,
    CMD,
    ArmorClass,
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

#[cfg_attr(feature = "tavern", derive(DbEnum))]
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
    RingLeft,
    RingRight,
    Shield,
    Shoulders,
    Wrist,
}

#[cfg_attr(feature = "tavern", derive(DbEnum))]
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