use crate::Result;
use std::collections::HashMap;

pub async fn alive() -> Result<impl warp::Reply> {
    Ok(warp::reply::json(
        &[("status", "I'm not dead")]
            .iter()
            .cloned()
            .collect::<HashMap<&str, &str>>(),
    ))
}

pub async fn ready() -> Result<impl warp::Reply> {
    Ok(warp::reply::json(
        &[("status", "Ready")]
            .iter()
            .cloned()
            .collect::<HashMap<&str, &str>>(),
    ))
}
