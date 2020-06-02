table! {
    use diesel::sql_types::*;
    use crate::pathfinder::item::ArmorClassMapping;

    armor (id) {
        id -> Uuid,
        material_id -> Nullable<Uuid>,
        max_dex_bonus -> Int4,
        ac -> Int4,
        spell_failure -> Int4,
        check_penalty -> Int4,
        armor_type -> ArmorClassMapping,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::pathfinder::AttributeMapping;

    attributefeatunits (feat_id, attr) {
        feat_id -> Uuid,
        attr -> AttributeMapping,
        score -> Int2,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::pathfinder::AttributeMapping;

    attributeunits (effect_id, attr) {
        effect_id -> Uuid,
        attr -> AttributeMapping,
        modifier -> Int2,
    }
}

table! {
    use diesel::sql_types::*;

    bags (id) {
        id -> Uuid,
        name -> Text,
        char_id -> Uuid,
        item_id -> Uuid,
        capacity -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    

    characterequipment (char_id, item_id) {
        char_id -> Uuid,
        item_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;

    characterfeats (char_id, feat_id) {
        char_id -> Uuid,
        feat_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;

    characterfeatures (char_id, feature_id) {
        char_id -> Uuid,
        feature_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::pathfinder::{AlignmentMapping, GenderMapping, SizeMapping};

    characters (id) {
        id -> Uuid,
        user_id -> Uuid,
        race_id -> Uuid,
        deity_id -> Nullable<Uuid>,
        name -> Text,
        age -> Int2,
        gender -> GenderMapping,
        alignment -> AlignmentMapping,
        backstory -> Nullable<Text>,
        height -> Nullable<Int2>,
        weight -> Nullable<Int2>,
        size -> SizeMapping,
        strength -> Int2,
        dexterity -> Int2,
        constitution -> Int2,
        intelligence -> Int2,
        wisdom -> Int2,
        charisma -> Int2,
        max_hp -> Int2,
        damage -> Int2,
        nonlethal -> Int2,
        copper -> Int2,
        silver -> Int2,
        gold -> Int2,
        platinum -> Int2,
    }
}

table! {
    use diesel::sql_types::*;

    characterspells (char_id, spell_id) {
        char_id -> Uuid,
        spell_id -> Uuid,
        casts_remaining -> Int2,
    }
}

table! {
    use diesel::sql_types::*;

    charactersubclasses (char_id, subclass_id) {
        char_id -> Uuid,
        subclass_id -> Uuid,
        levels_taken -> Int2,
        hp_taken -> Int2,
        skills_taken -> Int2,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::pathfinder::CharacterStatMapping;

    characterunits (effect_id, stat) {
        effect_id -> Uuid,
        stat -> CharacterStatMapping,
        modifier -> Int2,
    }
}

table! {
    use diesel::sql_types::*;

    classeffects (class_id, effect_id) {
        class_id -> Uuid,
        effect_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::pathfinder::AttributeMapping;

    classes (id) {
        id -> Uuid,
        description -> Text,
        name -> Text,
        hit_die -> Text,
        starting_wealth -> Text,
        bab_per_level -> Float8,
        skills_per_level -> Int2,
        skills_attr -> AttributeMapping,
    }
}

table! {
    use diesel::sql_types::*;

    classfeats (class_id, feat_id) {
        class_id -> Uuid,
        feat_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;

    classfeatures (class_id, feature_id) {
        class_id -> Uuid,
        feature_id -> Uuid,
        is_default -> Nullable<Bool>,
    }
}

table! {
    use diesel::sql_types::*;

    classnotproficientarmor (class_id, armor_id) {
        class_id -> Uuid,
        armor_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;

    classnotproficientweapons (class_id, weapon_id) {
        class_id -> Uuid,
        weapon_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;

    classproficientarmor (class_id, armor_id) {
        class_id -> Uuid,
        armor_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::pathfinder::item::ArmorClassMapping;

    classproficientarmorclasses (class_id) {
        class_id -> Uuid,
        armor_classes -> Array<ArmorClassMapping>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::pathfinder::item::WeaponClassMapping;

    classproficientweaponclasses (class_id) {
        class_id -> Uuid,
        weapon_classes -> Array<WeaponClassMapping>,
    }
}

table! {
    use diesel::sql_types::*;

    classproficientweapons (class_id, weapon_id) {
        class_id -> Uuid,
        weapon_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::pathfinder::CombatStatMapping;

    combatunits (effect_id, stat) {
        effect_id -> Uuid,
        stat -> CombatStatMapping,
        modifier -> Int2,
    }
}

table! {
    use diesel::sql_types::*;

    deities (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
        favored_animals -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;

    deitydomains (deity_id, domain_id) {
        deity_id -> Uuid,
        domain_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;

    deityweapons (deity_id, item_id) {
        deity_id -> Uuid,
        item_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;

    domaineffects (domain_id, effect_id) {
        domain_id -> Uuid,
        effect_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;

    domains (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
        power_description -> Text,
    }
}

table! {
    use diesel::sql_types::*;

    domainspells (domain_id, spell_id) {
        domain_id -> Uuid,
        spell_id -> Uuid,
        casts -> Int2,
    }
}

table! {
    use diesel::sql_types::*;

    effects (id) {
        id -> Uuid,
        name -> Text,
        short_description -> Text,
        long_description -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;

    feateffects (feat_id, effect_id) {
        feat_id -> Uuid,
        effect_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;

    featrequirements (feat_id, required_feat) {
        feat_id -> Uuid,
        required_feat -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;

    feats (id) {
        id -> Uuid,
        name -> Text,
        short_description -> Text,
        long_description -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;

    featureeffects (feature_id, effect_id) {
        feature_id -> Uuid,
        effect_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;

    features (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
    }
}

table! {
    use diesel::sql_types::*;

    itemeffects (item_id, effect_id) {
        item_id -> Uuid,
        effect_id -> Uuid,
        is_permanent -> Bool,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::pathfinder::EquipmentSlotMapping;

    items (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
        cost -> Int4,
        weight -> Float8,
        equip_slot -> Nullable<EquipmentSlotMapping>,
    }
}

table! {
    use diesel::sql_types::*;

    itemsinbags (bag_id, item_id) {
        item_id -> Uuid,
        bag_id -> Uuid,
        count -> Int4,
    }
}

table! {
    use diesel::sql_types::*;

    materialeffects (material_id, effect_id) {
        material_id -> Uuid,
        effect_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;

    materials (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
        hp_per_inch -> Nullable<Int4>,
        hardness -> Nullable<Int4>,
    }
}

table! {
    use diesel::sql_types::*;

    miscunits (effect_id) {
        effect_id -> Uuid,
        description -> Text,
    }
}

table! {
    use diesel::sql_types::*;

    raceeffects (race_id, effect_id) {
        race_id -> Uuid,
        effect_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::pathfinder::SizeMapping;

    races (id) {
        id -> Uuid,
        description -> Text,
        type_id -> Uuid,
        subtype_id -> Nullable<Uuid>,
        name -> Text,
        move_speed -> Int2,
        size -> SizeMapping,
        languages -> Nullable<Array<Text>>,
    }
}

table! {
    use diesel::sql_types::*;

    racesubtypeeffects (subtype_id, effect_id) {
        subtype_id -> Uuid,
        effect_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;

    racesubtypes (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
    }
}

table! {
    use diesel::sql_types::*;

    racetypeeffects (type_id, effect_id) {
        type_id -> Uuid,
        effect_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;

    racetypes (id) {
        id -> Uuid,
        name -> Text,
        hit_die -> Text,
        bab_per_hit_die -> Float4,
    }
}

table! {
    use diesel::sql_types::*;

    racialfeats (race_id, feat_id) {
        race_id -> Uuid,
        feat_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::pathfinder::SkillMapping;

    skillfeatunits (feat_id, skill) {
        feat_id -> Uuid,
        skill -> SkillMapping,
        ranks -> Int2,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::pathfinder::SkillMapping;

    skillunits (effect_id, skill) {
        effect_id -> Uuid,
        skill -> SkillMapping,
        modifier -> Int2,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::pathfinder::spell::ComponentTypeMapping;

    spellcomponents (id) {
        id -> Uuid,
        spell_id -> Uuid,
        item_id -> Nullable<Uuid>,
        item_amount -> Nullable<Int2>,
        component_type -> ComponentTypeMapping,
    }
}

table! {
    use diesel::sql_types::*;

    spelleffects (spell_id, effect_id) {
        spell_id -> Uuid,
        effect_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::pathfinder::SaveThrowMapping;
    use crate::pathfinder::spell::MagicSchoolMapping;
    use crate::pathfinder::spell::SpellRangeMapping;

    spells (id) {
        id -> Uuid,
        name -> Text,
        level -> Int2,
        school -> MagicSchoolMapping,
        casting_time -> Int8,
        range -> SpellRangeMapping,
        area -> Text,
        duration_per_level -> Int8,
        saving_throw -> Nullable<SaveThrowMapping>,
        spell_resistance -> Bool,
        description -> Text,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::pathfinder::AttributeMapping;
    use crate::pathfinder::spell::CasterTypeMapping;

    subclasses (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
        class_id -> Uuid,
        caster_type -> Nullable<CasterTypeMapping>,
        casting_attr -> Nullable<AttributeMapping>,
    }
}

table! {
    use diesel::sql_types::*;

    subclassfeatures (subclass_id, feature_id) {
        subclass_id -> Uuid,
        feature_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;

    subclassspells (subclass_id, spell_id) {
        subclass_id -> Uuid,
        spell_id -> Uuid,
        casts -> Int2,
        req_level -> Int2,
    }
}

table! {
    use diesel::sql_types::*;

    subdomains (id) {
        id -> Uuid,
        domain_id -> Uuid,
        name -> Text,
        description -> Text,
    }
}

table! {
    use diesel::sql_types::*;

    users (id) {
        id -> Uuid,
        email -> Text,
        username -> Text,
        is_admin -> Bool,
        pass_hash -> Bytea,
        salt -> Bytea,
        time_cost -> Int4,
        memory -> Int4,
        threads -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::pathfinder::DamageTypeMapping;
    use crate::pathfinder::item::WeaponClassMapping;

    weapons (id) {
        id -> Uuid,
        material_id -> Nullable<Uuid>,
        crit_range -> Int4range,
        damage -> Array<Text>,
        damage_type -> Array<DamageTypeMapping>,
        weapon_type -> WeaponClassMapping,
    }
}

joinable!(armor -> items (id));
joinable!(armor -> materials (material_id));
joinable!(attributefeatunits -> feats (feat_id));
joinable!(attributeunits -> effects (effect_id));
joinable!(bags -> characters (char_id));
joinable!(bags -> items (item_id));
joinable!(characterequipment -> characters (char_id));
joinable!(characterequipment -> items (item_id));
joinable!(characterfeats -> characters (char_id));
joinable!(characterfeats -> feats (feat_id));
joinable!(characterfeatures -> characters (char_id));
joinable!(characterfeatures -> features (feature_id));
joinable!(characters -> deities (deity_id));
joinable!(characters -> races (race_id));
joinable!(characters -> users (user_id));
joinable!(characterspells -> characters (char_id));
joinable!(characterspells -> spells (spell_id));
joinable!(charactersubclasses -> characters (char_id));
joinable!(charactersubclasses -> subclasses (subclass_id));
joinable!(characterunits -> effects (effect_id));
joinable!(classeffects -> classes (class_id));
joinable!(classeffects -> effects (effect_id));
joinable!(classfeats -> classes (class_id));
joinable!(classfeats -> feats (feat_id));
joinable!(classfeatures -> feats (class_id));
joinable!(classfeatures -> features (feature_id));
joinable!(classnotproficientarmor -> armor (armor_id));
joinable!(classnotproficientarmor -> classes (class_id));
joinable!(classnotproficientweapons -> classes (class_id));
joinable!(classnotproficientweapons -> weapons (weapon_id));
joinable!(classproficientarmor -> armor (armor_id));
joinable!(classproficientarmor -> classes (class_id));
joinable!(classproficientarmorclasses -> classes (class_id));
joinable!(classproficientweaponclasses -> classes (class_id));
joinable!(classproficientweapons -> classes (class_id));
joinable!(classproficientweapons -> weapons (weapon_id));
joinable!(combatunits -> effects (effect_id));
joinable!(deitydomains -> deities (deity_id));
joinable!(deitydomains -> domains (domain_id));
joinable!(deityweapons -> deities (deity_id));
joinable!(deityweapons -> weapons (item_id));
joinable!(domaineffects -> domains (domain_id));
joinable!(domaineffects -> effects (effect_id));
joinable!(domainspells -> domains (domain_id));
joinable!(domainspells -> spells (spell_id));
joinable!(feateffects -> effects (effect_id));
joinable!(feateffects -> feats (feat_id));
joinable!(featureeffects -> effects (effect_id));
joinable!(featureeffects -> features (feature_id));
joinable!(itemeffects -> effects (effect_id));
joinable!(itemeffects -> items (item_id));
joinable!(itemsinbags -> bags (bag_id));
joinable!(itemsinbags -> items (item_id));
joinable!(materialeffects -> effects (effect_id));
joinable!(materialeffects -> materials (material_id));
joinable!(miscunits -> effects (effect_id));
joinable!(raceeffects -> effects (effect_id));
joinable!(raceeffects -> races (race_id));
joinable!(races -> racesubtypes (subtype_id));
joinable!(races -> racetypes (type_id));
joinable!(racesubtypeeffects -> effects (effect_id));
joinable!(racesubtypeeffects -> racesubtypes (subtype_id));
joinable!(racetypeeffects -> effects (effect_id));
joinable!(racetypeeffects -> racetypes (type_id));
joinable!(racialfeats -> feats (feat_id));
joinable!(racialfeats -> races (race_id));
joinable!(skillfeatunits -> feats (feat_id));
joinable!(skillunits -> effects (effect_id));
joinable!(spellcomponents -> items (item_id));
joinable!(spellcomponents -> spells (spell_id));
joinable!(spelleffects -> effects (effect_id));
joinable!(spelleffects -> spells (spell_id));
joinable!(subclasses -> classes (class_id));
joinable!(subclassfeatures -> features (feature_id));
joinable!(subclassfeatures -> subclasses (subclass_id));
joinable!(subclassspells -> spells (spell_id));
joinable!(subclassspells -> subclasses (subclass_id));
joinable!(subdomains -> domains (domain_id));
joinable!(weapons -> items (id));
joinable!(weapons -> materials (material_id));

allow_tables_to_appear_in_same_query!(
    armor,
    attributefeatunits,
    attributeunits,
    bags,
    characterequipment,
    characterfeats,
    characterfeatures,
    characters,
    characterspells,
    charactersubclasses,
    characterunits,
    classeffects,
    classes,
    classfeats,
    classfeatures,
    classnotproficientarmor,
    classnotproficientweapons,
    classproficientarmor,
    classproficientarmorclasses,
    classproficientweaponclasses,
    classproficientweapons,
    combatunits,
    deities,
    deitydomains,
    deityweapons,
    domaineffects,
    domains,
    domainspells,
    effects,
    feateffects,
    featrequirements,
    feats,
    featureeffects,
    features,
    itemeffects,
    items,
    itemsinbags,
    materialeffects,
    materials,
    miscunits,
    raceeffects,
    races,
    racesubtypeeffects,
    racesubtypes,
    racetypeeffects,
    racetypes,
    racialfeats,
    skillfeatunits,
    skillunits,
    spellcomponents,
    spelleffects,
    spells,
    subclasses,
    subclassfeatures,
    subclassspells,
    subdomains,
    users,
    weapons,
);
