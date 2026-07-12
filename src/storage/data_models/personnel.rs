use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Person
{
    pub id: PersonId,
    pub name: String,
}

pub type PersonId = u64;

