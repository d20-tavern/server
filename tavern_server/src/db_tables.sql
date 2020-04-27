--enum types
CREATE TYPE IF NOT EXISTS Gender AS ENUM (
	'Male', 
	'Female', 
	'Other'
);

CREATE TYPE IF NOT EXISTS Size AS ENUM (
	'Fine', 
	'Diminutive', 
	'Tiny', 
	'Small', 
	'Medium', 
	'Large', 
	'Huge', 
	'Gargantuan', 
	'Colossal'
);

CREATE TYPE IF NOT EXISTS Alignment AS ENUM (
	'Lawful Good', 
	'Lawful Neutral', 
	'Lawful Evil', 
	'Neutral Good', 
	'True Neutral', 
	'Neutral Evil', 
	'Chaotic Good', 
	'Chaotic Neutral', 
	'Chaotic Evil'
);

CREATE TYPE IF NOT EXISTS Attribute AS ENUM (
	'Strength', 
	'Dexterity', 
	'Constitution', 
	'Intelligence', 
	'Wisdom', 
	'Charisma'
);

CREATE TYPE IF NOT EXISTS Skill AS ENUM (
	'Acrobatics', 
	'Appraise', 
	'Bluff', 
	'Climb', 
	'Craft', 
	'Diplomacy', 
	'Disable Device', 
	'Disguise', 
	'Escape Artist', 
	'Fly', 
	'Handle Animal', 
	'Heal', 
	'Intimidate', 
	'Knowledge (arcana)', 
	'Knowledge (dungeoneering)', 
	'Knowledge (engineering)', 
	'Knowledge (geography)', 
	'Knowledge (history)', 
	'Knowledge (local)', 
	'Knowledge (nobility)', 
	'Knowledge (planes)', 
	'Knowledge (religion)', 
	'Linguistics', 
	'Perception', 
	'Perform', 
	'Profession', 
	'Ride', 
	'Sense Motive', 
	'Sleight of Hand', 
	'Spellcraft', 
	'Stealth', 
	'Survival', 
	'Swim', 
	'Use Magic Device'
);

CREATE TYPE IF NOT EXISTS SaveThrow AS ENUM (
	'Fortitude', 
	'Reflex',
	'Will'
);

CREATE TYPE IF NOT EXISTS CharacterStat AS ENUM (
	'Name',
	'Race',
	'Size',
	'Height',
	'Weight',
	'Age',
	'Gender',
	'Alignment',
	'Deity',
	'Languages',
	'Appearance'
);

CREATE TYPE IF NOT EXISTS CombatStat AS ENUM (
	'Melee Attack Bonus',
	'Ranged Attack Bonus',
	'CMB',
	'CMD',
	'Armor Class',
	'Touch AC',
	'Flat-Footed AC',
	'Initiative Bonus',
	'Damage Reduction',
	'Spell Resistance',
	'Speed',
	'Fortitude Save',
	'Reflex Save',
	'Will Save'
);

CREATE TYPE IF NOT EXISTS RaceType AS ENUM (
	'Abberation', 
	'Animal', 
	'Construct', 
	'Dragon', 
	'Fey', 
	'Humanoid', 
	'Magical Beast', 
	'Monstrous Humanoid', 
	'Ooze', 
	'Outsider', 
	'Plant', 
	'Undead', 
	'Vermin'
);

CREATE TYPE IF NOT EXISTS CasterType AS ENUM (
	'Spontaneous', 
	'Prepared'
);

CREATE TYPE IF NOT EXISTS ComponentType AS ENUM (
	'Somatic',
	'Material',
	'Verbal'
);

CREATE TYPE IF NOT EXISTS MagicSchool AS ENUM (
	'Abjuration', 
	'Conjuration', 
	'Divination', 
	'Enchantment', 
	'Evocation', 
	'Illusion', 
	'Necromancy', 
	'Transmutation'
);

CREATE TYPE IF NOT EXISTS SpellRange AS ENUM (
	'Personal',
	'Touch',
	'Close',
	'Medium',
	'Long',
	'Unlimited'
);

CREATE TYPE IF NOT EXISTS WeaponClass AS ENUM (
	'Axes', 
	'Heavy Blades', 
	'Light Blades', 
	'Bows', 
	'Close', 
	'Crossbows', 
	'Double', 
	'Firearms', 
	'Flails', 
	'Hammers', 
	'Monk', 
	'Natural', 
	'Polearms', 
	'Siege Engines', 
	'Spears', 
	'Thrown', 
	'Tribal'
);

CREATE TYPE IF NOT EXISTS ArmorClass AS ENUM (
	'Light', 
	'Medium', 
	'Heavy'
);

CREATE TYPE IF NOT EXISTS EquipmentSlot AS ENUM (
	'None',
	'Armor', 
	'Belts', 
	'Body', 
	'Chest', 
	'Eyes', 
	'Feet', 
	'Hands', 
	'Head', 
	'Headband', 
	'Neck', 
	'Ring (left)', 
	'Ring (right)', 
	'Shield', 
	'Shoulders', 
	'Wrist'
);

CREATE TYPE IF NOT EXISTS DamageType AS ENUM (
	'Bludgeoning', 
	'Slashing', 
	'Piercing', 
	'Energy', 
	'Acid', 
	'Fire', 
	'Electricity', 
	'Cold', 
	'Sonic', 
	'Positive', 
	'Negative', 
	'Nonlethal'
);

CREATE TYPE IF NOT EXISTS Material AS ENUM (
	'Nonspecific', 
	'Abysium', 
	'Adamantine', 
	'Angelskin', 
	'Aszite', 
	'Magic Bridge Basalt', 
	'Blight Quartz', 
	'Bone', 
	'Bronze', 
	'Bulette', 
	'Coral', 
	'Clamshell', 
	'Chitin', 
	'Crypstone', 
	'Blood Crystal',
	'Darkleaf Cloth',
	'Darkwood',
	'Djezet',
	'Dragonhide',
	'Dragonskin',
	'Druchite',
	'Eel Hide',
	'Elysian Bronze',
	'Gold',
	'Greenwood',
	'Griffon Mane',
	'Horacalcum',
	'Inubrix',
	'Cold Iron',
	'Mindglass',
	'Mithral',
	'Noqual',
	'Obsidian',
	'Serpentstone',
	'Siccatite',
	'Alchemical Silver',
	'Silversheen',
	'Skymetal Alloy',
	'Spiresteel',
	'Fire-forged Steel',
	'Frost-Forged Steel',
	'Living Steel',
	'Singing Steel',
	'Stone',
	'Sunsilver',
	'Viridium',
	'Voidglass',
	'Whipwood',
	'Wyroot'
);

--table declarations
--user creds table - also used for hashing info
CREATE TABLE IF NOT EXISTS Users (
	id     			UUID PRIMARY KEY,

	email			TEXT,
        username       		TEXT,
	nickname		TEXT,
        is_admin    		BOOL,

        pass_hash   		BYTEA,
        salt        		BYTEA,

        time_cost   		INT,
        memory      		INT,
        threads     		INT
);

--Character overview table
CREATE TABLE IF NOT EXISTS Characters (
        char_id         	UUID PRIMARY KEY,

	id			UUID REFERENCES Users,
        race_id         	UUID REFERENCES Races,
        deity_id        	UUID REFERENCES Deities,

        name            	TEXT,
        age             	INT,
        gender          	Gender,
        alignment       	Alignment,
        backstory       	TEXT,
        height          	INT,
        weight          	INT,
        size            	Size,

        strength        	INT,
        dexterity       	INT,
        constitution    	INT,
        intelligence    	INT,
        wisdom          	INT,
        charisma        	INT,

        max_hp          	INT,
        damage          	INT,
        nonlethal       	INT,

        copper          	INT,
        silver          	INT,
        gold            	INT,
        platinum        	INT
);

--Race tables
CREATE TABLE IF NOT EXISTS Races (
	race_id 		UUID PRIMARY KEY,

	name			TEXT,
	move_speed		INT,
	size			Size,
	languages		TEXT,
	race_type		RaceType
);

--Class tables
CREATE TABLE IF NOT EXISTS Classes (
	class_id		UUID PRIMARY KEY,

	name			TEXT,
	hit_die			TEXT,
	starting_wealth		TEXT,
	bab_per_level		REAL,
	skills_per_level	INT,
	skills_attr		Attribute
);

CREATE TABLE IF NOT EXISTS Subclasses (
	subclass_id		UUID PRIMARY KEY,
	class_id		UUID REFERENCES Classes,

	caster_type		CasterType,
	casting_attr		Attribute
);

CREATE TABLE IF NOT EXISTS characterSubclasses (
	char_id			UUID REFERENCES Characters,
	subclass_id		UUID REFERENCES Subclasses,

	levels_taken		INT,
	hp_taken		INT,
	skills_taken		INT
);

--Feats tables
CREATE TABLE IF NOT EXISTS Feats (
	feat_id			UUID PRIMARY KEY,

	short_description	TEXT,
	long_description	TEXT
);

CREATE TABLE IF NOT EXISTS SkillReqUnits (
	feat_id			UUID REFERENCES Feats,
	skill_unit_id		UUID[] REFERENCES SkillFeatUnits
);

CREATE TABLE IF NOT EXISTS SkillFeatUnits (
	skill_unit_id		UUID PRIMARY KEY,
	req_skill		Skill,
	ranks			INT
);

CREATE TABLE IF NOT EXISTS AttributeReqUnits (
	feat_id			UUID REFERENCES Feats,
	attr_unit_id		UUID[] REFERENCES AttributeFeatUnits
);

CREATE TABLE IF NOT EXISTS AttributeFeatUnits (
	attr_unit_id		UUID PRIMARY KEY,
	req_attr		Attribute,
	score			INT
);

--this table creates a many to many realationship internally between feats.
--any given feat can require a number of other feats to be taken,
--and a given feat can be required by any number of other feats.
--this table accomplishes that.
CREATE TABLE IF NOT EXISTS FeatRequirements (
	feat_id			UUID REFERENCES Feats,
	required_feat		UUID[] REFERENCES Feats
);

CREATE TABLE IF NOT EXISTS CharacterFeats (
	char_id			UUID REFERENCES Characters,
	feat_id			UUID[] REFERENCES Feats
);

CREATE TABLE IF NOT EXISTS RacialFeats (
	race_id			UUID REFERENCES Races,
	feat_id			UUID[] REFERENCES Feats
);

CREATE TABLE IF NOT EXISTS ClassFeats (
	class_id		UUID REFERENCES Classes,
	feat_id			UUID[] REFERENCES Feats
);

--Features tables
CREATE TABLE IF NOT EXISTS Features (
	feature_id		UUID PRIMARY KEY,
	description		TEXT
);

CREATE TABLE IF NOT EXISTS ClassFeatures (
	class_id		UUID REFERENCES Feats,
	feature_id		UUID REFERENCES Features,
	is_default		BOOLEAN
);

CREATE TABLE IF NOT EXISTS CharacterFeatures (
	char_id			UUID REFERENCES Characters,
	feature_id		UUID REFERENCES Features
);

CREATE TABLE IF NOT EXISTS SubclassFeatures (
	subclass_id		UUID REFERENCES Subclasses,
	feature_id		UUID REFERENCES Features
);

--class proficiencies tables
CREATE TABLE IF NOT EXISTS ClassWeaponsNotProficient (
	class_id		UUID REFERENCES Classes,
	weapon_classes		WeaponClass[]
);

CREATE TABLE IF NOT EXISTS ClassArmorsNotProficient (
	class_id		UUID REFERENCES Classes,
	armor_classes		ArmorClass[]
);

--Spells tables
CREATE TABLE IF NOT EXISTS Spells (
	spell_id		UUID PRIMARY KEY,

	name			TEXT,
	level			INT,
	school			MagicSchool,

	casting_time		INT,
	range			SpellRange,
	area			TEXT,
	duration_per_level	INT,
	saving_throw		SaveThrow,
	spell_resistance	BOOLEAN,
	description		TEXT
);

CREATE TABLE IF NOT EXISTS CharacterSpells (
	char_id			UUID REFERENCES Characters,
	spell_id		UUID REFERENCES Spells,
	casts			INT
);

CREATE TABLE IF NOT EXISTS SubclassSpells (
	subclass_id		UUID REFERENCES Subclasses,
	spell_id		UUID REFERENCES Spells,
	casts			INT,
	req_level		INT
);

CREATE TABLE IF NOT EXISTS DomainSpells (
	domain_id		UUID REFERENCES Domains,
	spell_id		UUID REFERENCES Spells,
	casts			INT
);

CREATE TABLE IF NOT EXISTS SpellComponents (
	spell_id		UUID REFERENCES Spells,
	item_id			UUID REFERENCES Items,

	amount			INT
	component_type		ComponentType
);

--Domain tables
CREATE TABLE IF NOT EXISTS Domains (
	domain_id		UUID PRIMARY KEY,

	name			TEXT,
	description		TEXT,
	power_description	TEXT
);

CREATE TABLE IF NOT EXISTS Subdomains (
	domain_id		UUID REFERENCES Domains,

	name			TEXT,
	description		TEXT
);

--Deity tables
CREATE TABLE IF NOT EXISTS Deities (
	deity_id		UUID PRIMARY KEY,
	
	name			TEXT,
	description		TEXT,
	favored_animals		TEXT
);

CREATE TABLE IF NOT EXISTS DeityDomains (
	deity_id		UUID REFERENCES Deities,
	domain_id		UUID REFERENCES Domains
);

CREATE TABLE IF NOT EXISTS DeityWeapons (
	deity_id		UUID REFERENCES Deities,
	item_id			UUID REFERENCES Items
);

--Item tables
CREATE TABLE IF NOT EXISTS PathfinderItems (
	item_id			UUID PRIMARY KEY,

	cost			INT,
	description		TEXT,
	name			TEXT,
	weight			INT,
	equip_slot		EquipmentSlot
);

CREATE TABLE IF NOT EXISTS Weapons (
	item_id			UUID REFERENCES Items,

	weapon_range		INT4RANGE,
	crit_range		INT4RANGE,
	damage			TEXT[],
	damage_type		DamageType[],
	weapon_type		WeaponClass,
	material		Material
);

CREATE TABLE IF NOT EXISTS Armor (
	item_id			UUID REFERENCES Items,
	
	max_dex_bonus		INT,
	ac			INT,
	spell_failure		INT,
	check_penalty		INT,
	material		Material,
	armor_type		ArmorClass
);

CREATE TABLE IF NOT EXISTS ItemsInBags (
	item_id			UUID REFERENCES Items,
	bag_id			UUID REFERENCES Bags
);

CREATE TABLE IF NOT EXISTS Bags (
	bag_id			UUID PRIMARY KEY,
	char_id			UUID REFERENCES Characters,
	item_id			UUID REFERENCES Items,

	capacity		INT
);

CREATE TABLE IF NOT EXISTS CharacterEquipment (
	char_id			UUID REFERENCES Characters,
	item_id			UUID REFERENCES Items
);

--Effects tables
CREATE TABLE IF NOT EXISTS Effects (
	effect_id		UUID PRIMARY KEY,
	short_description	TEXT,
	long_description	TEXT
);

CREATE TABLE IF NOT EXISTS RaceEffects (
	race_id			UUID REFERENCES Races,
	effect_id		UUID REFERENCES Effects
);

CREATE TABLE IF NOT EXISTS ClassEffects (
	class_id		UUID REFERENCES Classes,
	effect_id		UUID REFERENCES Effects
);

CREATE TABLE IF NOT EXISTS ItemEffects (
	item_id			UUID REFERENCES Items,
	effect_id		UUID REFERENCES Effects,
	is_permanent		BOOLEAN
);

CREATE TABLE IF NOT EXISTS SpellEffects (
	spell_id		UUID REFERENCES Spells,
	effect_id		UUID REFERENCES Effects
);

CREATE TABLE IF NOT EXISTS FeatEffects (
	feat_id			UUID REFERENCES Feats,
	effect_id		UUID REFERENCES Effects
);

CREATE TABLE IF NOT EXISTS FeatureEffects (
	feature_id		UUID REFERENCES Features,
	effect_id		UUID REFERENCES Effects
);

CREATE TABLE IF NOT EXISTS DomainEffects (
	domain_id		UUID REFERENCES Domains,
	effect_id		UUID REFERENCES Effects
);

CREATE TABLE IF NOT EXISTS AttributeEffectUnits (
	effect_id		UUID REFERENCES Effects,
	attr_unit_id		UUID REFERENCES AttributeUnits
);

CREATE TABLE IF NOT EXISTS AttributeUnits (
	attr_unit_id		UUID PRIMARY KEY,
	base_attr		Attribute,
	modifier		INT
);

CREATE TABLE IF NOT EXISTS SkillEffectUnits (
	effect_id		UUID REFERENCES Effects,
	skill_unit_id		UUID REFERENCES SkillUnits
);

CREATE TABLE IF NOT EXISTS SkillUnits (
	skill_unit_id		UUID PRIMARY KEY,
	skill			Skill,
	modifier		INT
);

CREATE TABLE IF NOT EXISTS CharacterEffectUnits (
	effect_id		UUID REFERENCES Effects,
	char_unit_id		UUID REFERENCES CharacterUnits
);

CREATE TABLE IF NOT EXISTS CharacterUnits (
	char_unit_id		UUID PRIMARY KEY,
	character_stat		CharacterStat,
	modifier		INT
);

CREATE TABLE IF NOT EXISTS CombatEffectUnits (
	effect_id		UUID REFERENCES Effects,
	combat_unit_id		UUID REFERENCES CombatUnits
);

CREATE TABLE IF NOT EXISTS CombatUnits (
	combat_unit_id		UUID PRIMARY KEY,
	combat_stat		CombatStat,
	modifier		INT
);

CREATE TABLE IF NOT EXISTS MiscEffectUnits (
	effect_id		UUID REFERENCES Effects,
	misc_unit_id		UUID REFERENCES MiscUnits
);

CREATE TABLE IF NOT EXISTS MiscUnits (
	misc_unit_id		UUID PRIMARY KEY,
	description		TEXT
);
