use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

use super::{characters::Character, quotes::Quote};

pub type Characters = HashMap<i32, Character>;
pub type Quotes = HashMap<i32, Quote>;

#[derive(Clone)]
pub struct Store {
    pub characters: Arc<RwLock<Characters>>,
    pub quotes: Arc<RwLock<Quotes>>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            characters: Arc::new(RwLock::new(HashMap::new())),
            quotes: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
