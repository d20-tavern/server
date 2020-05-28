use async_trait::async_trait;
use crate::db::{self, TryFromUuid};
use crate::status;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
	}
}

impl<'c> FromRow<'c, PgRow<'c>> for Character {
    fn from_row(row: &PgRow<'c>) -> Result<Self, sqlx::Error> {
        let id: Uuid = row.try_get("id")?;
        let race: Uuid = row.try_get("race_id")?;
        let race = Race::try_from_uuid(race, None)?;
        let deity: Uuid = row.try_get("deity_id")?;
        let deity = Summary<Deity>::try_from_uuid(deity, None)?;
        
        let name: String = row.try_get("name")?;
        let age: i16 = row.try_get("age")?;
        let gender: Gender = row.try_get("gender")?;
        let alignment: Alignment = row.try_get("alignment")?;
        let backstory: String = row.try_get("backstory")?;
        let height: i16 = row.try_get("height")?;
        let weight: i16 = row.try_get("weight")?;
        let size: Size = row.try_get("size")?;

        let strength: i16 = row.try_get("strength")?;
        let dexterity: i16 = row.try_get("dexterity")?;
        let constitution: i16 = row.try_get("constitution")?;
        let intelligence: i16 = row.try_get("intelligence")?;
        let wisdom: i16 = row.try_get("wisdom")?;
        let charisma: i16 = row.try_get("charisma")?;
        
        let max_hp: i16 = row.try_get("max_hp")?;
        let damage: i16 = row.try_get("damage")?;
        let nonlethal: i16 = row.try_get("nonlethal")?;

        let copper: i16 = row.try_get("copper")?;
        let silver: i16 = row.try_get("silver")?;
        let gold: i16 = row.try_get("gold")?;
        let platinum: i16 = row.try_get("platinum")?;

        let subclasses = row.try_get("subclasses")?
            .map(|id| Summary<Subclass>::try_from_uuid(id, None));
        let feats = row.try_get("feats")?
            .map(|id| Summary<Feat>::try_from_uuid(id, None)?)
            .collect();
        let spells = row.try_get("spells")?
            .map(|id| Summary<Spell>::try_from_uuid(id, None)?)
            .collect();
        let bags = row.try_get("bags")?
            .map(|id| Bag::try_from_uuid(id, None)?)
            .collect();

        let mut character = Character {
            links: Links{},
            id, race, deity,
            subclasses, feats, spells, bags,
            name, age, gender, alignment, backstory, height, weight, size,
            strength, dexterity, constitution, intelligence, wisdom, charisma,
            max_hp, damage, nonlethal,
            copper, silver, gold, platinum,
            description: String::new(),
        };

        character.links.insert("self".to_string(), format!("/characters/{}", id));
        character.update_desc();

        Ok(character)
    }
}

#[async_trait]
impl TryFromUuid for Character {
    type Error = Rejection;

    async fn try_from_uuid(id: Uuid, user: Option<&Uuid>) -> Result<Self, Self::Error> {
        let conn = db::get_connection()?;
        
        let mut tx = conn.begin()
            .await
            .map_err(|err| status::server_error_into_rejection(err.to_string()))?;

        // The ARRAY(...) AS column_name syntax allows getting multiple rows in another
        // table as a single array value that can be accessed as a Vec<>.
        let query = sqlx::query(
            r"SELECT
                id, user_id, race_id, deity_id,
                name, age, gender, alignment, backstory, height, weight, size,
                strength, dexterity, constitution, intelligence, wisdom, charisma,
                max_hp, damage, nonlethal,
                copper, silver, gold, platinum,
                ARRAY(SELECT subclass_id FROM CharacterSubclasses WHERE characterSubclasses.char_id = $1) AS subclasses,
                ARRAY(SELECT feat_id FROM CharacterFeats WHERE CharacterFeats.char_id = $1) AS feats,
                ARRAY(SELECT id FROM Bags WHERE Bags.char_id = $1) AS bags,
                ARRAY(SELECT item_id FROM CharacterEquipment WHERE CharacterEquipment.char_id = $1) AS equipment,
                ARRAY(SELECT spell_id FROM CharacterSpells WHERE CharacterSpells.char_id = $1) AS spells
                FROM Characters
                WHERE Characters.id = $1
                LIMIT 1
            ")
            .bind(&id)
            .bind(&user);

        let rows = query.fetch(&mut tx).await.map_err(|err| {
            status::server_error_into_rejection(err.to_string())
        })?;

        // We only care about a single row
        let row = rows.next()
            .await
            .map_err(|err| status::server_error_into_rejection(err.to_string()))?
            .ok_or_else()?;

        // Get the character's user id to make sure the given user has access
        let char_user: Uuid = row.try_get("user_id")
            .map_err(status::server_error_into_rejection)?;
        
        if user != char_user {
            return Err(status::not_authorized());
        }

        // The user has access; generate the character
        let character = Character::from_row(&row)
            .map_err(status::server_error_into_rejection)?;

        tx.commit()
            .await
            .map_err(|err| status::server_error_into_rejection(err.to_string()))?;

        Ok(character)
    }
}
