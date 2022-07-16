use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Quote {
    pub quote: String,
    pub owner_id: i32,
    pub id: i32,
}

impl Quote {
    pub fn new(quote: String, owner_id: i32, quote_id: i32) -> Quote {
        Quote {
            quote,
            owner_id,
            id: quote_id,
        }
    }
}

pub enum QuoteError {
    InvalidQuote,
    InvalidOwnerId,
    InvalidQuoteId,
}

pub enum QuoteResult {
    Ok(Quote),
    Err(QuoteError),
}
