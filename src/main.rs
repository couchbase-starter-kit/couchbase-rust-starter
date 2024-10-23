use crate::common::test_config::{setup_tests, test_bucket, test_collection, test_scope};
use crate::common::{create_cluster_from_test_config, new_key};
use bytes::Bytes;
use couchbase::transcoder::{DefaultTranscoder, Transcoder};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use std::collections::BTreeMap;

mod common;


fn main() {
    
    let cluster = create_cluster_from_test_config().await;

    let collection = cluster
        .bucket(test_bucket())
        .await
        .scope(test_scope())
        .collection(test_collection());

    let key = new_key();

    collection.upsert(&key, "test").await.unwrap();

    let res = collection.get(key).await.unwrap();

    let content: String = res.content_as().unwrap();

    assert_eq!("test", content);
    
    println!("Hello, world!");
}
