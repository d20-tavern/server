use serde::{Serialize,Deserialize};	
use uuid::Uuid;

use crate::WeaponClass;
use crate::ArmorClass;

#[derive(Serialize,Deserialize)]
pub struct WeaponsNotProficient {
    class_id: Uuid,
    weapon_classes: Vec<WeaponClass>
}

#[derive(Serialize,Deserialize)]
pub struct ArmorNotProficient {
    class_id: Uuid,
    armor_classes: Vec<ArmorClass>
}
