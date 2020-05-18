use serde::{Serialize,Deserialize};	
use uuid::Uuid;

#[derive(Serialize,Deserialize)]
pub struct Deity {
    id: Uuid,

    name: String,
    description: String,
    favored_animals: Vec<String>,

    domains: Vec<Domain>,
}

#[derive(Serialize,Deserialize)]
pub struct Domain {
    id: Uuid,

    name: String,
    description: String,
    power_description: String,

    subdomains: Vec<Subdomain>,
}

#[derive(Serialize,Deserialize)]
pub struct Subdomain {
    name: String,
    description: String,
}
