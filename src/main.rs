mod common;

use crate::common::test_config::{setup_tests, test_bucket, test_collection, test_scope};
use crate::common::{create_cluster_from_test_config, new_key};

#[tokio::main]
async fn main() {
    test_upsert().await;
}

async fn test_upsert() {
    setup_tests();

    let cluster = create_cluster_from_test_config().await;

    let collection = cluster
        .bucket(test_bucket())
        .await
        .scope(test_scope())
        .collection(test_collection());

    let key = new_key();

    collection.upsert(&key, "test", None).await.unwrap();

    let res = collection.get(key, None).await.unwrap();

    let content: String = res.content_as().unwrap();

    println!("{}", content);
}