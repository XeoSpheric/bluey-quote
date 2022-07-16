use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    pub name: String,
    pub breed: String,
    pub gender: String,
    pub eyes: Vec<String>,
    pub fur: Vec<String>,
    pub relatives: Vec<Relationship>,
    pub friends: Vec<i32>,
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Relationship {
    pub person_id: i32,
    pub relationship: String,
}

impl Character {
    pub fn new(name: String, id: i32) -> Character {
        Character {
            name,
            breed: String::from("Blue Heeler"),
            gender: String::from("Male"),
            eyes: vec![],
            fur: vec![],
            relatives: vec![],
            friends: vec![],
            id,
        }
    }
}

pub enum CharacterError {
    InvalidName,
    InvalidBreed,
    InvalidGender,
    InvalidEyes,
    InvalidFur,
    InvalidRelatives,
    InvalidFriends,
}

pub enum CharacterResult {
    Ok(Character),
    Err(CharacterError),
}
