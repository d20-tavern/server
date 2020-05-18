use serde::{Serialize,Deserialize};	
use uuid::Uuid;

#[derive(Serialize,Deserialize)]
pub struct Deity {
    deity_id: Uuid,

    name: String,
    description: String,
    favored_animals: Vec<String>,
}

#[derive(Serialize,Deserialize)]
pub struct Domain {
    domain_id: Uuid,

    name: String,
    description: String,
    power_description: String,
}

#[derive(Serialize,Deserialize)]
pub struct Subdomain {
    domain_id: Uuid,

    name: String,
    description: String,
}
