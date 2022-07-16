use super::characters::Character;
use super::quotes::{Quote, QuoteError, QuoteResult};
use super::store::Store;
use rand::{self, prelude::IteratorRandom};
use warp::{http, Filter};

pub fn get_random_quote(quotes: &Vec<Quote>, character_id: Option<&i32>) -> Quote {
    let mut rng = rand::thread_rng();
    match character_id {
        Some(id) => quotes
            .into_iter()
            .cloned()
            .filter(|q| q.owner_id == *id)
            .choose(&mut rng)
            .expect("No quotes found for character"),
        None => quotes
            .into_iter()
            .cloned()
            .choose(&mut rng)
            .expect("No quotes found"),
    }
}

pub async fn add_quote(quote: Quote, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    store.quotes.write().insert(quote.id, quote.clone());

    Ok(warp::reply::with_status(
        "Added quote to the store",
        http::StatusCode::CREATED,
    ))
}

pub async fn add_charecter(
    character: Character,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    store
        .characters
        .write()
        .insert(character.id, character.clone());

    Ok(warp::reply::with_status(
        "Added character to the store",
        http::StatusCode::CREATED,
    ))
}
