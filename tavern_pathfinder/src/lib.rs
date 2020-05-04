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
        use crate::types;

        #[derive(Serialize, Deserialize)]
        pub struct Character {
            name: String,
            age: u32,
            //gender: types::Gender,
            //alignment: types::Alignment,
            backstory: String,
            height: u32,
            weight: u32,
            //size: types::Size,

            strength: u32,
            dexterity: u32,
            constitution: u32,
            intelligence: u32,
            wisdom: u32,

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
        use crate::types;
        
        #[derive(Serialize, Deserialize)]
        pub struct race {
            name: String,
            move_speed: u32,
            //size: types::Size,
            languages: Vec<String>,

            race_type: raceType::RaceType,
            race_subtype: raceSubtype::RaceSubtype,
        }
        pub mod raceType {
            use serde::{Serialize,Deserialize};
            use crate::types;
        
            #[derive(Serialize, Deserialize)]
            pub struct RaceType {
                name: String,
                hit_die: String,
                bab_per_hit_die: f32,
            }
        }

        pub mod raceSubtype {
            use serde::{Serialize,Deserialize};
            use crate::types;

            #[derive(Serialize, Deserialize)]
            pub struct RaceSubtype {
                name: String,
            }
        }
    }


    pub mod class {
        use serde::{Serialize,Deserialize};
        use crate::types;

        #[derive(Serialize, Deserialize)]
        pub struct Class {
            name: String,
            hit_die: String,
            starting_wealth: String,
            bab_per_level: f32,
            skills_per_level: i32,
            //skills_attr: types:Attribute,

            //subclass: super::subclass::Subclass,
        }

        pub mod subclass {
            use serde::{Serialize,Deserialize};
            use crate::types;
    
            #[derive(Serialize, Deserialize)]
            pub struct Subclass {
                //caster_type: types::CasterType,
                //casting_attr: types::Attribute,
            }
        }
    }


    pub mod feat {
        use serde::{Serialize,Deserialize};
        use crate::types;

        #[derive(Serialize, Deserialize)]
        pub struct Feat {
            short_description: String,
            long_description: String,
        }
    }
    pub mod feature {
        use serde::{Serialize,Deserialize};
        use crate::types;

        #[derive(Serialize, Deserialize)]
        pub struct Feature {
            description: String,
        }
    }

    pub mod spell {
        use serde::{Serialize,Deserialize};
        use crate::types;

        #[derive(Serialize, Deserialize)]
        pub struct Spell {
            name: String,
            level: u32,
            //school: types::MagicSchool,

            casting_time: u32,
            //range: types::Range,
            area: String,
            duration_per_level: u32,
            //saving_throw: types::SaveThrow,
            spell_resistance: bool,
            description: String,
        }
    }

    pub mod domain {
        use serde::{Serialize,Deserialize};
        use crate::types;

        #[derive(Serialize, Deserialize)]
        pub struct Domain {
            name: String,
            description: String,
            power_description: String,
        }
    }

    pub mod deity {
        use serde::{Serialize,Deserialize};
        use crate::types;

        #[derive(Serialize, Deserialize)]
        pub struct Deity {
            name: String,
            description: String,
            favored_animals: Vec<String>,

        }
    }

    pub mod item {
        use serde::{Serialize,Deserialize};
        use crate::types;

        #[derive(Serialize, Deserialize)]
        pub struct Item {
            name: String,
            description: String,
            cost: u32,
            weight: u32,
            //equip_slot: types::EquipmentSlot,
        }

        pub mod weapon {
            use serde::{Serialize,Deserialize};
            use crate::types;
            use std::ops::Range;

            #[derive(Serialize, Deserialize)]
            pub struct Weapon {
                weapon_range: Range<u32>,
                crit_range: Range<u32>,
                damage: Vec<String>,
                //damage_type: Vec<types::DamageType>,
                //weapon_type: WeaponClass,
            }
        }

        pub mod armor {
            use serde::{Serialize,Deserialize};
            use crate::types;

            #[derive(Serialize, Deserialize)]
            pub struct Armor {
                max_dex_bonus: u32,
                ac: u32,
                spell_failure: u32,
                check_penalty: u32,
                //armor_type: types::ArmorClass,
            }
        }

        pub mod material {
            use serde::{Serialize,Deserialize};
            use crate::types;

            #[derive(Serialize, Deserialize)]
            pub struct Material {
                name: String,
                description: String,

                hp_per_inch: u32,
                hardness: u32,
            }
        }
    }
}

pub mod effects {
    use serde::{Serialize,Deserialize};
    use crate::types;

    #[derive(Serialize, Deserialize)]
    pub struct Effect {
        short_description: String,
        long_description: String,
    }
}
