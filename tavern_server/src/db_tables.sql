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

--table declarations
--user creds table - also used for hashing info
CREATE TABLE IF NOT EXISTS Users (
	id     			UUID PRIMARY KEY,

	email			TEXT 
		NOT NULL,
        username       		TEXT 
		NOT NULL,
        is_admin    		BOOLEAN
		DEFAULT false,

        pass_hash   		BYTEA 
		NOT NULL,
        salt        		BYTEA 
		NOT NULL,

        time_cost   		INT 
		NOT NULL
		CHECK (time_cost > 0),
        memory      		INT 
		NOT NULL
		CHECK (memory > 0),
        threads     		INT 
		NOT NULL
		CHECK (threads > 0)
);

--Character overview table
CREATE TABLE IF NOT EXISTS Characters (
        char_id         	UUID PRIMARY KEY,

	id			UUID REFERENCES Users 
		NOT NULL,
        race_id         	UUID REFERENCES Races 
		NOT NULL,
        deity_id        	UUID REFERENCES Deities 
		NOT NULL,

        name            	TEXT 
		NOT NULL,
        age             	INT 
		NOT NULL 
		CHECK (age > 0),
        gender          	Gender
		NOT NULL,
        alignment       	Alignment 
		NOT NULL,
        backstory       	TEXT,
        height          	INT
		CHECK (height > 0),
        weight          	INT
		CHECK (weight > 0),
        size            	Size 
		NOT NULL,

        strength        	INT 
		NOT NULL 
		CHECK (strength >= 0 AND strength <= 20),
        dexterity       	INT 
		NOT NULL 
		CHECK (dexterity >= 0 AND dexterity <= 20),
        constitution    	INT 
		NOT NULL
		CHECK (constitution >= 0 AND constitution <= 20),
        intelligence    	INT 
		NOT NULL
		CHECK (intelligence >= 0 AND intelligence <= 20),
        wisdom          	INT
		NOT NULL
		CHECK (wisdom >= 0 AND wisdom <= 20),
        charisma        	INT
		NOT NULL
		CHECK (charisma >= 0 AND charisma <= 20),

        max_hp          	INT
		NOT NULL
		CHECK (max_hp >= 0),
        damage          	INT
		NOT NULL
		CHECK (damage >= 0),
        nonlethal       	INT
		NOT NULL
		CHECK (nonlethal >= 0),

        copper          	INT
		NOT NULL
		CHECK (copper >= 0),
        silver          	INT
		NOT NULL
		CHECK (silver >= 0),
        gold            	INT
		NOT NULL
		CHECK (gold >= 0),
        platinum        	INT
		NOT NULL
		CHECK (platinum >= 0)
);

--Race tables
CREATE TABLE IF NOT EXISTS Races (
	race_id 		UUID PRIMARY KEY,
	type_id			UUID REFERENCES RaceTypes
		NOT NULL,
	subtype_id		UUID REFERENCES RaceSubtypes,

	name			TEXT
		NOT NULL,
	move_speed		INT
		NOT NULL
		CHECK (move_speed > 0),
	size			Size
		NOT NULL,
	languages		TEXT[]
);

CREATE TABLE IF NOT EXISTS RaceTypes (
	type_id			UUID PRIMARY KEY,

	name			TEXT
		NOT NULL,
	hit_die			TEXT
		NOT NULL,
	bab_per_hit_die		REAL
		NOT NULL
);

CREATE TABLE IF NOT EXISTS RaceSubtypes (
	subtype_id		UUID PRIMARY KEY,

	name			TEXT
		NOT NULL
);
	
--Class tables
CREATE TABLE IF NOT EXISTS Classes (
	class_id		UUID PRIMARY KEY,

	name			TEXT
		NOT NULL,
	hit_die			TEXT
		NOT NULL,
	starting_wealth		TEXT
		NOT NULL,
	bab_per_level		REAL
		NOT NULL,
	skills_per_level	INT
		NOT NULL,
	skills_attr		Attribute
		NOT NULL
);

CREATE TABLE IF NOT EXISTS Subclasses (
	subclass_id		UUID PRIMARY KEY,
	class_id		UUID REFERENCES Classes
		NOT NULL,

	caster_type		CasterType,
	casting_attr		Attribute
);

CREATE TABLE IF NOT EXISTS characterSubclasses (
	char_id			UUID REFERENCES Characters
		NOT NULL,
	subclass_id		UUID REFERENCES Subclasses
		NOT NULL,

	levels_taken		INT
		NOT NULL
		CHECK (levels_taken > 0),
	hp_taken		INT
		NOT NULL
		CHECK (hp_taken >= 0),
	skills_taken		INT
		NOT NULL
		CHECK (skills_taken >= 0)
);

--Feats tables
CREATE TABLE IF NOT EXISTS Feats (
	feat_id			UUID PRIMARY KEY,

	short_description	TEXT
		NOT NULL,
	long_description	TEXT
);

CREATE TABLE IF NOT EXISTS SkillReqUnits (
	feat_id			UUID REFERENCES Feats
		NOT NULL,
	skill_unit_id		UUID REFERENCES SkillFeatUnits
		NOT NULL
);

CREATE TABLE IF NOT EXISTS SkillFeatUnits (
	skill_unit_id		UUID PRIMARY KEY,
	req_skill		Skill
		NOT NULL,
	ranks			INT
		NOT NULL
);

CREATE TABLE IF NOT EXISTS AttributeReqUnits (
	feat_id			UUID REFERENCES Feats
		NOT NULL,
	attr_unit_id		UUID REFERENCES AttributeFeatUnits
		NOT NULL
);

CREATE TABLE IF NOT EXISTS AttributeFeatUnits (
	attr_unit_id		UUID PRIMARY KEY,
	req_attr		Attribute
		NOT NULL,
	score			INT
		NOT NULL
);

--this table creates a many to many realationship internally between feats.
--any given feat can require a number of other feats to be taken,
--and a given feat can be required by any number of other feats.
--this table accomplishes that.
CREATE TABLE IF NOT EXISTS FeatRequirements (
	feat_id			UUID REFERENCES Feats
		NOT NULL,
	required_feat		UUID REFERENCES Feats
		NOT NULL
);

CREATE TABLE IF NOT EXISTS CharacterFeats (
	char_id			UUID REFERENCES Characters
		NOT NULL,
	feat_id			UUID REFERENCES Feats
		NOT NULL
);

CREATE TABLE IF NOT EXISTS RacialFeats (
	race_id			UUID REFERENCES Races
		NOT NULL,
	feat_id			UUID REFERENCES Feats
		NOT NULL
);

CREATE TABLE IF NOT EXISTS ClassFeats (
	class_id		UUID REFERENCES Classes,
		NOT NULL
	feat_id			UUID REFERENCES Feats
		NOT NULL
);

--Features tables
CREATE TABLE IF NOT EXISTS Features (
	feature_id		UUID PRIMARY KEY,
	description		TEXT
		NOT NULL
);

CREATE TABLE IF NOT EXISTS ClassFeatures (
	class_id		UUID REFERENCES Feats
		NOT NULL,
	feature_id		UUID REFERENCES Features
		NOT NULL,
	is_default		BOOLEAN
		DEFAULT false
);

CREATE TABLE IF NOT EXISTS CharacterFeatures (
	char_id			UUID REFERENCES Characters
		NOT NULL,
	feature_id		UUID REFERENCES Features
		NOT NULL
);

CREATE TABLE IF NOT EXISTS SubclassFeatures (
	subclass_id		UUID REFERENCES Subclasses
		NOT NULL,
	feature_id		UUID REFERENCES Features
		NOT NULL
);

--class proficiencies tables
CREATE TABLE IF NOT EXISTS ClassWeaponsNotProficient (
	class_id		UUID REFERENCES Classes
		NOT NULL,
	weapon_classes		WeaponClass[]
);

CREATE TABLE IF NOT EXISTS ClassArmorsNotProficient (
	class_id		UUID REFERENCES Classes
		NOT NULL,
	armor_classes		ArmorClass[]
);

--Spells tables
CREATE TABLE IF NOT EXISTS Spells (
	spell_id		UUID PRIMARY KEY,

	name			TEXT
		NOT NULL,
	level			INT
		NOT NULL
		CHECK (level >= 0),
	school			MagicSchool
		NOT NULL,

	casting_time		INT
		NOT NULL
		CHECK (casting_time >= 0),
	range			SpellRange
		NOT NULL,
	area			TEXT
		NOT NULL,
	duration_per_level	INT
		NOT NULL
		CHECK (duration_per_level > 0),
	saving_throw		SaveThrow,
	spell_resistance	BOOLEAN
		DEFAULT false,
	description		TEXT
		NOT NULL
);

CREATE TABLE IF NOT EXISTS CharacterSpells (
	char_id			UUID REFERENCES Characters
		NOT NULL,
	spell_id		UUID REFERENCES Spells
		NOT NULL,
	casts			INT
		NOT NULL
		CHECK (casts >= 0)
);

CREATE TABLE IF NOT EXISTS SubclassSpells (
	subclass_id		UUID REFERENCES Subclasses,
	spell_id		UUID REFERENCES Spells,

	casts			INT
		NOT NULL
		CHECK (casts >= 0),
	req_level		INT
		NOT NULL
		CHECK (casts >= 0)
);

CREATE TABLE IF NOT EXISTS DomainSpells (
	domain_id		UUID REFERENCES Domains,
	spell_id		UUID REFERENCES Spells,
	casts			INT
		NOT NULL
		CHECK (casts >= 0)
);

CREATE TABLE IF NOT EXISTS SpellComponents (
	spell_id		UUID REFERENCES Spells
		NOT NULL,
	item_id			UUID REFERENCES Items,

	item_amount			INT
		CHECK (item_amount >= 0)
	component_type		ComponentType
		NOT NULL
);

--Domain tables
CREATE TABLE IF NOT EXISTS Domains (
	domain_id		UUID PRIMARY KEY,

	name			TEXT
		NOT NULL,
	description		TEXT
		NOT NULL,
	power_description	TEXT
		NOT NULL
);

CREATE TABLE IF NOT EXISTS Subdomains (
	domain_id		UUID REFERENCES Domains
		NOT NULL,

	name			TEXT
		NOT NULL,
	description		TEXT
		NOT NULL
);

--Deity tables
CREATE TABLE IF NOT EXISTS Deities (
	deity_id		UUID PRIMARY KEY,
	
	name			TEXT
		NOT NULL,
	description		TEXT
		NOT NULL,
	favored_animals		TEXT
);

CREATE TABLE IF NOT EXISTS DeityDomains (
	deity_id		UUID REFERENCES Deities
		NOT NULL,
	domain_id		UUID REFERENCES Domains
		NOT NULL 	--no deities without domains
);

CREATE TABLE IF NOT EXISTS DeityWeapons (
	deity_id		UUID REFERENCES Deities
		NOT NULL,
	item_id			UUID REFERENCES Items
);

--Item tables
CREATE TABLE IF NOT EXISTS PathfinderItems (
	item_id			UUID PRIMARY KEY,

	name			TEXT
		NOT NULL,
	description		TEXT,
	cost			INT	--prices are in copper
		NOT NULL
		CHECK (cost >= 0),
	weight			INT
		NOT NULL
		CHECK (weight >=0),
	equip_slot		EquipmentSlot
);

CREATE TABLE IF NOT EXISTS Weapons (
	item_id			UUID REFERENCES Items
		NOT NULL,
	material_id		UUID REFERENCES Materials,

	weapon_range		INT4RANGE
		NOT NULL,
	crit_range		INT4RANGE
		NOT NULL,
	damage			TEXT[]
		NOT NULL,
	damage_type		DamageType[]
		NOT NULL,
	weapon_type		WeaponClass
		NOT NULL
);

CREATE TABLE IF NOT EXISTS Armor (
	item_id			UUID REFERENCES Items
		NOT NULL,
	material_id		UUID REFERENCES Materials,
	
	max_dex_bonus		INT
		NOT NULL
		CHECK (max_dex_bonus >= 0),
	ac			INT
		NOT NULL
		CHECK (ac >= 0),
	spell_failure		INT
		NOT NULL
		CHECK (spell_failure >= 0),
	check_penalty		INT
		NOT NULL
		CHECK (check_penalty >= 0),
	armor_type		ArmorClass
		NOT NULL
);

CREATE TABLE IF NOT EXISTS ItemsInBags (
	item_id			UUID REFERENCES Items
		NOT NULL,
	bag_id			UUID REFERENCES Bags
		NOT NULL
);

CREATE TABLE IF NOT EXISTS Bags (
	bag_id			UUID PRIMARY KEY,
	char_id			UUID REFERENCES Characters
		NOT NULL,
	item_id			UUID REFERENCES Items
		NOT NULL,

	capacity		INT
		NOT NULL
		CHECK (capacity > 0)
);

CREATE TABLE IF NOT EXISTS CharacterEquipment (
	char_id			UUID REFERENCES Characters
		NOT NULL,
	item_id			UUID REFERENCES Items
		NOT NULL
);

--Materials Tables
CREATE TABLE IF NOT EXISTS Materials (
	material_id		UUID PRIMARY KEY,

	name			TEXT
		NOT NULL,
	description		TEXT
		NOT NULL,

	--both of these are not universal for materials, as far as I can tell.
	hp_per_inch		INT,
	hardness		INT
);

--Effects tables
CREATE TABLE IF NOT EXISTS Effects (
	effect_id		UUID PRIMARY KEY,
	short_description	TEXT
		NOT NULL,
	long_description	TEXT
);

CREATE TABLE IF NOT EXISTS RaceEffects (
	race_id			UUID REFERENCES Races
		NOT NULL,
	effect_id		UUID REFERENCES Effects
		NOT NULL
);

CREATE TABLE IF NOT EXISTS RaceTypeEffects (
	type_id			UUID REFERENCES RaceTypes
		NOT NULL,
	effect_id		UUID REFERENCES Effects
		NOT NULL
	
);

CREATE TABLE IF NOT EXISTS RaceSubtypeEffects (
	subtype_id		UUID REFERENCES RaceSubtypes
		NOT NULL,
	effect_id		UUID REFERENCES Effects
		NOT NULL
);

CREATE TABLE IF NOT EXISTS ClassEffects (
	class_id		UUID REFERENCES Classes
		NOT NULL,
	effect_id		UUID REFERENCES Effects
		NOT NULL
);

CREATE TABLE IF NOT EXISTS ItemEffects (
	item_id			UUID REFERENCES Items
		NOT NULL,
	effect_id		UUID REFERENCES Effects
		NOT NULL,
	is_permanent		BOOLEAN
		DEFAULT false
);

CREATE TABLE IF NOT EXISTS SpellEffects (
	spell_id		UUID REFERENCES Spells
		NOT NULL,
	effect_id		UUID REFERENCES Effects
		NOT NULL
);

CREATE TABLE IF NOT EXISTS FeatEffects (
	feat_id			UUID REFERENCES Feats
		NOT NULL,
	effect_id		UUID REFERENCES Effects
		NOT NULL
);

CREATE TABLE IF NOT EXISTS FeatureEffects (
	feature_id		UUID REFERENCES Features
		NOT NULL,
	effect_id		UUID REFERENCES Effects
		NOT NULL
);

CREATE TABLE IF NOT EXISTS DomainEffects (
	domain_id		UUID REFERENCES Domains
		NOT NULL,
	effect_id		UUID REFERENCES Effects
		NOT NULL
);

CREATE TABLE IF NOT EXISTS MaterialEffects (
	material_id		UUID REFERENCES Materials
		NOT NULL,
	effect_id		UUID REFERENCES Effects
		NOT NULL
);

CREATE TABLE IF NOT EXISTS AttributeEffectUnits (
	effect_id		UUID REFERENCES Effects
		NOT NULL,
	attr_unit_id		UUID REFERENCES AttributeUnits
		NOT NULL
);

CREATE TABLE IF NOT EXISTS AttributeUnits (
	attr_unit_id		UUID PRIMARY KEY,
	base_attr		Attribute
		NOT NULL,
	modifier		INT
		NOT NULL
);

CREATE TABLE IF NOT EXISTS SkillEffectUnits (
	effect_id		UUID REFERENCES Effects
		NOT NULL,
	skill_unit_id		UUID REFERENCES SkillUnits
		NOT NULL
);

CREATE TABLE IF NOT EXISTS SkillUnits (
	skill_unit_id		UUID PRIMARY KEY,
	skill			Skill
		NOT NULL,
	modifier		INT
		NOT NULL
);

CREATE TABLE IF NOT EXISTS CharacterEffectUnits (
	effect_id		UUID REFERENCES Effects
		NOT NULL,
	char_unit_id		UUID REFERENCES CharacterUnits
		NOT NULL
);

CREATE TABLE IF NOT EXISTS CharacterUnits (
	char_unit_id		UUID PRIMARY KEY,
	character_stat		CharacterStat
		NOT NULL,
	modifier		INT
		NOT NULL
);

CREATE TABLE IF NOT EXISTS CombatEffectUnits (
	effect_id		UUID REFERENCES Effects
		NOT NULL,
	combat_unit_id		UUID REFERENCES CombatUnits
		NOT NULL
);

CREATE TABLE IF NOT EXISTS CombatUnits (
	combat_unit_id		UUID PRIMARY KEY,
	combat_stat		CombatStat
		NOT NULL,
	modifier		INT
		NOT NULL
);

CREATE TABLE IF NOT EXISTS MiscEffectUnits (
	effect_id		UUID REFERENCES Effects
		NOT NULL,
	misc_unit_id		UUID REFERENCES MiscUnits
		NOT NULL
);

CREATE TABLE IF NOT EXISTS MiscUnits (
	misc_unit_id		UUID PRIMARY KEY
		NOT NULL,
	description		TEXT
		NOT NULL
);
