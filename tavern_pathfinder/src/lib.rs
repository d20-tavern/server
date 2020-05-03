use serde::{Serialize, Deserialize};
use uuid::Uuid;
use byte;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
mod types {
    enum Gender {
        Male,
        Female,
        Other,
    }

    enum Size {
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

    enum Alignment {
        LawfulGood,
        LawfulNeutral,
        LawfulEvil,
        TrueNeutral,
        NeutralEvil,
        ChaoticGood,
        ChaoticNeutral,
        ChaoticEvil,
    }

    enum Attribute {
        Strength,
        Dexterity,
        Constitution,
        Intelligence,
        Wisdom,
        Charisma,
    }

    enum Skill {
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

    enum SaveThrow {
        Fortitude,
        Reflex,
        Will,
    }

    enum CharacterStat {
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

    enum CombatStat {
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

    enum CasterType {
        Spontaneous,
        Prepared,
    }

    enum ComponentType {
        Somatic,
        Material,
        Verbal,
    }

    enum MagicSchool {    
    	Abjuration, 
    	Conjuration, 
    	Divination, 
    	Enchantment, 
    	Evocation, 
    	Illusion, 
    	Necromancy, 
	    Transmutation,
    }

    enum SpellRange {
        Personal,
        Touch,
        Close,
        Medium,
        Long,
        Unlimited,
    }

    enum WeaponClass {
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

    enum ArmorClass {
        Light,
        Medium,
        Heavy,
    }

    enum EquipmentSlot {
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

    enum DamageType {
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
);

}

mod entities {
    mod user {
        #[derive(Serialize, Deserialize, Debug)]
        struct user { 
            id: Uuid,

            email: String,
            username: String,
            is_admin: bool,


        }
    }
}
