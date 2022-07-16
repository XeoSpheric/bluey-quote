mod api;

use api::{
    api::{add_charecter, add_quote},
    characters::Character,
    quotes::Quote,
    store::Store,
};
use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let add_quotes = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("quote"))
        .and(warp::path::end())
        .and(json_body_quote())
        .and(store_filter.clone())
        .and_then(add_quote);

    let add_chars = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("charecter"))
        .and(warp::path::end())
        .and(json_body_char())
        .and(store_filter.clone())
        .and_then(add_charecter);

    // let get_quotes = warp::get()
    //     .and(warp::path("v1"))
    //     .and(warp::path("groceries"))
    //     .and(warp::path::end())
    //     .and(store_filter.clone())
    //     .and_then(get_grocery_list);

    let routes = add_quotes.or(add_chars);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn json_body_quote() -> impl Filter<Extract = (Quote,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn json_body_char() -> impl Filter<Extract = (Character,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
