-- Your SQL goes here
-- This trigger ensures that items inserted into CharacterEquipment
-- (a) have an equipment slot
-- (b) do not conflict with another item for the same character
--     (i.e. have the same slot)

CREATE FUNCTION validate_character_equipment()
    RETURNS trigger AS
$$
DECLARE
    slot equipment_slot;
    row RECORD;
BEGIN
    SELECT Items.equip_slot INTO slot FROM Items WHERE Items.id = NEW.item_id LIMIT 1;
    IF slot IS NULL THEN
        RAISE EXCEPTION 'Item does not have equipment slot %', item_id
            USING HINT = 'Use an item with an equipment slot',
                  ERRCODE = 'check_violation',
                  COLUMN = 'item_id',
                  TABLE = 'CharacterEquipment',
                  CONSTRAINT = 'character_equipment_has_slot';
    END IF;

    FOR row IN
        SELECT Items.equip_slot AS equip FROM Items INNER JOIN CharacterEquipment ON CharacterEquipment.item_id = Items.id
            WHERE CharacterEquipment.char_id = NEW.char_id
    LOOP
        IF row.equip == slot THEN
            RAISE EXCEPTION 'Character % already has an item equipped in %', NEW.char_id, slot
                USING HINT = 'Remove the existing equipment in that slot first',
                    ERRCODE = 'check_violation',
                    COLUMN = 'item_id',
                    TABLE = 'CharacterEquipment',
                    CONSTRAINT = 'character_equipment_unique_slot';
        END IF;
    END LOOP;
    RETURN NEW;
END
$$
LANGUAGE plpgsql;

CREATE CONSTRAINT TRIGGER character_equipment_consistency AFTER INSERT ON CharacterEquipment
    FOR EACH ROW EXECUTE FUNCTION validate_character_equipment();