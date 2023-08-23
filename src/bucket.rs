// use google_cloud_default::WithAuthExt;
use google_cloud_storage::{
    client::{Client, ClientConfig},
    http::objects::{download::Range, get::GetObjectRequest},
};
use std::env::var_os;
use warp::{reject, Reply};

use crate::{cache::InMemoryCache, error::Error, Result};

const BUCKET_NAME: &str = "BUCKET_NAME";
const SA_EMAIL: &str = "application";

pub async fn get_bucket_object(obj: String) -> Result<String> {
    let mut config = ClientConfig::default().with_auth().await.unwrap();
    config.default_google_access_id = google_cloud_metadata::email(SA_EMAIL).await.ok();
    let client = Client::new(config);
    let b = match var_os(BUCKET_NAME) {
        Some(v) => v.into_string().unwrap(),
        None => panic!("env var BUCKET_NAME is not set"),
    };
    let data = client
        .download_object(
            &GetObjectRequest {
                bucket: b,
                object: obj.clone(),
                ..Default::default()
            },
            &Range::default(),
        )
        .await;
    match data {
        Ok(data) => {
            if let Ok(utf) = String::from_utf8(data) {
                return Ok(utf);
            }
        }
        Err(e) => println!("{:?}", e),
    }
    Err(reject::custom(Error::ReadFile(obj)))
}

pub async fn root(cache: InMemoryCache) -> Result<impl Reply> {
    let obj = String::from("rocket.txt");
    if cache.values.read().contains_key(&obj) {
        match cache.values.read().get(&obj) {
            Some(value) => return Ok(warp::reply::json(value)),
            None => {
                return Err(warp::reject::custom(Error::Cache(format!(
                    "Failed to allocate space for {}",
                    obj
                ))));
            }
        }
    }
    match get_bucket_object(obj.clone()).await {
        Ok(file) => {
            cache.values.write().insert(obj, file.clone());
            Ok(warp::reply::json(&file))
        }
        Err(err) => Err(err),
    }
}
