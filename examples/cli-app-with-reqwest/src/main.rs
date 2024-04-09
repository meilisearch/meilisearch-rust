use async_trait::async_trait;
use lazy_static::lazy_static;
use meilisearch_sdk::errors::Error;
use meilisearch_sdk::request::{
    add_query_parameters, parse_response, qualified_version, HttpClient, Method,
};
use meilisearch_sdk::{client::*, settings::Settings};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::fmt;
use std::io::stdin;

lazy_static! {
    static ref CLIENT: Client<ReqwestClient> =
        Client::new_with_client("http://localhost:7700", Some("masterKey"), ReqwestClient);
}

#[derive(Debug, Clone, Serialize)]
pub struct ReqwestClient;

#[async_trait(?Send)]
impl HttpClient for ReqwestClient {
    async fn request<Query, Body, Output>(
        self,
        url: &str,
        apikey: Option<&str>,
        method: Method<Query, Body>,
        expected_status_code: u16,
    ) -> Result<Output, Error>
    where
        Query: Serialize + Send + Sync,
        Body: Serialize + Send + Sync,
        Output: DeserializeOwned + 'static + Send,
    {
        let response = match &method {
            Method::Get { query } => {
                let url = add_query_parameters(url, query)?;
                let client = reqwest::Client::new();
                let mut builder = client.request(reqwest::Method::GET, url.as_str());
                builder = builder.header(reqwest::header::USER_AGENT, qualified_version());
                if let Some(apikey) = apikey {
                    builder =
                        builder.header(reqwest::header::AUTHORIZATION, format!("Bearer {apikey}"));
                }
                let req = builder.build().unwrap();
                client.execute(req).await.unwrap()
            }
            Method::Post { query, body } => {
                let url = add_query_parameters(url, query)?;
                let client = reqwest::Client::new();
                let mut builder = client.request(reqwest::Method::POST, url.as_str());
                if let Some(apikey) = apikey {
                    builder =
                        builder.header(reqwest::header::AUTHORIZATION, format!("Bearer {apikey}"));
                }
                builder = builder.header(reqwest::header::CONTENT_TYPE, "application/json");
                let req = builder.body(to_string(body).unwrap()).build().unwrap();
                client.execute(req).await.unwrap()
            }
            Method::Patch { query, body } => {
                let url = add_query_parameters(url, query)?;
                let client = reqwest::Client::new();
                let mut builder = client.request(reqwest::Method::PATCH, url.as_str());
                if let Some(apikey) = apikey {
                    builder =
                        builder.header(reqwest::header::AUTHORIZATION, format!("Bearer {apikey}"));
                }
                builder = builder.header(reqwest::header::CONTENT_TYPE, "application/json");
                let req = builder.body(to_string(body).unwrap()).build().unwrap();
                client.execute(req).await.unwrap()
            }
            Method::Put { query, body } => {
                let url = add_query_parameters(url, query)?;
                let client = reqwest::Client::new();
                let mut builder = client.request(reqwest::Method::PUT, url.as_str());
                if let Some(apikey) = apikey {
                    builder =
                        builder.header(reqwest::header::AUTHORIZATION, format!("Bearer {apikey}"));
                }
                builder = builder.header(reqwest::header::CONTENT_TYPE, "application/json");
                let req = builder.body(to_string(body).unwrap()).build().unwrap();
                client.execute(req).await.unwrap()
            }
            Method::Delete { query } => {
                let url = add_query_parameters(url, query)?;
                let client = reqwest::Client::new();
                let mut builder = client.request(reqwest::Method::DELETE, url.as_str());
                if let Some(apikey) = apikey {
                    builder =
                        builder.header(reqwest::header::AUTHORIZATION, format!("Bearer {apikey}"));
                }
                builder = builder.header(reqwest::header::CONTENT_TYPE, "application/json");
                let req = builder.build().unwrap();
                client.execute(req).await.unwrap()
            }
        };

        let status = response.status().as_u16();

        let mut body = response.text().await.unwrap();

        if body.is_empty() {
            body = "null".to_string();
        }

        parse_response(status, expected_status_code, &body, url.to_string())
        // parse_response(status, expected_status_code, body)
    }

    async fn stream_request<
        'a,
        Query: Serialize + Send + Sync,
        Body: futures::AsyncRead + Send + Sync + 'static,
        Output: DeserializeOwned + 'static,
    >(
        self,
        _url: &str,
        _apikey: Option<&str>,
        _method: Method<Query, Body>,
        _content_type: &str,
        _expected_status_code: u16,
    ) -> Result<Output, Error> {
        unimplemented!("stream_request is not implemented for ReqwestClient")
    }
}

#[tokio::main]
async fn main() {
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

    //create displayed attributes
    let displayed_attributes = ["article", "cost", "size", "pattern"];

    // Create ranking rules
    let ranking_rules = ["words", "typo", "attribute", "exactness", "cost:asc"];

    //create searchable attributes
    let searchable_attributes = ["seaon", "article", "size", "pattern"];

    // create the synonyms hashmap
    let mut synonyms = std::collections::HashMap::new();
    synonyms.insert("sweater", vec!["cardigan", "long-sleeve"]);
    synonyms.insert("sweat pants", vec!["joggers", "gym pants"]);
    synonyms.insert("t-shirt", vec!["tees", "tshirt"]);

    //create the settings struct
    let settings = Settings::new()
        .with_ranking_rules(ranking_rules)
        .with_searchable_attributes(searchable_attributes)
        .with_displayed_attributes(displayed_attributes)
        .with_synonyms(synonyms);

    //add the settings to the index
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
