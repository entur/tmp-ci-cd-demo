use parking_lot::RwLock;
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use warp::Filter;

type FileCache = HashMap<String, String>;

#[derive(Clone)]
pub struct InMemoryCache {
    pub values: Arc<RwLock<FileCache>>,
}

impl InMemoryCache {
    pub fn new() -> Self {
        InMemoryCache {
            values: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

pub fn with_cache(
    cache: InMemoryCache,
) -> impl Filter<Extract = (InMemoryCache,), Error = Infallible> + Clone {
    warp::any().map(move || cache.clone())
}
