use serde::{Serialize,Deserialize};	
use uuid::Uuid;

#[derive(Serialize,Deserialize)]
pub struct DeitySummary {
    id: Uuid,
    name: String,
    domains: Vec<String>,   //name fields from Domains
}

#[derive(Serialize,Deserialize)]
pub struct Deity {
    links: HashMap<&str, Link>,

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
