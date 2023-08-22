use crate::{cache::InMemoryCache, router};
use serial_test::serial;
use warp::test::request;

#[tokio::test]
#[should_panic]
#[serial]
async fn test_panic_if_env_not_set() {
    let cache = InMemoryCache::new();
    let r = router(cache);
    let resp = request().path("/").reply(&r).await;
    assert_eq!(resp.status(), 500);
}

#[tokio::test()]
#[serial]
async fn test_use_cache_if_exist() {
    let cache = InMemoryCache::new();
    cache
        .values
        .write()
        .insert("rocket.txt".into(), "From text".to_string());
    let r = router(cache.clone());
    std::env::set_var("BUCKET_NAME", "ent-gcs-rocketlaunch-sbx-001");
    let resp = request().path("/").reply(&r).await;
    std::env::remove_var("BUCKET_NAME");
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.body(), "\"From text\"");
}

#[tokio::test()]
#[serial]
#[ignore]
async fn test_integration_test_with_bucket() {
    let cache = InMemoryCache::new();
    let r = router(cache.clone());
    std::env::set_var("BUCKET_NAME", "ent-gcs-rocketlaunch-sbx-001");
    let resp = request().path("/").reply(&r).await;
    std::env::remove_var("BUCKET_NAME");
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.body(), "\"We have liftoff!\"");
    assert!(cache.values.read().contains_key("rocket.txt"));
}
