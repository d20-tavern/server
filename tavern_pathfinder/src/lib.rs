extern crate serde;

use serde::{Serialize, Deserialize};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
pub mod types {
    pub enum Gender {
        Male,
        Female,
        Other,
    }

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

    pub enum Attribute {
        Strength,
        Dexterity,
        Constitution,
        Intelligence,
        Wisdom,
        Charisma,
    }

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

    pub enum SaveThrow {
        Fortitude,
        Reflex,
        Will,
    }

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

    pub enum CasterType {
        Spontaneous,
        Prepared,
    }

    pub enum ComponentType {
        Somatic,
        Material,
        Verbal,
    }

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

    pub enum SpellRange {
        Personal,
        Touch,
        Close,
        Medium,
        Long,
        Unlimited,
    }

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

    pub enum ArmorClass {
        Light,
        Medium,
        Heavy,
    }

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
}

pub mod entities {
    pub mod character {
        use serde::{Serialize,Deserialize};
        use super::super::types;

        pub struct Character {
            name: String,
            age: u32,
            //gender: types::Gender,
            //alignment: types::Alignment,
            backstory: String,
            height: u32,
            weight: u32,
            //size: types::Size,

            strength: u8,
            dexterity: u8,
            constitution: u8,
            intelligence: u8,
            wisdom: u8,

            max_hp: u32,
            damage: u32,
            nonlethal: u32,

            copper: u32,
            silver: u32,
            gold: u32,
            platinum: u32,
        }
    }

    pub mod race {
        use serde::{Serialize,Deserialize};
        use super::super::types;
        
        pub struct race {
            name: String,
            move_speed: u32,
            //size: types::Size,
            languages: [String, 256],
        }
    }

    pub mod raceType {
        use serde::{Serialize,Deserialize};
        use super::super::types;
        
        pub struct raceType {
            name: String,
            hit_die: String,
            bab_per_hit_die: f32,
        }
    }

    pub mod raceSubtype {
        use serde::{Serialize,Deserialize};
        use super::super::types;

        pub struct raceSubtype {
            name: String,
        }
    }
}
