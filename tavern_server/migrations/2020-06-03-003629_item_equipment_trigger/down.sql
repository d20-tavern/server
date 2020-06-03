-- This file should undo anything in `up.sql`
DROP TRIGGER character_equipment_consistency ON CharacterEquipment;
DROP FUNCTION validate_character_equipment;