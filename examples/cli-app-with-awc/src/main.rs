use async_trait::async_trait;
use meilisearch_sdk::errors::Error;
use meilisearch_sdk::request::{parse_response, HttpClient, Method};
use meilisearch_sdk::{client::*, settings::Settings};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::stdin;

#[derive(Debug, Clone)]
pub struct AwcClient {
    api_key: Option<String>,
}

impl AwcClient {
    pub fn new(api_key: Option<&str>) -> Result<Self, Error> {
        Ok(AwcClient {
            api_key: api_key.map(|key| key.to_string()),
        })
    }
}

#[async_trait(?Send)]
impl HttpClient for AwcClient {
    async fn stream_request<
        Query: Serialize + Send + Sync,
        Body: futures::AsyncRead + Send + Sync + 'static,
        Output: DeserializeOwned + 'static,
    >(
        &self,
        url: &str,
        method: Method<Query, Body>,
        content_type: &str,
        expected_status_code: u16,
    ) -> Result<Output, Error> {
        let mut builder = awc::ClientBuilder::new();
        if let Some(ref api_key) = self.api_key {
            builder = builder.bearer_auth(api_key);
        }
        builder = builder.add_default_header(("User-Agent", "Rust client with Awc"));
        let client = builder.finish();

        let query = method.query();
        let query = yaup::to_string(query)?;

        let url = if query.is_empty() {
            url.to_string()
        } else {
            format!("{url}?{query}")
        };

        let url = add_query_parameters(&url, method.query())?;
        let request = client.request(verb(&method), &url);

        let mut response = if let Some(body) = method.into_body() {
            let reader = tokio_util::compat::FuturesAsyncReadCompatExt::compat(body);
            let stream = tokio_util::io::ReaderStream::new(reader);
            request
                .content_type(content_type)
                .send_stream(stream)
                .await
                .map_err(|err| Error::Other(Box::new(err)))?
        } else {
            request
                .send()
                .await
                .map_err(|err| Error::Other(Box::new(err)))?
        };

        let status = response.status().as_u16();
        let mut body = String::from_utf8(
            response
                .body()
                .await
                .map_err(|err| Error::Other(Box::new(err)))?
                .to_vec(),
        )
        .map_err(|err| Error::Other(Box::new(err)))?;

        if body.is_empty() {
            body = "null".to_string();
        }

        parse_response(status, expected_status_code, &body, url.to_string())
    }
}

#[actix_rt::main]
async fn main() {
    let http_client = AwcClient::new(Some("masterKey")).unwrap();
    let client = Client::new_with_client("http://localhost:7700", Some("masterKey"), http_client);

    // build the index
    build_index(&client).await;

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
                search(&client, input_string.trim()).await;
            }
        }
    }
    // get rid of the index at the end, doing this only so users don't have the index without knowing
    let _ = client.delete_index("clothes").await.unwrap();
}

async fn search(client: &Client<AwcClient>, query: &str) {
    // make the search query, which excutes and serializes hits into the
    // ClothesDisplay struct
    let query_results = client
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

async fn build_index(client: &Client<AwcClient>) {
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
    let result = client
        .index("clothes")
        .set_settings(&settings)
        .await
        .unwrap()
        .wait_for_completion(client, None, None)
        .await
        .unwrap();

    if result.is_failure() {
        panic!(
            "Encountered an error while setting settings for index: {:?}",
            result.unwrap_failure()
        );
    }

    // add the documents
    let result = client
        .index("clothes")
        .add_or_update(&clothes, Some("id"))
        .await
        .unwrap()
        .wait_for_completion(client, None, None)
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

fn add_query_parameters<Query: Serialize>(url: &str, query: &Query) -> Result<String, Error> {
    let query = yaup::to_string(query)?;

    if query.is_empty() {
        Ok(url.to_string())
    } else {
        Ok(format!("{url}?{query}"))
    }
}

fn verb<Q, B>(method: &Method<Q, B>) -> awc::http::Method {
    match method {
        Method::Get { .. } => awc::http::Method::GET,
        Method::Delete { .. } => awc::http::Method::DELETE,
        Method::Post { .. } => awc::http::Method::POST,
        Method::Put { .. } => awc::http::Method::PUT,
        Method::Patch { .. } => awc::http::Method::PATCH,
    }
}
