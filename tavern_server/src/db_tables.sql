--enum types
CREATE TYPE Gender AS ENUM (
	'Male', 
	'Female', 
	'Other'
);

CREATE TYPE Size AS ENUM (
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

CREATE TYPE Alignment AS ENUM (
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

CREATE TYPE Attribute AS ENUM (
	'Strength', 
	'Dexterity', 
	'Constitution', 
	'Intelligence', 
	'Wisdom', 
	'Charisma'
);

CREATE TYPE Skill AS ENUM (
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

CREATE TYPE SaveThrow AS ENUM (
	'Fortitude', 
	'Reflex',
	'Will'
);

CREATE TYPE CharacterStat AS ENUM (
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

CREATE TYPE CombatStat AS ENUM (
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
	'Fortititude Save',
	'Reflex Save',
	'Will Save'
);

CREATE TYPE RaceType AS ENUM (
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

CREATE TYPE CasterType AS ENUM (
	'Spontaneous', 
	'Prepared'
);

CREATE TYPE MagicSchool AS ENUM (
	'Abjuration', 
	'Conjuration', 
	'Divination', 
	'Enchantment', 
	'Evocation', 
	'Illusion', 
	'Necromancy', 
	'Transmutation'
);

CREATE TYPE WeaponClass AS ENUM (
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

CREATE TYPE ArmorClass AS ENUM (
	'Light', 
	'Medium', 
	'Heavy'
);

CREATE TYPE EquipmentSlot AS ENUM (
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

CREATE TYPE DamageType AS ENUM (
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

CREATE TYPE Material AS ENUM (
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
CREATE TABLE UserInfo (
	user_id     		UUID

        name        		TEXT
        is_admin    		BOOL

        pass_hash   		BYTEA
        salt        		BYTEA

        time_cost   		INT
        memory      		INT
        threads     		INT
);

--Character overview table
CREATE TABLE Character (
        user_id         	UUID
        char_id         	UUID
        race_id         	UUID
        deity_id        	UUID

        name            	TEXT
        age             	INT
        gender          	Gender
        alignment       	Alignment
        backstory       	TEXT
        height          	INT
        weight          	INT
        size            	Size

        strength        	INT
        dexterity       	INT
        constitution    	INT
        intelligence    	INT
        wisdom          	INT
        charisma        	INT

        max_hp          	INT
        damage          	INT
        nonlethal       	INT

        copper          	INT
        silver          	INT
        gold            	INT
        platinum        	INT
);

--Race tables
CREATE TABLE Races (
	race_id 		UUID

	name			TEXT
	move_speed		INT
	size			Size
	languages		TEXT
	race_type		RaceType
);

--Class tables
CREATE TABLE Classes (
	class_id		UUID

	name			TEXT
	hit_die			TEXT
	starting_wealth		TEXT
	bab_per_level		REAL
	skills_per_level	INT
	skills_attr		Attribute
);

CREATE TABLE Subclasses (
	subclass_id		UUID
	class_id		UUID

	caster_type		CasterType
	casting_attr		Attribute
);

CREATE TABLE characterSubclasses (
	char_id			UUID
	subclass_id		UUID

	levels_taken		INT
	hp_taken		INT
	skills_taken		INT
);

--Feats tables
CREATE TABLE Feats (
	feat_id			UUID
	req_id			UUID

	short_description	TEXT
	long description	TEXT
);

CREATE TABLE SkillReqUnits (
	req_id			UUID
	skill_unit_id		UUID
);

CREATE TABLE SkillUnits (
	skill_unit_id		UUID
	req_skill		Skill
	ranks			INT
);

CREATE TABLE AttrReqUnits (
	req_id			UUID
	attr_unit_id		UUID
);

CREATE TABLE AttrUnits (
	attr_unit_id		UUID
	req_attr		Attribute
	score			INT
);

--this table creates a many to many realationship internally between feats.
--any given feat can require a number of other feats to be taken,
--and a given feat can be required by any number of other feats.
--this table accomplishes that.
CREATE TABLE RequiredFeats (
	req_id			UUID
	feat__id		UUID
);

CREATE TABLE CharacterFeats (
	char_id			UUID
	feat_id			UUID
);

CREATE TABLE RacialFeats (
	race_id			UUID
	feat_id			UUID
);

CREATE TABLE ClassFeats (
	class_id		UUID
	feat_id			UUID
);

--Features tables
CREATE TABLE Features (
	feature_id		UUID
	description		TEXT
);

CREATE TABLE ClassFeatures (
	class_id		UUID
	feature_id		UUID
	is_default		BOOLEAN
);

CREATE TABLE CharacterFeatures (
	char_id			UUID
	feature_id		UUID
);

CREATE TABLE SubclassFeatures (
	subclass_id		UUID
	feature_id		UUID
);

--class proficiencies tables
CREATE TABLE ClassProficientSingleWeapon (
	class_id		UUID
	item_id			UUID
);

CREATE TABLE ClassProficientWeaponClass (
	class_id		UUID
	weapon_class		WeaponClass
);

CREATE TABLE ClassProficientWeaponAllBut (
	 class_id		UUID
	weapon_class		WeaponClass
);

CREATE TABLE ClassProficientArmorClass (
	class_id		UUID
	armor_class		ArmorClass
);

CREATE TABLE ClassProficientArmorAllBut (
	class_id		UUID
	armor_class		ArmorClass
);

--Spells tables
CREATE TABLE Spells (
	spell_id		UUID

	name			TEXT
	level			INT
	school			MagicSchool

	casting_time		INT
	range			Range
	area			TEXT
	duration_per_level	INT
	saving_throw		SaveThrow
	spell_resistance	BOOLEAN
	description		TEXT
);

CREATE TABLE CharacterSpells (
	char_id			UUID
	spell_id		UUID
	casts			INT
);

CREATE TABLE SubclassSpells (
	class_id		UUID
	spell_id		UUID
	casts			INT
	req_level		INT
);

CREATE TABLE DomainSpells (
	domain_id		UUID
	spell_id		UUID
	casts			INT
);

CREATE TABLE SpellComponents (
	spell_id		UUID
	item_id			UUID
	amount			INT
);

--Domain tables
CREATE TABLE Domains (
	domain_id		UUID
	effect_id		UUID

	name			TEXT
	description		TEXT
	power_description	TEXT
);

CREATE TABLE Subdomains (
	domain_id		UUID
	effect_id		UUID

	name			TEXT
	description		TEXT
);

--Deity tables
CREATE TABLE Deities (
	deity_id		UUID
	
	name			TEXT
	description		TEXT
	favored_animals		TEXT
);

CREATE TABLE DeityDomains (
	deity_id		UUID
	domain_id		UUID
);

CREATE TABLE DeityWeapons (
	deity_id		UUID
	item_id			UUID
);

--Item tables
CREATE TABLE Items (
	item_id			UUID
	cost			INT
	description		TEXT
	name			TEXT
	weight			INT
	equip_slot		EquipmentSlot
)

CREATE TABLE Weapons (
	item_id			UUID
	weapon_range		INT4RANGE
	crit_range		INT4RANGE
	damage			TEXT
	damage_type		DamageType
	weapon_type		WeaponClass
	material		Material
);

CREATE TABLE Armor (
	item_id			UUID
	max_dex_bonus		INT
	ac			INT
	spell_failure		INT
	check_penalty		INT
	material		Material
	armor_type		ArmorClass
);

CREATE TABLE ItemsInBags (
	item_id			UUID
	bag_id			UUID
);

CREATE TABLE CharacterBags (
	char_id			UUID
	bag_id			UUID
	item_id			UUID
	capacity		INT
);

CREATE TABLE CharacterEquipment (
	char_id			UUID
	item_id			UUID
);

--Effects tables
CREATE TABLE Effects (
	effect_id		UUID
	short_description	TEXT
	long_description	TEXT
);

CREATE TABLE RaceEffects (
	race_id			UUID
	effect_id		UUID
);

CREATE TABLE ClassEffects (
	class_id		UUID
	effect_id		UUID
);

CREATE TABLE ItemEffects (
	item_id			UUID
	effect_id		UUID
	is_permanent		BOOLEAN
);

CREATE TABLE SpellEffects (
	spell_id		UUID
	effect_id		UUID
);

CREATE TABLE FeatEffects (
	feat_id			UUID
	effect_id		UUID
);

CREATE TABLE FeatureEffects (
	feature_id		UUID
	effect_id		UUID
);

CREATE TABLE AttributeEffectUnits (
	effect_id		UUID
	attr_unit_id		UUID
);

CREATE TABLE AttributeUnits (
	attr_unit_id		UUID
	base_attr		Attribute
	modifier		INT
);

CREATE TABLE SkillEffectUnits (
	effect_id		UUID
	skill_unit_id		UUID
);

CREATE TABLE SkillUnits (
	skill_unit_id		UUID
	skill			Skill
	modifier		INT
);

CREATE TABLE CharacterEffectUnits (
	effect_id		UUID
	char_unit_id		UUID
);

CREATE TABLE CharacterUnits (
	char_unit_id		UUID
	character_stat		CharacterStat
	modifier		INT
);

CREATE TABLE CombatEffectUnits (
	effect_id		UUID
	combat_unit_id		UUID
);

CREATE TABLE CombatUnits (
	combat_unit_id		UUID
	combat_stat		CombatStat
	modifier		INT
);

CREATE TABLE MiscEffectUnits (
	effect_id		UUID
	misc_unit_id		UUID
);

CREATE TABLE MiscUnits (
	misc_unit_id		UUID
	description		TEXT
);
