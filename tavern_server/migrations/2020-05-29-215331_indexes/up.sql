CREATE INDEX user_id ON Users (id);
CREATE INDEX user_username ON Users (username);
CREATE INDEX race_type_id ON RaceTypes (id);
CREATE INDEX race_subtype_id ON RaceSubtypes (id);
CREATE INDEX race_id ON Races (id);
CREATE INDEX deity_id ON Deities (id);
CREATE INDEX character_id ON Characters (id);
CREATE INDEX class_id ON Classes (id);
CREATE INDEX subclass_id ON Subclasses (id);
CREATE INDEX character_subclass_char_id ON CharacterSubclasses (char_id);
CREATE INDEX feat_id ON Feats (id);
CREATE INDEX skill_feat_unit_feat_id ON SkillFeatUnits (feat_id);
CREATE INDEX attribute_feat_unit_feat_id ON AttributeFeatUnits (feat_id);
CREATE INDEX feat_requirement_feat_id ON FeatRequirements (feat_id);
CREATE INDEX character_feat_char_id ON CharacterFeats (char_id);
CREATE INDEX racial_feat_race_id ON RacialFeats (race_id);
CREATE INDEX class_feat_class_id ON ClassFeats (class_id);
CREATE INDEX feature_id ON Features (id);
CREATE INDEX class_feature_class_id ON ClassFeatures (class_id);
CREATE INDEX character_feature_char_id ON CharacterFeatures (char_id);
CREATE INDEX subclass_feature_class_id ON SubclassFeatures (subclass_id);
CREATE INDEX item_id ON Items (id);
CREATE INDEX material_id ON Materials (id);
CREATE INDEX weapon_id ON Weapons (id);
CREATE INDEX armor_id ON Armor (id);
CREATE INDEX class_proficient_armor_class_class_id ON ClassProficientArmorClasses (class_id);
CREATE INDEX class_proficient_armor_class_id ON ClassProficientArmor (class_id);
CREATE INDEX class_not_proficient_armor_class_id ON ClassNotProficientArmor (class_id);
CREATE INDEX class_proficient_weapon_class_class_id ON ClassProficientWeaponClasses (class_id);
CREATE INDEX class_proficient_weapon_class_id ON ClassProficientWeapons (class_id);
CREATE INDEX class_not_proficient_weapon_class_id ON ClassNotProficientWeapons (class_id);
CREATE INDEX bag_id ON Bags (id);
CREATE INDEX bag_char_id ON Bags (char_id);
CREATE INDEX item_in_bag_bag_id ON ItemsInBags (bag_id);
CREATE INDEX character_equipment_char_id ON CharacterEquipment (char_id);
CREATE INDEX spell_id ON Spells (id);
CREATE INDEX character_spell_char_id ON CharacterSpells (char_id);
CREATE INDEX subclass_spell_subclass_id ON SubclassSpells (subclass_id);
CREATE INDEX domain_id ON Domains (id);
CREATE INDEX domain_spell_id ON DomainSpells (domain_id);
CREATE INDEX spell_component_id ON SpellComponents (id);
CREATE INDEX spell_component_spell_id ON SpellComponents (spell_id);
CREATE INDEX subdomain_id ON Subdomains (id);
CREATE INDEX subdomain_domain_id ON Subdomains (domain_id);
CREATE INDEX deity_domain_deity_id ON DeityDomains (deity_id);
CREATE INDEX deity_domain_weapon_id ON DeityWeapons (deity_id);
CREATE INDEX effect_id ON Effects (id);
CREATE INDEX race_effect_race_id ON RaceEffects (race_id);
CREATE INDEX race_type_effect_type_id ON RaceTypeEffects (type_id);
CREATE INDEX race_subtype_effect_subtype_id ON RaceSubtypeEffects (subtype_id);
CREATE INDEX class_effect_class_id ON ClassEffects (class_id);
CREATE INDEX item_effect_item_id ON ItemEffects (item_id);
CREATE INDEX spell_effect_spell_id ON SpellEffects (spell_id);
CREATE INDEX feat_effect_feat_id ON FeatEffects (feat_id);
CREATE INDEX feature_effect_feature_id ON FeatureEffects (feature_id);
CREATE INDEX domain_effect_domain_id ON DomainEffects (domain_id);
CREATE INDEX material_effect_material_id ON MaterialEffects (material_id);
CREATE INDEX attribute_unit_effect_id ON AttributeUnits (effect_id);
CREATE INDEX skill_unit_effect_id ON SkillUnits (effect_id);
CREATE INDEX character_unit_effect_id ON CharacterUnits (effect_id);
CREATE INDEX combat_unit_effect_id ON CombatUnits (effect_id);
CREATE INDEX misc_unit_effect_id ON MiscUnits (effect_id);