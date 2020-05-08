extern crate serde;
extern crate uuid;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
pub mod types {
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
}

pub mod entities {
    pub mod character {
        use serde::{Serialize,Deserialize};
        use crate::types;
        use uuid::Uuid;

        #[derive(Serialize, Deserialize)]
        pub struct Character {
            id: Uuid,
            char_id: Uuid,
            race_id: Uuid,
            deity_id: Uuid,

            name: String,
            age: u32,
            gender: types::Gender,
            alignment: types::Alignment,
            backstory: String,
            height: u32,
            weight: u32,
            size: types::Size,

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
        use uuid::Uuid;
        
        #[derive(Serialize, Deserialize)]
        pub struct race {
            race_id: Uuid,
            type_id: Uuid,
            subtype_id: Uuid,

            name: String,
            move_speed: u32,
            size: types::Size,
            languages: Vec<String>,

            //Possible alternative to Uuids
            //
            //race_type: raceType::RaceType,
            //race_subtype: raceSubtype::RaceSubtype,
        }
        pub mod raceType {
            use serde::{Serialize,Deserialize};
            use crate::types;
            use uuid::Uuid;

            #[derive(Serialize, Deserialize)]
            pub struct RaceType {
                type_id: Uuid,

                name: String,
                hit_die: String,
                bab_per_hit_die: f32,
            }
        }

        pub mod raceSubtype {
            use serde::{Serialize,Deserialize};
            use crate::types;
            use uuid::Uuid;

            #[derive(Serialize, Deserialize)]
            pub struct RaceSubtype {
                subtype_id: Uuid,
                name: String,
            }
        }
    }


    pub mod class {
        use serde::{Serialize,Deserialize};
        use crate::types;
        use uuid::Uuid;

        #[derive(Serialize, Deserialize)]
        pub struct Class {
            class_id: Uuid,

            name: String,
            hit_die: String,
            starting_wealth: String,
            bab_per_level: f32,
            skills_per_level: i32,
            skills_attr: types::Attribute,
        }

        pub mod subclass {
            use serde::{Serialize,Deserialize};
            use crate::types;
            use uuid::Uuid;
    
            #[derive(Serialize, Deserialize)]
            pub struct Subclass {
                subclass_id: Uuid,
                class_id: Uuid,

                caster_type: types::CasterType,
                casting_attr: types::Attribute,
            }
        }
    }


    pub mod feat {
        use serde::{Serialize,Deserialize};
        use crate::types;
        use uuid::Uuid;

        #[derive(Serialize, Deserialize)]
        pub struct Feat {
            feat_id: Uuid,

            short_description: String,
            long_description: String,
        }

        pub mod FeatReqUnits {
            use serde::{Serialize,Deserialize};
            use crate::types;
            use uuid::Uuid;

            pub struct SkillFeatUnit {
                skill_unit_id: Uuid,
                req_skil: types::Skill,
                ranks: u8,
            }

            pub struct AttributeFeatUnit {
                attr_unit_id: Uuid,
                req_attr: types::Attribute,
                score: u8
            }

            pub struct RequiredFeat {
                feat_id: Uuid,
                required_feat: Uuid
            }
        }
    }

    pub mod feature {
        use serde::{Serialize,Deserialize};
        use crate::types;
        use uuid::Uuid;

        #[derive(Serialize, Deserialize)]
        pub struct Feature {
            feature_id: Uuid,
            description: String,
        }
    }

    pub mod proficiencies {
        use serde::{Serialize,Deserialize};
        use crate::types;
        use uuid::Uuid;

        pub struct ClassWeaponsNotProficient {
            class_id: Uuid,
            weapon_classes: Vec<types::WeaponClass>
        }

        pub struct ClassArmorNotProficient {
            class_id: Uuid,
            armor_classes: Vec<types::ArmorClass>
        }
    }

    pub mod spell {
        use serde::{Serialize,Deserialize};
        use crate::types;
        use uuid::Uuid;

        #[derive(Serialize, Deserialize)]
        pub struct Spell {
            spell_id: Uuid,

            name: String,
            level: u32,
            school: types::MagicSchool,

            casting_time: u32,
            range: types::SpellRange,
            area: String,
            duration_per_level: u32,
            saving_throw: types::SaveThrow,
            spell_resistance: bool,
            description: String,
        }
    }

    pub mod domain {
        use serde::{Serialize,Deserialize};
        use crate::types;
        use uuid::Uuid;

        #[derive(Serialize, Deserialize)]
        pub struct Domain {
            domain_id: Uuid,

            name: String,
            description: String,
            power_description: String,
        }

        #[derive(Serialize, Deserialize)]
        pub struct Subdomain {
            domain_id: Uuid,

            name: String,
            description: String,
        }
    }

    pub mod deity {
        use serde::{Serialize,Deserialize};
        use crate::types;
        use uuid::Uuid;

        #[derive(Serialize, Deserialize)]
        pub struct Deity {
            deity_id: Uuid,

            name: String,
            description: String,
            favored_animals: Vec<String>,

        }
    }

    pub mod item {
        use serde::{Serialize,Deserialize};
        use crate::types;
        use uuid::Uuid;

        #[derive(Serialize, Deserialize)]
        pub struct Item {
            item_id: Uuid,

            name: String,
            description: String,
            cost: u32,
            weight: u32,
            equip_slot: types::EquipmentSlot,
        }

        pub mod bag {
            use serde::{Serialize,Deserialize};
            use crate::types;
            use uuid::Uuid;

            #[derive(Serialize, Deserialize)]
            pub struct Bag {
                bag_id: Uuid,
                char_id: Uuid,
                item_id: Uuid,

                capacity: u32,
            }
        }
            

        pub mod weapon {
            use serde::{Serialize,Deserialize};
            use crate::types;
            use uuid::Uuid;
            use std::ops::Range;

            #[derive(Serialize, Deserialize)]
            pub struct Weapon {
                item_id: Uuid,
                material_id: Uuid,

                weapon_range: Range<u32>,
                crit_range: Range<u32>,
                damage: Vec<String>,
                damage_type: Vec<types::DamageType>,
                weapon_type: types::WeaponClass,
            }
        }

        pub mod armor {
            use serde::{Serialize,Deserialize};
            use crate::types;
            use uuid::Uuid;

            #[derive(Serialize, Deserialize)]
            pub struct Armor {
                item_id: Uuid,
                material_id: Uuid,

                max_dex_bonus: u32,
                ac: u32,
                spell_failure: u32,
                check_penalty: u32,
                armor_type: types::ArmorClass,
            }
        }

        pub mod material {
            use serde::{Serialize,Deserialize};
            use crate::types;
            use uuid::Uuid;

            #[derive(Serialize, Deserialize)]
            pub struct Material {
                material_id: Uuid,

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
    use uuid::Uuid;

    #[derive(Serialize, Deserialize)]
    pub struct Effect {
        effect_id: Uuid,

        short_description: String,
        long_description: String,
    }

    pub mod effectUnits {
        use serde::{Serialize,Deserialize};
        use crate::types;
        use uuid::Uuid;

        pub struct AttributeUnit {
            attr_unit_id: Uuid,

            base_attr: types::Attribute,
            modifier: i32,
        }

        pub struct SkillUnit {
            skill_unit_id: Uuid,
            skill: types::Skill,
            modifier: i32,
        }

        pub struct CharacterUnit {
            char_unit_id: Uuid,

            character_stat: types::CharacterStat,
            modifier: i32
        }

        pub struct CombatUnit {
            combat_unit_id: Uuid,

            combat_stat: types::CombatStat,
            modifier: i32,
        }

        pub struct MiscUnit {
            misc_unit_id: Uuid,
            description: String,
        }
    }
}
