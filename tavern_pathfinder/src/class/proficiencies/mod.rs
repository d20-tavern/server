use serde::{Serialize,Deserialize};	
use uuid::Uuid;
#[derive(Serialize,Deserialize)]
pub struct ClassWeaponsNotProficient {
    class_id: Uuid,
    weapon_classes: Vec<crate::WeaponClass>
}

#[derive(Serialize,Deserialize)]
pub struct ClassArmorNotProficient {
    class_id: Uuid,
    armor_classes: Vec<crate::ArmorClass>
}
