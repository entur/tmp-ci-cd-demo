use std::{convert::Infallible, env::var_os};

use cache::with_cache;
use warp::{Filter, Rejection, Reply};

use crate::cache::InMemoryCache;

mod bucket;
mod cache;
mod error;
mod health;

#[cfg(test)]
mod tests;

type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    let cache = InMemoryCache::new();

    let port = match var_os("PORT") {
        Some(v) => v
            .into_string()
            .ok()
            .unwrap_or("8080".into())
            .parse::<u16>()
            .unwrap_or(8080),
        None => 8080,
    };

    println!("Server started at localhost:{}", port);
    warp::serve(router(cache)).run(([0, 0, 0, 0], port)).await;
}

fn router(cache: InMemoryCache) -> impl Filter<Extract = impl Reply, Error = Infallible> + Clone {
    let bucket = warp::path::end()
        .and(warp::get())
        .and(with_cache(cache))
        .and_then(bucket::root);
    let alive = warp::path!("health" / "alive").and_then(health::alive);
    let ready = warp::path!("health" / "ready").and_then(health::ready);

    let routes = bucket.or(alive).or(ready);
    routes.recover(error::handle_rejection)
}
