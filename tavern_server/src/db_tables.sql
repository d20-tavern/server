CREATE SCHEMA IF NOT EXISTS tavern;
SET search_path TO tavern;

DO $$ BEGIN
    CREATE TYPE Gender AS ENUM (
        'Male', 
        'Female', 
        'Other'
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
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
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
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
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE Attribute AS ENUM (
        'Strength', 
        'Dexterity', 
        'Constitution', 
        'Intelligence', 
        'Wisdom', 
        'Charisma'
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
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
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE SaveThrow AS ENUM (
        'Fortitude', 
        'Reflex',
        'Will'
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
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
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
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
        'Fortitude Save',
        'Reflex Save',
        'Will Save'
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE CasterType AS ENUM (
        'Spontaneous', 
        'Prepared'
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE ComponentType AS ENUM (
        'Somatic',
        'Material',
        'Verbal'
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
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
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE SpellRange AS ENUM (
        'Personal',
        'Touch',
        'Close',
        'Medium',
        'Long',
        'Unlimited'
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
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
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE ArmorClass AS ENUM (
        'Light', 
        'Medium', 
        'Heavy'
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE EquipmentSlot AS ENUM (
        'NoSlot',
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
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
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
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

--table declarations
--user creds table - also used for hashing info
CREATE TABLE IF NOT EXISTS Users (
    id          UUID    PRIMARY KEY,
    email       TEXT    CONSTRAINT user_email_required    NOT NULL CONSTRAINT user_email_unique    UNIQUE,
    username    TEXT    CONSTRAINT user_username_required NOT NULL CONSTRAINT user_username_unique UNIQUE,
    is_admin    BOOLEAN DEFAULT false,
    pass_hash   BYTEA   NOT NULL,
    salt        BYTEA   NOT NULL,
    time_cost   INT     NOT NULL CHECK (time_cost > 0),
    memory      INT     NOT NULL CHECK (memory > 0),
    threads     INT     NOT NULL CHECK (threads > 0)
);

CREATE TABLE IF NOT EXISTS RaceTypes (
    id                  UUID    PRIMARY KEY,
    name                TEXT    NOT NULL,
    hit_die             TEXT    NOT NULL,
    bab_per_hit_die     REAL    NOT NULL
);

CREATE TABLE IF NOT EXISTS RaceSubtypes (
    id              UUID    PRIMARY KEY,
    name            TEXT    NOT NULL,
    description     TEXT    NOT NULL
);

CREATE TABLE IF NOT EXISTS Races (
    id              UUID        PRIMARY KEY,
    description     TEXT        NOT NULL,
    type_id         UUID        REFERENCES RaceTypes(id) NOT NULL,
    subtype_id      UUID        REFERENCES RaceSubtypes(id),
    name            TEXT        NOT NULL,
    move_speed      SMALLINT    NOT NULL CHECK (move_speed > 0),
    size            Size        NOT NULL,
    languages       TEXT[]
);

--Deity tables
CREATE TABLE IF NOT EXISTS Deities (
    id                  UUID    PRIMARY KEY,
    name                TEXT    NOT NULL,
    description         TEXT    NOT NULL,
    favored_animals     TEXT
);

--Character overview table
CREATE TABLE IF NOT EXISTS Characters (
    id              UUID        PRIMARY KEY,
    user_id         UUID        REFERENCES Users(id) NOT NULL,
    race_id         UUID        REFERENCES Races(id) NOT NULL,
    deity_id        UUID        REFERENCES Deities(id) NOT NULL,
    name            TEXT        NOT NULL,
    age             SMALLINT    NOT NULL CHECK (age > 0),
    gender          Gender      NOT NULL,
    alignment       Alignment   NOT NULL,
    backstory       TEXT,
    height          SMALLINT    CHECK (height > 0),
    weight          SMALLINT    CHECK (weight > 0),
    size            Size        NOT NULL,

    strength        SMALLINT    NOT NULL CHECK (strength >= 0     AND strength <= 20),
    dexterity       SMALLINT    NOT NULL CHECK (dexterity >= 0    AND dexterity <= 20),
    constitution    SMALLINT    NOT NULL CHECK (constitution >= 0 AND constitution <= 20),
    intelligence    SMALLINT    NOT NULL CHECK (intelligence >= 0 AND intelligence <= 20),
    wisdom          SMALLINT    NOT NULL CHECK (wisdom >= 0       AND wisdom <= 20),
    charisma        SMALLINT    NOT NULL CHECK (charisma >= 0     AND charisma <= 20),

    max_hp          SMALLINT    NOT NULL CHECK (max_hp >= 0),
    damage          SMALLINT    NOT NULL CHECK (damage >= 0),
    nonlethal       SMALLINT    NOT NULL CHECK (nonlethal >= 0),

    copper          SMALLINT    NOT NULL CHECK (copper >= 0),
    silver          SMALLINT    NOT NULL CHECK (silver >= 0),
    gold            SMALLINT    NOT NULL CHECK (gold >= 0),
    platinum        SMALLINT    NOT NULL CHECK (platinum >= 0)
);

--Class tables
CREATE TABLE IF NOT EXISTS Classes (
    id                  UUID                PRIMARY KEY,
    description         TEXT                NOT NULL,
    name                TEXT                NOT NULL,
    hit_die             TEXT                NOT NULL,
    starting_wealth     TEXT                NOT NULL,
    bab_per_level       DOUBLE PRECISION    NOT NULL,
    skills_per_level    SMALLINT            NOT NULL,
    skills_attr         Attribute           NOT NULL
);

CREATE TABLE IF NOT EXISTS Subclasses (
    id              UUID        PRIMARY KEY,
    class_id        UUID        REFERENCES Classes(id) NOT NULL,
    caster_type     CasterType,
    casting_attr    Attribute
);

CREATE TABLE IF NOT EXISTS characterSubclasses (
    char_id         UUID        REFERENCES Characters(id) NOT NULL,
    subclass_id     UUID        REFERENCES Subclasses(id) NOT NULL,
    levels_taken    SMALLINT    NOT NULL CHECK (levels_taken > 0),
    hp_taken        SMALLINT    NOT NULL CHECK (hp_taken >= 0),
    skills_taken    SMALLINT    NOT NULL CHECK (skills_taken >= 0)
);

--Feats tables
CREATE TABLE IF NOT EXISTS Feats (
    id                  UUID    PRIMARY KEY,
    name                TEXT    UNIQUE NOT NULL,
    short_description   TEXT    NOT NULL,
    long_description    TEXT
);

CREATE TABLE IF NOT EXISTS SkillFeatUnits (
    id          UUID        REFERENCES Feats(id) NOT NULL,
    skill       Skill       NOT NULL,
    ranks       SMALLINT    NOT NULL,
    PRIMARY KEY (id, skill)
);

CREATE TABLE IF NOT EXISTS AttributeFeatUnits (
    id              UUID        REFERENCES Feats(id) NOT NULL,
    attr            Attribute   NOT NULL,
    score           SMALLINT    NOT NULL CHECK(score != 0),
    PRIMARY KEY (id, attr)
);

--this table creates a many to many realationship internally between feats.
--any given feat can require a number of other feats to be taken,
--and a given feat can be required by any number of other feats.
--this table accomplishes that.
CREATE TABLE IF NOT EXISTS FeatRequirements (
    feat_id         UUID    REFERENCES Feats(id) NOT NULL,
    required_feat   UUID    REFERENCES Feats(id) NOT NULL,
    CHECK (feat_id != required_feat)
);

CREATE TABLE IF NOT EXISTS CharacterFeats (
    char_id     UUID    REFERENCES Characters(id),
    feat_id     UUID    REFERENCES Feats(id),
    PRIMARY KEY (char_id, feat_id)
);

CREATE TABLE IF NOT EXISTS RacialFeats (
    race_id     UUID    REFERENCES Races(id),
    feat_id     UUID    REFERENCES Feats(id),
    PRIMARY KEY (race_id, feat_id)
);

CREATE TABLE IF NOT EXISTS ClassFeats (
    class_id    UUID    REFERENCES Classes(id),
    feat_id     UUID    REFERENCES Feats(id),
    PRIMARY KEY (class_id, feat_id)
);

--Features tables
CREATE TABLE IF NOT EXISTS Features (
    id              UUID    PRIMARY KEY,
    description     TEXT    NOT NULL
);

CREATE TABLE IF NOT EXISTS ClassFeatures (
    class_id    UUID        REFERENCES Feats(id) NOT NULL,
    feature_id  UUID        REFERENCES Features(id) NOT NULL,
    is_default  BOOLEAN     DEFAULT false
);

CREATE TABLE IF NOT EXISTS CharacterFeatures (
    char_id     UUID        REFERENCES Characters(id) NOT NULL,
    feature_id  UUID        REFERENCES Features(id) NOT NULL
);

CREATE TABLE IF NOT EXISTS SubclassFeatures (
    subclass_id     UUID    REFERENCES Subclasses(id) NOT NULL,
    feature_id      UUID    REFERENCES Features(id) NOT NULL
);

--Item tables
CREATE TABLE IF NOT EXISTS Items (
    id              UUID    PRIMARY KEY,
    name            TEXT    NOT NULL,
    description     TEXT,
    cost            INT     NOT NULL CHECK (cost >= 0), -- prices are in copper
    weight          FLOAT8  NOT NULL CHECK (weight >=0),
    equip_slot      EquipmentSlot
);

--Materials Tables
CREATE TABLE IF NOT EXISTS Materials (
    id              UUID    PRIMARY KEY,
    name            TEXT    UNIQUE NOT NULL,
    description     TEXT    NOT NULL,
    --both of these are not universal for materials, as far as I can tell.
    hp_per_inch     INT,
    hardness        INT
);

CREATE TABLE IF NOT EXISTS Weapons (
    id              UUID            PRIMARY KEY REFERENCES Items(id),
    material_id     UUID            REFERENCES Materials(id),
    weapon_range    INT4RANGE       NOT NULL,
    crit_range      INT4RANGE       NOT NULL,
    damage          TEXT[]          NOT NULL,
    damage_type     DamageType[]    NOT NULL,
    weapon_type     WeaponClass     NOT NULL
);

CREATE TABLE IF NOT EXISTS Armor (
    id              UUID        PRIMARY KEY REFERENCES Items(id),
    material_id     UUID        REFERENCES Materials(id),
    max_dex_bonus   INT         NOT NULL CHECK (max_dex_bonus >= 0),
    ac              INT         NOT NULL CHECK (ac >= 0),
    spell_failure   INT         NOT NULL CHECK (spell_failure >= 0),
    check_penalty   INT         NOT NULL CHECK (check_penalty >= 0),
    armor_type      ArmorClass  NOT NULL
);

--class proficiencies tables
CREATE TABLE IF NOT EXISTS ClassProficientArmorClasses (
    class_id        UUID            PRIMARY KEY REFERENCES Classes(id),
    armor_classes   ArmorClass[]    NOT NULL
);

CREATE TABLE IF NOT EXISTS ClassProficientArmor (
    class_id    UUID    REFERENCES Classes(id),
    armor_id    UUID    REFERENCES Armor(id),
    PRIMARY KEY (class_id, armor_id)
);

CREATE TABLE IF NOT EXISTS ClassNotProficientArmor (
    class_id    UUID    REFERENCES Classes(id),
    armor_id    UUID    REFERENCES Armor(id),
    PRIMARY KEY (class_id, armor_id)
);

CREATE TABLE IF NOT EXISTS ClassProficientWeaponClasses (
    class_id        UUID            PRIMARY KEY REFERENCES Classes(id),
    weapon_classes  WeaponClass[]   NOT NULL
);

CREATE TABLE IF NOT EXISTS ClassProficientWeapons (
    class_id    UUID    REFERENCES Classes(id),
    weapon_id   UUID    REFERENCES Weapons(id),
    PRIMARY KEY (class_id, weapon_id)
);

CREATE TABLE IF NOT EXISTS ClassNotProficientWeapons (
    class_id    UUID    REFERENCES Classes(id),
    weapon_id   UUID    REFERENCES Weapons(id),
    PRIMARY KEY (class_id, weapon_id)
);

CREATE TABLE IF NOT EXISTS Bags (
    id          UUID    PRIMARY KEY,
    char_id     UUID    REFERENCES Characters(id) NOT NULL,
    item_id     UUID    REFERENCES Items(id) NOT NULL,
    capacity    INT     NOT NULL CHECK (capacity > 0)
);

CREATE TABLE IF NOT EXISTS ItemsInBags (
    item_id     UUID    REFERENCES Items(id) NOT NULL,
    bag_id      UUID    REFERENCES Bags(id) NOT NULL,
    count       INT     NOT NULL CHECK (count > 0)
);

CREATE TABLE IF NOT EXISTS CharacterEquipment (
    char_id     UUID    REFERENCES Characters(id) NOT NULL,
    item_id     UUID    REFERENCES Items(id) NOT NULL
);

--Spells tables
CREATE TABLE IF NOT EXISTS Spells (
    id                  UUID            PRIMARY KEY,
    name                TEXT            NOT NULL,
    level               SMALLINT        NOT NULL CHECK (level >= 0),
    school              MagicSchool     NOT NULL,
    casting_time        BIGINT          NOT NULL CHECK (casting_time >= 0),
    range               SpellRange      NOT NULL,
    area                TEXT            NOT NULL,
    duration_per_level  BIGINT          NOT NULL CHECK (duration_per_level > 0),
    saving_throw        SaveThrow,
    spell_resistance    BOOLEAN         DEFAULT false,
    description         TEXT            NOT NULL
);

CREATE TABLE IF NOT EXISTS CharacterSpells (
    char_id     UUID        REFERENCES Characters(id) NOT NULL,
    spell_id    UUID        REFERENCES Spells(id) NOT NULL,
    casts       SMALLINT    NOT NULL CHECK (casts >= 0)
);

CREATE TABLE IF NOT EXISTS SubclassSpells (
    subclass_id     UUID        REFERENCES Subclasses(id),
    spell_id        UUID        REFERENCES Spells(id),
    casts           SMALLINT    NOT NULL CHECK (casts >= 0),
    req_level       SMALLINT    NOT NULL CHECK (req_level >= 0)
);

--Domain tables
CREATE TABLE IF NOT EXISTS Domains (
    id                  UUID    PRIMARY KEY,
    name                TEXT    NOT NULL,
    description         TEXT    NOT NULL,
    power_description   TEXT    NOT NULL
);

CREATE TABLE IF NOT EXISTS DomainSpells (
    domain_id   UUID        REFERENCES Domains(id),
    spell_id    UUID        REFERENCES Spells(id),
    casts       SMALLINT    NOT NULL CHECK (casts >= 0)
);

CREATE TABLE IF NOT EXISTS SpellComponents (
    spell_id        UUID            REFERENCES Spells(id) NOT NULL,
    item_id         UUID            REFERENCES Items(id),
    item_amount     SMALLINT        CHECK (item_amount >= 0),
    component_type  ComponentType   NOT NULL
);

CREATE TABLE IF NOT EXISTS Subdomains (
    domain_id       UUID    REFERENCES Domains(id) NOT NULL,
    name            TEXT    NOT NULL,
    description     TEXT    NOT NULL
);

CREATE TABLE IF NOT EXISTS DeityDomains (
    deity_id    UUID    REFERENCES Deities(id) NOT NULL,
    domain_id   UUID    REFERENCES Domains(id) NOT NULL     --no deities without domains
);

CREATE TABLE IF NOT EXISTS DeityWeapons (
    deity_id    UUID    REFERENCES Deities(id),
    item_id     UUID    REFERENCES Weapons(id),
    PRIMARY KEY (deity_id, item_id)
);

--Effects tables
CREATE TABLE IF NOT EXISTS Effects (
    id                  UUID    PRIMARY KEY,
    name                TEXT    UNIQUE NOT NULL,
    short_description   TEXT    NOT NULL,
    long_description    TEXT
);

CREATE TABLE IF NOT EXISTS RaceEffects (
    race_id     UUID    REFERENCES Races(id) NOT NULL,
    effect_id   UUID    REFERENCES Effects(id) NOT NULL
);

CREATE TABLE IF NOT EXISTS RaceTypeEffects (
    type_id     UUID    REFERENCES RaceTypes(id) NOT NULL,
    effect_id   UUID    REFERENCES Effects(id) NOT NULL
);

CREATE TABLE IF NOT EXISTS RaceSubtypeEffects (
    subtype_id  UUID    REFERENCES RaceSubtypes(id) NOT NULL,
    effect_id   UUID    REFERENCES Effects(id) NOT NULL
);

CREATE TABLE IF NOT EXISTS ClassEffects (
    class_id    UUID    REFERENCES Classes(id) NOT NULL,
    effect_id   UUID    REFERENCES Effects(id) NOT NULL
);

CREATE TABLE IF NOT EXISTS ItemEffects (
    item_id         UUID        REFERENCES Items(id) NOT NULL,
    effect_id       UUID        REFERENCES Effects(id) NOT NULL,
    is_permanent    BOOLEAN     DEFAULT false
);

CREATE TABLE IF NOT EXISTS SpellEffects (
    spell_id        UUID    REFERENCES Spells(id) NOT NULL,
    effect_id       UUID    REFERENCES Effects(id) NOT NULL
);

CREATE TABLE IF NOT EXISTS FeatEffects (
    feat_id     UUID    REFERENCES Feats(id) NOT NULL,
    effect_id   UUID    REFERENCES Effects(id) NOT NULL
);

CREATE TABLE IF NOT EXISTS FeatureEffects (
    feature_id  UUID    REFERENCES Features(id) NOT NULL,
    effect_id   UUID    REFERENCES Effects(id) NOT NULL
);

CREATE TABLE IF NOT EXISTS DomainEffects (
    domain_id   UUID    REFERENCES Domains(id) NOT NULL,
    effect_id   UUID    REFERENCES Effects(id) NOT NULL
);

CREATE TABLE IF NOT EXISTS MaterialEffects (
    material_id     UUID REFERENCES Materials(id) NOT NULL,
    effect_id       UUID REFERENCES Effects(id) NOT NULL
);

CREATE TABLE IF NOT EXISTS AttributeUnits (
    id              UUID        REFERENCES Effects(id),
    attr            Attribute   NOT NULL,
    modifier        SMALLINT    NOT NULL,
    PRIMARY KEY (id, attr)
);

CREATE TABLE IF NOT EXISTS SkillUnits (
    id              UUID        REFERENCES Effects(id),
    skill           Skill       NOT NULL,
    modifier        SMALLINT    NOT NULL,
    PRIMARY KEY (id, skill)
);

CREATE TABLE IF NOT EXISTS CharacterUnits (
    id              UUID            REFERENCES Effects(id),
    stat            CharacterStat   NOT NULL,
    modifier        SMALLINT        NOT NULL,
    PRIMARY KEY (id, stat)
);

CREATE TABLE IF NOT EXISTS CombatUnits (
    id              UUID        REFERENCES Effects(id),
    stat            CombatStat  NOT NULL,
    modifier        SMALLINT    NOT NULL,
    PRIMARY KEY (id, stat)
);

CREATE TABLE IF NOT EXISTS MiscUnits (
    id              UUID    PRIMARY KEY REFERENCES Effects(id) NOT NULL,
    description     TEXT    NOT NULL
);
