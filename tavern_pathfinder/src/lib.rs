extern crate serde;
extern crate uuid;

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}

use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
pub enum Gender {
	Male,
	Female,
	Other,
}

#[derive(Serialize,Deserialize)]
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

#[derive(Serialize,Deserialize)]
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

#[derive(Serialize,Deserialize)]
pub enum Attribute {
	Strength,
	Dexterity,
	Constitution,
	Intelligence,
	Wisdom,
	Charisma,
}

#[derive(Serialize,Deserialize)]
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

#[derive(Serialize,Deserialize)]
pub enum SaveThrow {
	Fortitude,
	Reflex,
	Will,
}	

#[derive(Serialize,Deserialize)]
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

#[derive(Serialize,Deserialize)]
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
	FortitudeSave,
	ReflexSave,
	WillSave,
}

#[derive(Serialize,Deserialize)]
pub enum CasterType {
	Spontaneous,
	Prepared,
}

#[derive(Serialize,Deserialize)]
pub enum ComponentType {
	Somatic,
	Material,
	Verbal,
}

#[derive(Serialize,Deserialize)]
pub enum MagicSchool {	
	Abjuration, 
	Conjuration, 
	Divination, 
	Enchantment, 
	Evocation, 
	Illusion, 
	Necromancy, 
	Transmutation,
}

#[derive(Serialize,Deserialize)]
pub enum SpellRange {
	Personal,
	Touch,
	Close,
	Medium,
	Long,
	Unlimited,
}

#[derive(Serialize,Deserialize)]
pub enum WeaponClass {
	Axes, 
	HeavyBlades, 
	LightBlades, 
	Bows, 
	Close, 
	Crossbows, 
	Double, 
	Firearms, 
	Flails, 
	Hammers, 
	Monk, 
	Natural, 
	Polearms, 
	SiegeEngines, 
	Spears,
	Thrown,
	Tribal,
}

#[derive(Serialize,Deserialize)]
pub enum ArmorClass {
	Light,
	Medium,
	Heavy,
}

#[derive(Serialize,Deserialize)]
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

#[derive(Serialize,Deserialize)]
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
pub struct Link {
    rel: String,
    url: String/Url,
}

pub mod character;
pub mod class;
pub mod feat;
pub mod spell;
pub mod religion;
pub mod item;
pub mod effects;
