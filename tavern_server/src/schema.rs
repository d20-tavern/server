table! {
    armor (id) {
        id -> Uuid,
        material_id -> Nullable<Uuid>,
        max_dex_bonus -> Int4,
        ac -> Int4,
        spell_failure -> Int4,
        check_penalty -> Int4,
        armor_type -> Armor_class,
    }
}

table! {
    attributefeatunits (feat_id, attr) {
        feat_id -> Uuid,
        attr -> Attribute,
        score -> Int2,
    }
}

table! {
    attributeunits (effect_id, attr) {
        effect_id -> Uuid,
        attr -> Attribute,
        modifier -> Int2,
    }
}

table! {
    bags (id) {
        id -> Uuid,
        char_id -> Uuid,
        item_id -> Uuid,
        capacity -> Int4,
    }
}

table! {
    characterequipment (char_id, item_id) {
        char_id -> Uuid,
        item_id -> Uuid,
    }
}

table! {
    characterfeats (char_id, feat_id) {
        char_id -> Uuid,
        feat_id -> Uuid,
    }
}

table! {
    characterfeatures (char_id, feature_id) {
        char_id -> Uuid,
        feature_id -> Uuid,
    }
}

table! {
    characters (id) {
        id -> Uuid,
        user_id -> Uuid,
        race_id -> Uuid,
        deity_id -> Nullable<Uuid>,
        name -> Text,
        age -> Int2,
        gender -> Gender,
        alignment -> Alignment,
        backstory -> Nullable<Text>,
        height -> Nullable<Int2>,
        weight -> Nullable<Int2>,
        size -> Size,
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
    characterspells (char_id, spell_id) {
        char_id -> Uuid,
        spell_id -> Uuid,
        casts_remaining -> Int2,
    }
}

table! {
    charactersubclasses (char_id, subclass_id) {
        char_id -> Uuid,
        subclass_id -> Uuid,
        levels_taken -> Int2,
        hp_taken -> Int2,
        skills_taken -> Int2,
    }
}

table! {
    characterunits (effect_id, stat) {
        effect_id -> Uuid,
        stat -> Character_stat,
        modifier -> Int2,
    }
}

table! {
    classeffects (class_id, effect_id) {
        class_id -> Uuid,
        effect_id -> Uuid,
    }
}

table! {
    classes (id) {
        id -> Uuid,
        description -> Text,
        name -> Text,
        hit_die -> Text,
        starting_wealth -> Text,
        bab_per_level -> Float8,
        skills_per_level -> Int2,
        skills_attr -> Attribute,
    }
}

table! {
    classfeats (class_id, feat_id) {
        class_id -> Uuid,
        feat_id -> Uuid,
    }
}

table! {
    classfeatures (class_id, feature_id) {
        class_id -> Uuid,
        feature_id -> Uuid,
        is_default -> Nullable<Bool>,
    }
}

table! {
    classnotproficientarmor (class_id, armor_id) {
        class_id -> Uuid,
        armor_id -> Uuid,
    }
}

table! {
    classnotproficientweapons (class_id, weapon_id) {
        class_id -> Uuid,
        weapon_id -> Uuid,
    }
}

table! {
    classproficientarmor (class_id, armor_id) {
        class_id -> Uuid,
        armor_id -> Uuid,
    }
}

table! {
    classproficientarmorclasses (class_id) {
        class_id -> Uuid,
        armor_classes -> Array<Armor_class>,
    }
}

table! {
    classproficientweaponclasses (class_id) {
        class_id -> Uuid,
        weapon_classes -> Array<Weapon_class>,
    }
}

table! {
    classproficientweapons (class_id, weapon_id) {
        class_id -> Uuid,
        weapon_id -> Uuid,
    }
}

table! {
    combatunits (effect_id, stat) {
        effect_id -> Uuid,
        stat -> Combat_stat,
        modifier -> Int2,
    }
}

table! {
    deities (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
        favored_animals -> Nullable<Text>,
    }
}

table! {
    deitydomains (deity_id, domain_id) {
        deity_id -> Uuid,
        domain_id -> Uuid,
    }
}

table! {
    deityweapons (deity_id, item_id) {
        deity_id -> Uuid,
        item_id -> Uuid,
    }
}

table! {
    domaineffects (domain_id, effect_id) {
        domain_id -> Uuid,
        effect_id -> Uuid,
    }
}

table! {
    domains (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
        power_description -> Text,
    }
}

table! {
    domainspells (domain_id, spell_id) {
        domain_id -> Uuid,
        spell_id -> Uuid,
        casts -> Int2,
    }
}

table! {
    effects (id) {
        id -> Uuid,
        name -> Text,
        short_description -> Text,
        long_description -> Nullable<Text>,
    }
}

table! {
    feateffects (feat_id, effect_id) {
        feat_id -> Uuid,
        effect_id -> Uuid,
    }
}

table! {
    featrequirements (feat_id, required_feat) {
        feat_id -> Uuid,
        required_feat -> Uuid,
    }
}

table! {
    feats (id) {
        id -> Uuid,
        name -> Text,
        short_description -> Text,
        long_description -> Nullable<Text>,
    }
}

table! {
    featureeffects (feature_id, effect_id) {
        feature_id -> Uuid,
        effect_id -> Uuid,
    }
}

table! {
    features (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
    }
}

table! {
    itemeffects (item_id, effect_id) {
        item_id -> Uuid,
        effect_id -> Uuid,
        is_permanent -> Nullable<Bool>,
    }
}

table! {
    items (id) {
        id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
        cost -> Int4,
        weight -> Float8,
        equip_slot -> Nullable<Equipment_slot>,
    }
}

table! {
    itemsinbags (bag_id, item_id) {
        item_id -> Uuid,
        bag_id -> Uuid,
        count -> Int4,
    }
}

table! {
    materialeffects (material_id, effect_id) {
        material_id -> Uuid,
        effect_id -> Uuid,
    }
}

table! {
    materials (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
        hp_per_inch -> Nullable<Int4>,
        hardness -> Nullable<Int4>,
    }
}

table! {
    miscunits (effect_id) {
        effect_id -> Uuid,
        description -> Text,
    }
}

table! {
    raceeffects (race_id, effect_id) {
        race_id -> Uuid,
        effect_id -> Uuid,
    }
}

table! {
    races (id) {
        id -> Uuid,
        description -> Text,
        type_id -> Uuid,
        subtype_id -> Nullable<Uuid>,
        name -> Text,
        move_speed -> Int2,
        size -> Size,
        languages -> Nullable<Array<Text>>,
    }
}

table! {
    racesubtypeeffects (subtype_id, effect_id) {
        subtype_id -> Uuid,
        effect_id -> Uuid,
    }
}

table! {
    racesubtypes (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
    }
}

table! {
    racetypeeffects (type_id, effect_id) {
        type_id -> Uuid,
        effect_id -> Uuid,
    }
}

table! {
    racetypes (id) {
        id -> Uuid,
        name -> Text,
        hit_die -> Text,
        bab_per_hit_die -> Float4,
    }
}

table! {
    racialfeats (race_id, feat_id) {
        race_id -> Uuid,
        feat_id -> Uuid,
    }
}

table! {
    skillfeatunits (feat_id, skill) {
        feat_id -> Uuid,
        skill -> Skill,
        ranks -> Int2,
    }
}

table! {
    skillunits (effect_id, skill) {
        effect_id -> Uuid,
        skill -> Skill,
        modifier -> Int2,
    }
}

table! {
    spellcomponents (id) {
        id -> Uuid,
        spell_id -> Uuid,
        item_id -> Nullable<Uuid>,
        item_amount -> Nullable<Int2>,
        component_type -> Component_type,
    }
}

table! {
    spelleffects (spell_id, effect_id) {
        spell_id -> Uuid,
        effect_id -> Uuid,
    }
}

table! {
    spells (id) {
        id -> Uuid,
        name -> Text,
        level -> Int2,
        school -> Magic_school,
        casting_time -> Int8,
        range -> Spell_range,
        area -> Text,
        duration_per_level -> Int8,
        saving_throw -> Nullable<Save_throw>,
        spell_resistance -> Nullable<Bool>,
        description -> Text,
    }
}

table! {
    subclasses (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
        class_id -> Uuid,
        caster_type -> Nullable<Caster_type>,
        casting_attr -> Nullable<Attribute>,
    }
}

table! {
    subclassfeatures (subclass_id, feature_id) {
        subclass_id -> Uuid,
        feature_id -> Uuid,
    }
}

table! {
    subclassspells (subclass_id, spell_id) {
        subclass_id -> Uuid,
        spell_id -> Uuid,
        casts -> Int2,
        req_level -> Int2,
    }
}

table! {
    subdomains (id) {
        id -> Uuid,
        domain_id -> Uuid,
        name -> Text,
        description -> Text,
    }
}

table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        username -> Text,
        is_admin -> Nullable<Bool>,
        pass_hash -> Bytea,
        salt -> Bytea,
        time_cost -> Int4,
        memory -> Int4,
        threads -> Int4,
    }
}

table! {
    weapons (id) {
        id -> Uuid,
        material_id -> Nullable<Uuid>,
        crit_range -> Int4range,
        damage -> Array<Text>,
        damage_type -> Array<Damage_type>,
        weapon_type -> Weapon_class,
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
