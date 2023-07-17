use futures::executor::block_on;
use lazy_static::lazy_static;
use meilisearch_sdk::{client::*, Settings};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::stdin;

// instantiate the client. load it once
lazy_static! {
    static ref CLIENT: Client = Client::new("http://localhost:7700", Some("masterKey"));
}

fn main() {
    block_on(async move {
        // build the index
        build_index().await;

        // enter in search queries or quit
        loop {
            println!("Enter a search query or type \"q\" or \"quit\" to quit:");
            let mut input_string = String::new();
            stdin()
                .read_line(&mut input_string)
                .expect("Failed to read line");
            match input_string.trim() {
                "quit" | "q" | "" => {
                    println!("exiting...");
                    break;
                }
                _ => {
                    search(input_string.trim()).await;
                }
            }
        }
        // get rid of the index at the end, doing this only so users don't have the index without knowing
        let _ = CLIENT.delete_index("clothes").await.unwrap();
    })
}

async fn search(query: &str) {
    // make the search query, which excutes and serializes hits into the
    // ClothesDisplay struct
    let query_results = CLIENT
        .index("clothes")
        .search()
        .with_query(query)
        .execute::<ClothesDisplay>()
        .await
        .unwrap()
        .hits;

    // display the query results
    if query_results.is_empty() {
        println!("no results...");
    } else {
        for clothes in query_results {
            let display = clothes.result;
            println!("{}", format_args!("{}", display));
        }
    }
}

async fn build_index() {
    // reading and parsing the file
    let content = include_str!("../assets/clothes.json");

    // serialize the string to clothes objects
    let clothes: Vec<Clothes> = serde_json::from_str(content).unwrap();

    // create displayed attributes
    let displayed_attributes = ["article", "cost", "size", "pattern"];

    // Create ranking rules
    let ranking_rules = ["words", "typo", "attribute", "exactness", "cost:asc"];

    // create searchable attributes
    let searchable_attributes = ["seaon", "article", "size", "pattern"];

    // create the synonyms hashmap
    let mut synonyms = std::collections::HashMap::new();
    synonyms.insert("sweater", vec!["cardigan", "long-sleeve"]);
    synonyms.insert("sweat pants", vec!["joggers", "gym pants"]);
    synonyms.insert("t-shirt", vec!["tees", "tshirt"]);

    // create the settings struct
    let settings = Settings::new()
        .with_ranking_rules(ranking_rules)
        .with_searchable_attributes(searchable_attributes)
        .with_displayed_attributes(displayed_attributes)
        .with_synonyms(synonyms);

    // add the settings to the index
    let result = CLIENT
        .index("clothes")
        .set_settings(&settings)
        .await
        .unwrap()
        .wait_for_completion(&CLIENT, None, None)
        .await
        .unwrap();

    if result.is_failure() {
        panic!(
            "Encountered an error while setting settings for index: {:?}",
            result.unwrap_failure()
        );
    }

    // add the documents
    let result = CLIENT
        .index("clothes")
        .add_or_update(&clothes, Some("id"))
        .await
        .unwrap()
        .wait_for_completion(&CLIENT, None, None)
        .await
        .unwrap();

    if result.is_failure() {
        panic!(
            "Encountered an error while sending the documents: {:?}",
            result.unwrap_failure()
        );
    }
}

/// Base search object.
#[derive(Serialize, Deserialize, Debug)]
pub struct Clothes {
    id: usize,
    seaon: String,
    article: String,
    cost: f32,
    size: String,
    pattern: String,
}

/// Search results get serialized to this struct
#[derive(Serialize, Deserialize, Debug)]
pub struct ClothesDisplay {
    article: String,
    cost: f32,
    size: String,
    pattern: String,
}

impl fmt::Display for ClothesDisplay {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "result\n article: {},\n price: {},\n size: {},\n pattern: {}\n",
            self.article, self.cost, self.size, self.pattern
        )
    }
}
