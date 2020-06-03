CREATE TYPE gender AS ENUM (
    'male',
    'female',
    'other'
);

CREATE TYPE size AS ENUM (
    'fine',
    'diminutive',
    'tiny',
    'small',
    'medium',
    'large',
    'huge',
    'gargantuan',
    'colossal'
);

CREATE TYPE alignment AS ENUM (
    'lawful_good',
    'lawful_neutral',
    'lawful_evil',
    'neutral_good',
    'true_neutral',
    'neutral_evil',
    'chaotic_good',
    'chaotic_neutral',
    'chaotic_evil'
);

CREATE TYPE attribute AS ENUM (
    'strength',
    'dexterity',
    'constitution',
    'intelligence',
    'wisdom',
    'charisma'
);

CREATE TYPE skill AS ENUM (
    'acrobatics',
    'appraise',
    'bluff',
    'climb',
    'craft',
    'diplomacy',
    'disable_device',
    'disguise',
    'escape_artist',
    'fly',
    'handle_animal',
    'heal',
    'intimidate',
    'knowledge_arcana',
    'knowledge_dungeoneering',
    'knowledge_engineering',
    'knowledge_geography',
    'knowledge_history',
    'knowledge_local',
    'knowledge_nobility',
    'knowledge_planes',
    'knowledge_religion',
    'linguistics',
    'perception',
    'perform',
    'profession',
    'ride',
    'sense_motive',
    'sleight_of_hand',
    'spellcraft',
    'stealth',
    'survival',
    'swim',
    'use_magic_device'
);

CREATE TYPE save_throw AS ENUM (
    'fortitude',
    'reflex',
    'will'
);

CREATE TYPE character_stat AS ENUM (
    'name',
    'race',
    'size',
    'height',
    'weight',
    'age',
    'gender',
    'alignment',
    'deity',
    'languages',
    'appearance'
);

CREATE TYPE combat_stat AS ENUM (
    'melee_attack_bonus',
    'ranged_attack_bonus',
    'cmb',
    'cmd',
    'armor_class',
    'touch_ac',
    'flat_footed_ac',
    'initiative_bonus',
    'damage_reduction',
    'spell_resistance',
    'speed',
    'fortitude',
    'reflex',
    'will'
);

CREATE TYPE caster_type AS ENUM (
    'spontaneous',
    'prepared'
);

CREATE TYPE component_type AS ENUM (
    'somatic',
    'material',
    'verbal'
);

CREATE TYPE magic_school AS ENUM (
    'abjuration',
    'conjuration',
    'divination',
    'enchantment',
    'evocation',
    'illusion',
    'necromancy',
    'transmutation'
);

CREATE TYPE spell_range AS ENUM (
    'personal',
    'touch',
    'close',
    'medium',
    'long',
    'unlimited'
);

CREATE TYPE weapon_class AS ENUM (
    'axes',
    'heavy_blades',
    'light_blades',
    'bows',
    'close',
    'crossbows',
    'double',
    'firearms',
    'flails',
    'hammers',
    'monk',
    'natural',
    'polearms',
    'siege_engines',
    'spears',
    'thrown',
    'tribal'
);

CREATE TYPE armor_class AS ENUM (
    'light',
    'medium',
    'heavy'
);

CREATE TYPE equipment_slot AS ENUM (
    'armor',
    'belts',
    'body',
    'chest',
    'eyes',
    'feet',
    'hands',
    'head',
    'headband',
    'neck',
    'ring_left',
    'ring_right',
    'shield',
    'shoulders',
    'wrist'
);

CREATE TYPE damage_type AS ENUM (
    'bludgeoning',
    'slashing',
    'piercing',
    'energy',
    'acid',
    'fire',
    'electricity',
    'cold',
    'sonic',
    'positive',
    'negative',
    'nonlethal'
);

--table declarations
--user creds table - also used for hashing info
CREATE TABLE Users (
    id          UUID    PRIMARY KEY,
    email       TEXT    CONSTRAINT user_email_required    NOT NULL CONSTRAINT user_email_unique    UNIQUE,
    username    TEXT    CONSTRAINT user_username_required NOT NULL CONSTRAINT user_username_unique UNIQUE,
    is_admin    BOOLEAN NOT NULL DEFAULT false,
    pass_hash   BYTEA   NOT NULL,
    salt        BYTEA   NOT NULL,
    time_cost   INT     NOT NULL CHECK (time_cost > 0),
    memory      INT     NOT NULL CHECK (memory > 0),
    threads     INT     NOT NULL CHECK (threads > 0)
);

CREATE TABLE RaceTypes (
    id                  UUID    PRIMARY KEY,
    name                TEXT    NOT NULL,
    hit_die             TEXT    NOT NULL,
    bab_per_hit_die     REAL    NOT NULL
);

CREATE TABLE RaceSubtypes (
    id              UUID    PRIMARY KEY,
    name            TEXT    NOT NULL,
    description     TEXT    NOT NULL
);

CREATE TABLE Races (
    id              UUID        PRIMARY KEY,
    description     TEXT        NOT NULL,
    type_id         UUID        REFERENCES RaceTypes(id) NOT NULL,
    subtype_id      UUID        REFERENCES RaceSubtypes(id),
    name            TEXT        NOT NULL,
    move_speed      SMALLINT    NOT NULL CHECK (move_speed > 0),
    size            Size        NOT NULL,
    languages       TEXT[]      NOT NULL
);

--Deity tables
CREATE TABLE Deities (
    id                  UUID    PRIMARY KEY,
    name                TEXT    NOT NULL,
    description         TEXT    NOT NULL,
    favored_animals     TEXT
);

--Character overview table
CREATE TABLE Characters (
    id              UUID        PRIMARY KEY,
    user_id         UUID        REFERENCES Users(id) NOT NULL,
    race_id         UUID        REFERENCES Races(id) NOT NULL,
    deity_id        UUID        REFERENCES Deities(id),
    name            TEXT        NOT NULL,
    age             SMALLINT    NOT NULL CHECK (age > 0),
    gender          gender      NOT NULL,
    alignment       alignment   NOT NULL,
    backstory       TEXT        NOT NULL,
    height          SMALLINT    NOT NULL CHECK (height > 0),
    weight          SMALLINT    NOT NULL CHECK (weight > 0),
    size            size        NOT NULL,

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
CREATE TABLE Classes (
    id                  UUID                PRIMARY KEY,
    description         TEXT                NOT NULL,
    name                TEXT                NOT NULL,
    hit_die             TEXT                NOT NULL,
    starting_wealth     TEXT                NOT NULL,
    bab_per_level       DOUBLE PRECISION    NOT NULL,
    skills_per_level    SMALLINT            NOT NULL,
    skills_attr         attribute           NOT NULL
);

CREATE TABLE Subclasses (
    id              UUID        PRIMARY KEY,
    name            TEXT        NOT NULL UNIQUE,
    description     TEXT        NOT NULL,
    class_id        UUID        REFERENCES Classes(id) NOT NULL,
    caster_type     caster_type,
    casting_attr    attribute
);

CREATE TABLE CharacterSubclasses (
    char_id         UUID        REFERENCES Characters(id) NOT NULL,
    subclass_id     UUID        REFERENCES Subclasses(id) NOT NULL,
    levels_taken    SMALLINT    NOT NULL CHECK (levels_taken > 0),
    hp_taken        SMALLINT    NOT NULL CHECK (hp_taken >= 0),
    skills_taken    SMALLINT    NOT NULL CHECK (skills_taken >= 0),
    PRIMARY KEY(char_id, subclass_id)
);

--Feats tables
CREATE TABLE Feats (
    id                  UUID    PRIMARY KEY,
    name                TEXT    UNIQUE NOT NULL,
    short_description   TEXT    NOT NULL,
    long_description    TEXT
);

CREATE TABLE SkillFeatUnits (
    feat_id     UUID        REFERENCES Feats(id) NOT NULL,
    skill       skill       NOT NULL,
    ranks       SMALLINT    NOT NULL,
    PRIMARY KEY (feat_id, skill)
);

CREATE TABLE AttributeFeatUnits (
    feat_id         UUID        REFERENCES Feats(id) NOT NULL,
    attr            attribute   NOT NULL,
    score           SMALLINT    NOT NULL CHECK(score != 0),
    PRIMARY KEY (feat_id, attr)
);

--this table creates a many to many relationship internally between feats.
--any given feat can require a number of other feats to be taken,
--and a given feat can be required by any number of other feats.
--this table accomplishes that.

-- TODO: Ensure there are no cycles before inserting
-- This may help: https://stackoverflow.com/questions/26671612/prevent-and-or-detect-cycles-in-postgres
CREATE TABLE FeatRequirements (
    feat_id         UUID    REFERENCES Feats(id) NOT NULL,
    required_feat   UUID    REFERENCES Feats(id) NOT NULL,
    CHECK (feat_id != required_feat),
    PRIMARY KEY(feat_id, required_feat)
);

CREATE TABLE CharacterFeats (
    char_id     UUID    REFERENCES Characters(id),
    feat_id     UUID    REFERENCES Feats(id),
    PRIMARY KEY (char_id, feat_id)
);

CREATE TABLE RacialFeats (
    race_id     UUID    REFERENCES Races(id),
    feat_id     UUID    REFERENCES Feats(id),
    PRIMARY KEY (race_id, feat_id)
);

CREATE TABLE ClassFeats (
    class_id    UUID    REFERENCES Classes(id),
    feat_id     UUID    REFERENCES Feats(id),
    PRIMARY KEY (class_id, feat_id)
);

--Features tables
CREATE TABLE Features (
    id              UUID    PRIMARY KEY,
    name            TEXT    NOT NULL UNIQUE,
    description     TEXT    NOT NULL
);

CREATE TABLE ClassFeatures (
    class_id    UUID        REFERENCES Feats(id) NOT NULL,
    feature_id  UUID        REFERENCES Features(id) NOT NULL,
    is_default  BOOLEAN     DEFAULT false,
    PRIMARY KEY(class_id, feature_id)
);

CREATE TABLE CharacterFeatures (
    char_id     UUID        REFERENCES Characters(id) NOT NULL,
    feature_id  UUID        REFERENCES Features(id) NOT NULL,
    PRIMARY KEY(char_id, feature_id)
);

CREATE TABLE SubclassFeatures (
    subclass_id     UUID    REFERENCES Subclasses(id) NOT NULL,
    feature_id      UUID    REFERENCES Features(id) NOT NULL,
    PRIMARY KEY(subclass_id, feature_id)
);

--Item tables
CREATE TABLE Items (
    id              UUID    PRIMARY KEY,
    name            TEXT    NOT NULL,
    description     TEXT    NOT NULL,
    cost            INT     NOT NULL CHECK (cost >= 0), -- prices are in copper
    weight          FLOAT8  NOT NULL CHECK (weight >=0),
    equip_slot      equipment_slot
);

--Materials Tables
CREATE TABLE Materials (
    id              UUID    PRIMARY KEY,
    name            TEXT    UNIQUE NOT NULL,
    description     TEXT    NOT NULL,
    --both of these are not universal for materials, as far as I can tell.
    hp_per_inch     INT,
    hardness        INT
);

CREATE TABLE Weapons (
    id              UUID            PRIMARY KEY REFERENCES Items(id),
    material_id     UUID            REFERENCES Materials(id),
    crit_range      INT4RANGE       NOT NULL,
    damage          TEXT[]          NOT NULL,
    damage_type     damage_type[]    NOT NULL,
    weapon_type     weapon_class     NOT NULL
);

CREATE TABLE Armor (
    id              UUID        PRIMARY KEY REFERENCES Items(id),
    material_id     UUID        REFERENCES Materials(id),
    max_dex_bonus   INT         NOT NULL CHECK (max_dex_bonus >= 0),
    ac              INT         NOT NULL CHECK (ac >= 0),
    spell_failure   INT         NOT NULL CHECK (spell_failure >= 0),
    check_penalty   INT         NOT NULL CHECK (check_penalty >= 0),
    armor_type      armor_class  NOT NULL
);

--class proficiencies tables
CREATE TABLE ClassProficientArmorClasses (
    class_id        UUID            PRIMARY KEY REFERENCES Classes(id),
    armor_classes   armor_class[]    NOT NULL
);

CREATE TABLE ClassProficientArmor (
    class_id    UUID    REFERENCES Classes(id),
    armor_id    UUID    REFERENCES Armor(id),
    PRIMARY KEY (class_id, armor_id)
);

CREATE TABLE ClassNotProficientArmor (
    class_id    UUID    REFERENCES Classes(id),
    armor_id    UUID    REFERENCES Armor(id),
    PRIMARY KEY (class_id, armor_id)
);

CREATE TABLE ClassProficientWeaponClasses (
    class_id        UUID            PRIMARY KEY REFERENCES Classes(id),
    weapon_classes  weapon_class[]   NOT NULL
);

CREATE TABLE ClassProficientWeapons (
    class_id    UUID    REFERENCES Classes(id),
    weapon_id   UUID    REFERENCES Weapons(id),
    PRIMARY KEY (class_id, weapon_id)
);

CREATE TABLE ClassNotProficientWeapons (
    class_id    UUID    REFERENCES Classes(id),
    weapon_id   UUID    REFERENCES Weapons(id),
    PRIMARY KEY (class_id, weapon_id)
);

CREATE TABLE Bags (
    id          UUID    PRIMARY KEY,
    name        TEXT    NOT NULL,
    char_id     UUID    REFERENCES Characters(id) NOT NULL,
    item_id     UUID    REFERENCES Items(id) NOT NULL,
    capacity    INT     NOT NULL CHECK (capacity > 0)
);

CREATE TABLE ItemsInBags (
    item_id     UUID    REFERENCES Items(id) NOT NULL,
    bag_id      UUID    REFERENCES Bags(id) NOT NULL,
    count       INT     NOT NULL CHECK (count > 0),
    PRIMARY KEY(bag_id, item_id)
);

CREATE TABLE CharacterEquipment (
    char_id     UUID    REFERENCES Characters(id) NOT NULL,
    item_id     UUID    REFERENCES Items(id) NOT NULL,
    PRIMARY KEY(char_id, item_id)
);

--Spells tables
CREATE TABLE Spells (
    id                  UUID            PRIMARY KEY,
    name                TEXT            NOT NULL,
    level               SMALLINT        NOT NULL CHECK (level >= 0),
    school              magic_school    NOT NULL,
    casting_time        BIGINT          NOT NULL CHECK (casting_time >= 0),
    range               spell_range     NOT NULL,
    area                TEXT            NOT NULL,
    duration_per_level  BIGINT          NOT NULL CHECK (duration_per_level > 0),
    saving_throw        save_throw,
    spell_resistance    BOOLEAN         NOT NULL DEFAULT false,
    description         TEXT            NOT NULL
);

CREATE TABLE CharacterSpells (
    char_id             UUID        REFERENCES Characters(id) NOT NULL,
    spell_id            UUID        REFERENCES Spells(id) NOT NULL,
    casts_remaining     SMALLINT    NOT NULL CHECK (casts_remaining >= 0),
    PRIMARY KEY(char_id, spell_id)
);

CREATE TABLE SubclassSpells (
    subclass_id     UUID        REFERENCES Subclasses(id),
    spell_id        UUID        REFERENCES Spells(id),
    casts           SMALLINT    NOT NULL CHECK (casts >= 0),
    req_level       SMALLINT    NOT NULL CHECK (req_level >= 0),
    PRIMARY KEY(subclass_id, spell_id)
);

--Domain tables
CREATE TABLE Domains (
    id                  UUID    PRIMARY KEY,
    name                TEXT    NOT NULL,
    description         TEXT    NOT NULL,
    power_description   TEXT    NOT NULL
);

CREATE TABLE DomainSpells (
    domain_id   UUID        REFERENCES Domains(id),
    spell_id    UUID        REFERENCES Spells(id),
    casts       SMALLINT    NOT NULL CHECK (casts >= 0),
    PRIMARY KEY(domain_id, spell_id)
);

CREATE TABLE SpellComponents (
    id              UUID            PRIMARY KEY,
    spell_id        UUID            REFERENCES Spells(id) NOT NULL,
    item_id         UUID            REFERENCES Items(id),
    item_amount     SMALLINT        CHECK (item_amount >= 0),
    component_type  component_type  NOT NULL,
    CONSTRAINT component_item_nullness CHECK ((item_id IS NULL AND item_amount IS NULL) or (item_id IS NOT NULL AND item_amount IS NOT NULL))
);

CREATE TABLE Subdomains (
    id              UUID    PRIMARY KEY,
    domain_id       UUID    REFERENCES Domains(id) NOT NULL,
    name            TEXT    NOT NULL,
    description     TEXT    NOT NULL
);

CREATE TABLE DeityDomains (
    deity_id    UUID    REFERENCES Deities(id) NOT NULL,
    domain_id   UUID    REFERENCES Domains(id) NOT NULL,
    PRIMARY KEY(deity_id, domain_id)
);

CREATE TABLE DeityWeapons (
    deity_id    UUID    REFERENCES Deities(id),
    item_id     UUID    REFERENCES Weapons(id),
    PRIMARY KEY (deity_id, item_id)
);

--Effects tables
CREATE TABLE Effects (
    id                  UUID    PRIMARY KEY,
    name                TEXT    UNIQUE NOT NULL,
    short_description   TEXT    NOT NULL,
    long_description    TEXT
);

CREATE TABLE RaceEffects (
    race_id     UUID    REFERENCES Races(id) NOT NULL,
    effect_id   UUID    REFERENCES Effects(id) NOT NULL,
    PRIMARY KEY(race_id, effect_id)
);

CREATE TABLE RaceTypeEffects (
    type_id     UUID    REFERENCES RaceTypes(id) NOT NULL,
    effect_id   UUID    REFERENCES Effects(id) NOT NULL,
    PRIMARY KEY(type_id, effect_id)
);

CREATE TABLE RaceSubtypeEffects (
    subtype_id  UUID    REFERENCES RaceSubtypes(id) NOT NULL,
    effect_id   UUID    REFERENCES Effects(id) NOT NULL,
    PRIMARY KEY(subtype_id, effect_id)
);

CREATE TABLE ClassEffects (
    class_id    UUID    REFERENCES Classes(id) NOT NULL,
    effect_id   UUID    REFERENCES Effects(id) NOT NULL,
    PRIMARY KEY(class_id, effect_id)
);

CREATE TABLE ItemEffects (
    item_id         UUID        REFERENCES Items(id) NOT NULL,
    effect_id       UUID        REFERENCES Effects(id) NOT NULL,
    is_permanent    BOOLEAN     NOT NULL DEFAULT false,
    PRIMARY KEY(item_id, effect_id)
);

CREATE TABLE SpellEffects (
    spell_id        UUID    REFERENCES Spells(id) NOT NULL,
    effect_id       UUID    REFERENCES Effects(id) NOT NULL,
    PRIMARY KEY(spell_id, effect_id)
);

CREATE TABLE FeatEffects (
    feat_id     UUID    REFERENCES Feats(id) NOT NULL,
    effect_id   UUID    REFERENCES Effects(id) NOT NULL,
    PRIMARY KEY(feat_id, effect_id)
);

CREATE TABLE FeatureEffects (
    feature_id  UUID    REFERENCES Features(id) NOT NULL,
    effect_id   UUID    REFERENCES Effects(id) NOT NULL,
    PRIMARY KEY(feature_id, effect_id)
);

CREATE TABLE DomainEffects (
    domain_id   UUID    REFERENCES Domains(id) NOT NULL,
    effect_id   UUID    REFERENCES Effects(id) NOT NULL,
    PRIMARY KEY(domain_id, effect_id)
);

CREATE TABLE MaterialEffects (
    material_id     UUID REFERENCES Materials(id) NOT NULL,
    effect_id       UUID REFERENCES Effects(id) NOT NULL,
    PRIMARY KEY(material_id, effect_id)
);

CREATE TABLE AttributeUnits (
    effect_id       UUID        REFERENCES Effects(id),
    attr            attribute   NOT NULL,
    modifier        SMALLINT    NOT NULL,
    PRIMARY KEY (effect_id, attr)
);

CREATE TABLE SkillUnits (
    effect_id       UUID        REFERENCES Effects(id),
    skill           skill       NOT NULL,
    modifier        SMALLINT    NOT NULL,
    PRIMARY KEY (effect_id, skill)
);

CREATE TABLE CharacterUnits (
    effect_id       UUID        REFERENCES Effects(id),
    stat            character_stat   NOT NULL,
    modifier        SMALLINT        NOT NULL,
    PRIMARY KEY (effect_id, stat)
);

CREATE TABLE CombatUnits (
    effect_id       UUID        REFERENCES Effects(id),
    stat            combat_stat  NOT NULL,
    modifier        SMALLINT    NOT NULL,
    PRIMARY KEY (effect_id, stat)
);

CREATE TABLE MiscUnits (
    effect_id       UUID    PRIMARY KEY REFERENCES Effects(id),
    description     TEXT    NOT NULL
);
