use crate::{errors::Error, indexes::Index};
use serde::{de::DeserializeOwned, Deserialize, Serialize, Serializer};
use std::collections::HashMap;

#[derive(Deserialize, Debug, PartialEq)]
pub struct MatchRange {
    start: usize,
    length: usize,
}

/// A single result.
/// Contains the complete object, optionally the formatted object, and optionally an object that contains information about the matches.
#[derive(Deserialize, Debug)]
pub struct SearchResult<T> {
    /// The full result.
    #[serde(flatten)]
    pub result: T,
    /// The formatted result.
    #[serde(rename = "_formatted")]
    pub formatted_result: Option<T>,
    /// The object that contains information about the matches.
    #[serde(rename = "_matchesInfo")]
    pub matches_info: Option<HashMap<String, Vec<MatchRange>>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A struct containing search results and other information about the search.
pub struct SearchResults<T> {
    /// Results of the query
    pub hits: Vec<SearchResult<T>>,
    /// Number of documents skipped
    pub offset: usize,
    /// Number of results returned
    pub limit: usize,
    /// Total number of matches
    pub nb_hits: usize,
    /// Whether nb_hits is exhaustive
    pub exhaustive_nb_hits: bool,
    /// Distribution of the given facets
    pub facets_distribution: Option<HashMap<String, HashMap<String, usize>>>,
    /// Whether facet_distribution is exhaustive
    pub exhaustive_facets_count: Option<bool>,
    /// Processing time of the query
    pub processing_time_ms: usize,
    /// Query originating the response
    pub query: String,
}

fn serialize_with_wildcard<S: Serializer, T: Serialize>(
    data: &Option<Selectors<T>>,
    s: S,
) -> Result<S::Ok, S::Error> {
    match data {
        Some(Selectors::All) => ["*"].serialize(s),
        Some(Selectors::Some(data)) => data.serialize(s),
        None => s.serialize_none(),
    }
}

fn serialize_attributes_to_crop_with_wildcard<S: Serializer>(
    data: &Option<Selectors<Vec<AttributeToCrop>>>,
    s: S,
) -> Result<S::Ok, S::Error> {
    match data {
        Some(Selectors::All) => ["*"].serialize(s),
        Some(Selectors::Some(data)) => {
            let mut results = Vec::new();
            for attr in data.iter() {
                let mut result = String::new();
                result.push_str(&attr.key);
                if let Some(value) = attr.value {
                    result.push(':');
                    result.push_str(value.to_string().as_str());
                }
                results.push(result)
            }
            results.serialize(s)
        }
        None => s.serialize_none(),
    }
}

/// Some list fields in a `Query` can be set to a wildcard value.
/// This structure allows you to choose between the wildcard value and an exhaustive list of selectors.
#[derive(Debug, Clone)]
pub enum Selectors<T> {
    /// A list of selectors
    Some(T),
    /// The wildcard
    All,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AttributeToCrop {
    pub key: String,
    pub value: Option<usize>,
}

impl AttributeToCrop {

    pub fn new<S: AsRef<str>>(key: S, value: Option<usize>) -> Self {
        Self { key: key.as_ref().into(), value: value }
    }

}

/// A struct representing a query.
/// You can add search parameters using the builder syntax.
/// See [this page](https://docs.meilisearch.com/reference/features/search_parameters.html#query-q) for the official list and description of all parameters.
///
/// # Examples
///
/// ```
/// # use meilisearch_sdk::{client::Client, search::Query, indexes::Index};
/// # let client = Client::new("http://localhost:7700", "masterKey");
/// # let index = client.assume_index("does not matter");
/// let query = Query::new(&index)
///     .with_query("space")
///     .with_offset(42)
///     .with_limit(21)
///     .build(); // you can also execute() instead of build()
/// ```
///
/// ```
/// # use meilisearch_sdk::{client::Client, search::Query, indexes::Index};
/// # let client = Client::new("http://localhost:7700", "masterKey");
/// # let index = client.assume_index("does not matter");
/// let query = index.search()
///     .with_query("space")
///     .with_offset(42)
///     .with_limit(21)
///     .build(); // you can also execute() instead of build()
/// ```
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    #[serde(skip_serializing)]
    index: Index,
    /// The text that will be searched for among the documents.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "q")]
    pub query: Option<String>,
    /// The number of documents to skip.
    /// If the value of the parameter `offset` is `n`, the `n` first documents (ordered by relevance) will not be returned.
    /// This is helpful for pagination.
    ///
    /// Example: If you want to skip the first document, set offset to `1`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    /// The maximum number of documents returned.
    /// If the value of the parameter `limit` is `n`, there will never be more than `n` documents in the response.
    /// This is helpful for pagination.
    ///
    /// Example: If you don't want to get more than two documents, set limit to `2`.
    /// Default: `20`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    /// Filters applied to documents.
    /// Read the [dedicated guide](https://docs.meilisearch.com/reference/features/filtering.html) to learn the syntax.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<String>,
    /// Facet names and values to filter on.
    /// Read [this page](https://docs.meilisearch.com/reference/features/search_parameters.html#facet-filters) for a complete explanation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facet_filters: Option<Vec<Vec<String>>>,
    /// Facets for which to retrieve the matching count.
    ///
    /// Can be set to a [wildcard value](enum.Selectors.html#variant.All) that will select all existing attributes.
    /// Default: all attributes found in the documents.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_with_wildcard")]
    pub facets_distribution: Option<Selectors<Vec<String>>>,
    /// Attributes to display in the returned documents.
    ///
    /// Can be set to a [wildcard value](enum.Selectors.html#variant.All) that will select all existing attributes.
    /// Default: all attributes found in the documents.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_with_wildcard")]
    pub attributes_to_retrieve: Option<Selectors<Vec<String>>>,
    /// Attributes whose values have to be cropped.
    /// Attributes are composed by the attribute name and an optional `usize` that overwrites the `crop_length` parameter.
    ///
    /// Can be set to a [wildcard value](enum.Selectors.html#variant.All) that will select all existing attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_attributes_to_crop_with_wildcard")]
    pub attributes_to_crop: Option<Selectors<Vec<AttributeToCrop>>>,
    /// Number of characters to keep on each side of the start of the matching word.
    /// See [attributes_to_crop](#structfield.attributes_to_crop).
    ///
    /// Default: `200`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crop_length: Option<usize>,
    /// Attributes whose values will contain **highlighted matching terms**.
    ///
    /// Can be set to a [wildcard value](enum.Selectors.html#variant.All) that will select all existing attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_with_wildcard")]
    pub attributes_to_highlight: Option<Selectors<Vec<String>>>,
    /// Defines whether an object that contains information about the matches should be returned or not.
    ///
    /// Default: `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matches: Option<bool>,
}

#[allow(missing_docs)]
impl Query {

    pub fn new(index: &Index) -> Self {
        Self {
            index: index.clone(),
            query: None,
            offset: None,
            limit: None,
            filters: None,
            facet_filters: None,
            facets_distribution: None,
            attributes_to_retrieve: None,
            attributes_to_crop: None,
            crop_length: None,
            attributes_to_highlight: None,
            matches: None,
        }
    }

    pub fn with_query<S: AsRef<str>>(&mut self, query: S) -> &mut Query {
        self.query = Some(query.as_ref().to_string());
        self
    }

    pub fn with_offset(&mut self, offset: usize) -> &mut Query {
        self.offset = Some(offset);
        self
    }

    pub fn with_limit(&mut self, limit: usize) -> &mut Query {
        self.limit = Some(limit);
        self
    }

    pub fn with_filters<S: AsRef<str>>(&mut self, filters: S) -> &mut Query {
        self.filters = Some(filters.as_ref().to_string());
        self
    }

    pub fn with_facet_filters<S: AsRef<str>>(
        &mut self,
        facet_filters: Vec<Vec<S>>,
    ) -> & mut Query {
        self.facet_filters = Some(
            facet_filters
                .iter()
                .map(|v| v.iter().map(|v| v.as_ref().into()).collect())
                .collect()
            );
        self
    }

    pub fn with_facets_distribution<S: AsRef<str>>(
        &mut self,
        facets_distribution: Selectors<Vec<S>>,
    ) -> & mut Query {
        let facets_distribution = match facets_distribution {
            Selectors::All => Selectors::All,
            Selectors::Some(data) => 
                Selectors::Some(
                    data
                        .iter()
                        .map(|data| data.as_ref().into())
                        .collect()
                    ),
        };
        self.facets_distribution = Some(facets_distribution);
        self
    }

    pub fn with_attributes_to_retrieve<S: AsRef<str>>(
        &mut self,
        attributes_to_retrieve: Selectors<Vec<S>>,
    ) -> &mut Query {
        let attributes_to_retrieve = match attributes_to_retrieve {
            Selectors::All => Selectors::All,
            Selectors::Some(data) => 
                Selectors::Some(
                    data
                        .iter()
                        .map(|data| data.as_ref().into())
                        .collect()
                    ),
        };
        self.attributes_to_retrieve = Some(attributes_to_retrieve);
        self
    }

    pub fn with_attributes_to_crop(
        &mut self,
        attributes_to_crop: Selectors<Vec<AttributeToCrop>>,
    ) -> &mut Query {
        self.attributes_to_crop = Some(attributes_to_crop);
        self
    }

    pub fn with_attributes_to_highlight<S: AsRef<str>>(
        &mut self,
        attributes_to_highlight: Selectors<Vec<S>>,
    ) -> &mut Query {
        let attributes_to_highlight = match attributes_to_highlight {
            Selectors::All => Selectors::All,
            Selectors::Some(data) => 
                Selectors::Some(
                    data
                        .iter()
                        .map(|data| data.as_ref().into())
                        .collect()
                    ),
        };
        self.attributes_to_highlight = Some(attributes_to_highlight);
        self
    }

    pub fn with_crop_length(&mut self, crop_length: usize) -> &mut Query {
        self.crop_length = Some(crop_length);
        self
    }

    pub fn with_matches(&mut self, matches: bool) -> &mut Query {
        self.matches = Some(matches);
        self
    }

    pub fn build(&mut self) -> Query {
        self.clone()
    }

    /// Execute the query and fetch the results.
    pub async fn execute<T: 'static + DeserializeOwned>(
        &self
    ) -> Result<SearchResults<T>, Error> {
        self.index.execute_query::<T>(&self).await
    }
}

#[cfg(test)]
mod tests {

    use crate::{client::*, document, search::*};
    use serde::{Deserialize, Serialize};
    use std::thread::sleep;
    use std::time::Duration;
    use futures_await_test::async_test;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Document {
        id: usize,
        value: String,
        kind: String,
    }

    impl document::Document for Document {
        type UIDType = usize;

        fn get_uid(&self) -> &Self::UIDType {
            &self.id
        }
    }

    #[allow(unused_must_use)]
    async fn setup_test_index<S: AsRef<str>>(client: &Client, name: S) -> Index {

        let name: &str = name.as_ref();

        // try to delete
        client.delete_index(name).await;

        let index = client.create_index(name, None).await.unwrap();
        index.add_documents(&[
            Document { id: 0, kind: "text".into(), value: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_string() },
            Document { id: 1, kind: "text".into(), value: "dolor sit amet, consectetur adipiscing elit".to_string() },
            Document { id: 2, kind: "title".into(), value: "The Social Network".to_string() },
            Document { id: 3, kind: "title".into(), value: "Harry Potter and the Sorcerer's Stone".to_string() },
            Document { id: 4, kind: "title".into(), value: "Harry Potter and the Chamber of Secrets".to_string() },
            Document { id: 5, kind: "title".into(), value: "Harry Potter and the Prisoner of Azkaban".to_string() },
            Document { id: 6, kind: "title".into(), value: "Harry Potter and the Goblet of Fire".to_string() },
            Document { id: 7, kind: "title".into(), value: "Harry Potter and the Order of the Phoenix".to_string() },
            Document { id: 8, kind: "title".into(), value: "Harry Potter and the Half-Blood Prince".to_string() },
            Document { id: 9, kind: "title".into(), value: "Harry Potter and the Deathly Hallows".to_string() },
        ], None).await.unwrap();
        index.set_attributes_for_faceting(["kind"]).await.unwrap();
        sleep(Duration::from_secs(1));
        index
    }

    #[async_test]
    async fn test_query_string() {
        let client = Client::new("http://localhost:7700", "masterKey");
        let index = setup_test_index(&client, "test_query_string").await;

        let results: SearchResults<Document> =
            index.search().with_query("dolor").execute().await.unwrap();
        assert_eq!(results.hits.len(), 2);

        client.delete_index("test_query_string").await.unwrap();
    }

    #[async_test]
    async fn test_query_limit() {
        let client = Client::new("http://localhost:7700", "masterKey");
        let index = setup_test_index(&client, "test_query_limit").await;

        let results: SearchResults<Document> =
            index.search().with_limit(5).execute().await.unwrap();
        assert_eq!(results.hits.len(), 5);

        client.delete_index("test_query_limit").await.unwrap();
    }

    #[async_test]
    async fn test_query_offset() {
        let client = Client::new("http://localhost:7700", "masterKey");
        let index = setup_test_index(&client, "test_query_offset").await;

        let results: SearchResults<Document> =
            index.search().with_offset(6).execute().await.unwrap();
        assert_eq!(results.hits.len(), 4);

        client.delete_index("test_query_offset").await.unwrap();
    }

    #[async_test]
    async fn test_query_filters() {
        let client = Client::new("http://localhost:7700", "masterKey");
        let index = setup_test_index(&client, "test_query_filters").await;

        let results: SearchResults<Document> = index
            .search()
            .with_filters("value = \"The Social Network\"")
            .execute()
            .await
            .unwrap();
        assert_eq!(results.hits.len(), 1);

        let results: SearchResults<Document> = index
            .search()
            .with_filters("NOT value = \"The Social Network\"")
            .execute()
            .await
            .unwrap();
        assert_eq!(results.hits.len(), 9);

        client.delete_index("test_query_filters").await.unwrap();
    }

    #[async_test]
    async fn test_query_facet_filters() {
        let client = Client::new("http://localhost:7700", "masterKey");
        let index = setup_test_index(&client, "test_query_facet_filters").await;

        let mut query = Query::new(&index);
        query.with_facet_filters::<&str>(vec!(vec!("kind:text")));
        let results: SearchResults<Document> = index.execute_query(&query).await.unwrap();
        assert_eq!(results.hits.len(), 8);

        let mut query = Query::new(&index);
        query.with_facet_filters::<&str>(vec!(vec!("kind:text")));
        let results: SearchResults<Document> = index.execute_query(&query).await.unwrap();
        assert_eq!(results.hits.len(), 2);

        let mut query = Query::new(&index);
        query.with_facet_filters::<&str>(vec!(vec!("kind:text", "kind:title")));
        let results: SearchResults<Document> = index.execute_query(&query).await.unwrap();
        assert_eq!(results.hits.len(), 10);

        client
            .delete_index("test_query_facet_filters")
            .await
            .unwrap();
    }

    #[async_test]
    async fn test_query_facet_distribution() {
        let client = Client::new("http://localhost:7700", "masterKey");
        let index = setup_test_index(&client, "test_query_facet_distribution").await;

        let mut query = Query::new(&index);
        query.with_facets_distribution::<&str>(Selectors::All);
        let results: SearchResults<Document> = index.execute_query(&query).await.unwrap();
        assert_eq!(
            results
                .facets_distribution
                .unwrap()
                .get("kind")
                .unwrap()
                .get("title")
                .unwrap(),
            &8
        );

        let mut query = Query::new(&index);
        query.with_facets_distribution::<&str>(Selectors::Some(vec!("kind")));
        query.with_facet_filters(vec!(vec!("kind:text")));
        let results: SearchResults<Document> = index.execute_query(&query).await.unwrap();
        assert_eq!(
            results
                .facets_distribution
                .clone()
                .unwrap()
                .get("kind")
                .unwrap()
                .get("title")
                .unwrap(),
            &0
        );
        assert_eq!(
            results
                .facets_distribution
                .unwrap()
                .get("kind")
                .unwrap()
                .get("text")
                .unwrap(),
            &2
        );

        client
            .delete_index("test_query_facet_distribution")
            .await
            .unwrap();
    }

    #[async_test]
    async fn test_query_attributes_to_retrieve() {
        let client = Client::new("http://localhost:7700", "masterKey");
        let index = setup_test_index(&client, "test_query_attributes_to_retrieve").await;

        let results: SearchResults<Document> = index
            .search()
            .with_attributes_to_retrieve::<&str>(Selectors::All)
            .execute()
            .await
            .unwrap();
        assert_eq!(results.hits.len(), 10);

        let mut query = Query::new(&index);
        query.with_attributes_to_retrieve(Selectors::Some(vec!("kind", "id"))); // omit the "value" field
        assert!(index.execute_query::<Document>(&query).await.is_err()); // error: missing "value" field

        client
            .delete_index("test_query_attributes_to_retrieve")
            .await
            .unwrap();
    }

    #[async_test]
    async fn test_query_attributes_to_crop() {
        let client = Client::new("http://localhost:7700", "masterKey");
        let index = setup_test_index(&client, "test_query_attributes_to_crop").await;

        let mut query = Query::new(&index);
        query.with_query("lorem ipsum");
        query.with_attributes_to_crop(Selectors::All);
        let results: SearchResults<Document> = index.execute_query(&query).await.unwrap();
        assert_eq!(results.hits[0].formatted_result.as_ref().unwrap(), &Document {
            id: 0,
            value: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip".to_string(),
            kind: "text".to_string()
        });

        let mut query = Query::new(&index);
        query.with_query("lorem ipsum");
        query.with_attributes_to_crop(Selectors::Some(vec!(AttributeToCrop::new("value", Some(50)), AttributeToCrop::new("kind", None))));
        let results: SearchResults<Document> = index.execute_query(&query).await.unwrap();
        assert_eq!(
            results.hits[0].formatted_result.as_ref().unwrap(),
            &Document {
                id: 0,
                value: "Lorem ipsum dolor sit amet, consectetur adipiscing".to_string(),
                kind: "text".to_string()
            }
        );

        client
            .delete_index("test_query_attributes_to_crop")
            .await
            .unwrap();
    }

    #[async_test]
    async fn test_query_crop_lenght() {
        let client = Client::new("http://localhost:7700", "masterKey");
        let index = setup_test_index(&client, "test_query_crop_lenght").await;

        let mut query = Query::new(&index);
        query.with_query("lorem ipsum");
        query.with_attributes_to_crop(Selectors::All);
        query.with_crop_length(200);
        let results: SearchResults<Document> = index.execute_query(&query).await.unwrap();
        assert_eq!(results.hits[0].formatted_result.as_ref().unwrap(), &Document {
            id: 0,
            value: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip".to_string(),
            kind: "text".to_string()
        });

        let mut query = Query::new(&index);
        query.with_query("lorem ipsum");
        query.with_attributes_to_crop(Selectors::All);
        query.with_crop_length(50);
        let results: SearchResults<Document> = index.execute_query(&query).await.unwrap();
        assert_eq!(
            results.hits[0].formatted_result.as_ref().unwrap(),
            &Document {
                id: 0,
                value: "Lorem ipsum dolor sit amet, consectetur adipiscing".to_string(),
                kind: "text".to_string()
            }
        );

        client.delete_index("test_query_crop_lenght").await.unwrap();
    }

    #[async_test]
    async fn test_query_attributes_to_highlight() {
        let client = Client::new("http://localhost:7700", "masterKey");
        let index = setup_test_index(&client, "test_query_attributes_to_highlight").await;

        let mut query = Query::new(&index);
        query.with_query("dolor text");
        query.with_attributes_to_highlight::<&str>(Selectors::All);
        let results: SearchResults<Document> = index.execute_query(&query).await.unwrap();
        assert_eq!(
            results.hits[0].formatted_result.as_ref().unwrap(),
            &Document {
                id: 1,
                value: "<em>dolor</em> sit amet, consectetur adipiscing elit".to_string(),
                kind: "<em>text</em>".to_string()
            }
        );

        let mut query = Query::new(&index);
        query.with_query("dolor text");
        query.with_attributes_to_highlight(Selectors::Some(vec!("value")));
        let results: SearchResults<Document> = index.execute_query(&query).await.unwrap();
        assert_eq!(
            results.hits[0].formatted_result.as_ref().unwrap(),
            &Document {
                id: 1,
                value: "<em>dolor</em> sit amet, consectetur adipiscing elit".to_string(),
                kind: "text".to_string()
            }
        );

        client
            .delete_index("test_query_attributes_to_highlight")
            .await
            .unwrap();
    }

    #[async_test]
    async fn test_query_matches() {
        let client = Client::new("http://localhost:7700", "masterKey");
        let index = setup_test_index(&client, "test_query_matches").await;

        let mut query = Query::new(&index);
        query.with_query("dolor text");
        query.with_matches(true);
        let results: SearchResults<Document> = index.execute_query(&query).await.unwrap();
        assert_eq!(results.hits[0].matches_info.as_ref().unwrap().len(), 2);
        assert_eq!(
            results.hits[0]
                .matches_info
                .as_ref()
                .unwrap()
                .get("value")
                .unwrap(),
            &vec![MatchRange {
                start: 0,
                length: 5
            }]
        );

        client.delete_index("test_query_matches").await.unwrap();
    }
}
