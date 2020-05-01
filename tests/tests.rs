use env_logger::init;
use log::{error, info, warn};
use meilisearch_sdk::{client::*, documents::*, errors::Error, indexes::*};

#[test]
fn test() {
    std::panic::catch_unwind(|| init());
    let client = Client::new("http://localhost:7700", "fzd");

    for index in client.list_all_indexes().unwrap() {
        index.delete().unwrap();
    }
    assert!(client.list_all_indexes().unwrap().is_empty());

    client.create_index("movies", None).unwrap();
    assert_eq!(client.list_all_indexes().unwrap().len(), 1);

    println!("{:?}", client.list_all_indexes())
}