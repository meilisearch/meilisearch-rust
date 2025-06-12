use crate::{
    errors::Error,
    indexes::Index,
    request::HttpClient,
    search::{serialize_with_wildcard, Filter, Selectors},
};
use either::Either;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{Map, Value};

/// A single result.
#[derive(Deserialize, Debug, Clone)]
pub struct SimilarResult<T> {
    /// The full result.
    #[serde(flatten)]
    pub result: T,
    /// The relevancy score of the match.
    #[serde(rename = "_rankingScore")]
    pub ranking_score: Option<f64>,
    #[serde(rename = "_rankingScoreDetails")]
    pub ranking_score_details: Option<Map<String, Value>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// A struct containing search results and other information about the search.
pub struct SimilarResults<T> {
    /// Results of the query.
    pub hits: Vec<SimilarResult<T>>,
    /// Number of documents skipped.
    pub offset: Option<usize>,
    /// Number of results returned.
    pub limit: Option<usize>,
    /// Estimated total number of matches.
    pub estimated_total_hits: Option<usize>,
    /// Processing time of the query.
    pub processing_time_ms: usize,
    /// Search Doc ID
    pub id: String,
}

/// A struct representing a query.
///
/// You can add similar parameters using the builder syntax.
///
/// See [this page](https://www.meilisearch.com/docs/reference/api/similar#get-similar-documents-with-post) for the official list and description of all parameters.
///
/// # Examples
///
/// ```
/// # use serde::{Serialize, Deserialize};
/// # use meilisearch_sdk::{client::Client, search::*, indexes::Index};
/// #
/// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
/// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
/// #
/// #[derive(Serialize, Deserialize, Debug)]
/// struct Movie {
///     name: String,
///     description: String,
/// }
/// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
/// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
/// # let index = client
/// #  .create_index("similar_query_builder", None)
/// #  .await
/// #  .unwrap()
/// #  .wait_for_completion(&client, None, None)
/// #  .await.unwrap()
/// #  .try_make_index(&client)
/// #  .unwrap();
///
/// let mut res = SimilarQuery::new(&index, "100", "default")
///     .execute::<Movie>()
///     .await
///     .unwrap();
///
/// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
/// # });

/// ```
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SimilarQuery<'a, Http: HttpClient> {
    #[serde(skip_serializing)]
    index: &'a Index<Http>,
    /// Document id
    pub id: &'a str,
    /// embedder name
    pub embedder: &'a str,
    /// The number of documents to skip.
    /// If the value of the parameter `offset` is `n`, the `n` first documents (ordered by relevance) will not be returned.
    /// This is helpful for pagination.
    ///
    /// Example: If you want to skip the first document, set offset to `1`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    /// The maximum number of documents returned.
    ///
    /// If the value of the parameter `limit` is `n`, there will never be more than `n` documents in the response.
    /// This is helpful for pagination.
    ///
    /// Example: If you don't want to get more than two documents, set limit to `2`.
    ///
    /// **Default: `20`**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    /// Filter applied to documents.
    ///
    /// Read the [dedicated guide](https://www.meilisearch.com/docs/learn/advanced/filtering) to learn the syntax.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Filter<'a>>,
    /// Attributes to display in the returned documents.
    ///
    /// Can be set to a [wildcard value](enum.Selectors.html#variant.All) that will select all existing attributes.
    ///
    /// **Default: all attributes found in the documents.**
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_with_wildcard")]
    pub attributes_to_retrieve: Option<Selectors<&'a [&'a str]>>,

    /// Defines whether to show the relevancy score of the match.
    ///
    /// **Default: `false`**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_ranking_score: Option<bool>,

    ///Adds a detailed global ranking score field to each document.
    ///
    /// **Default: `false`**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_ranking_score_details: Option<bool>,

    ///Excludes results below the specified ranking score.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranking_score_threshold: Option<f64>,

    /// **Default: `false`**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieve_vectors: Option<bool>,
}

#[allow(missing_docs)]
impl<'a, Http: HttpClient> SimilarQuery<'a, Http> {
    #[must_use]
    pub fn new(index: &'a Index<Http>, id: &'a str, embedder: &'a str) -> SimilarQuery<'a, Http> {
        SimilarQuery {
            index,
            id,
            embedder,
            offset: None,
            limit: None,
            filter: None,
            attributes_to_retrieve: None,
            show_ranking_score: None,
            show_ranking_score_details: None,
            ranking_score_threshold: None,
            retrieve_vectors: None,
        }
    }

    pub fn with_offset<'b>(&'b mut self, offset: usize) -> &'b mut SimilarQuery<'a, Http> {
        self.offset = Some(offset);
        self
    }
    pub fn with_limit<'b>(&'b mut self, limit: usize) -> &'b mut SimilarQuery<'a, Http> {
        self.limit = Some(limit);
        self
    }
    pub fn with_filter<'b>(&'b mut self, filter: &'a str) -> &'b mut SimilarQuery<'a, Http> {
        self.filter = Some(Filter::new(Either::Left(filter)));
        self
    }
    pub fn with_array_filter<'b>(
        &'b mut self,
        filter: Vec<&'a str>,
    ) -> &'b mut SimilarQuery<'a, Http> {
        self.filter = Some(Filter::new(Either::Right(filter)));
        self
    }
    pub fn with_attributes_to_retrieve<'b>(
        &'b mut self,
        attributes_to_retrieve: Selectors<&'a [&'a str]>,
    ) -> &'b mut SimilarQuery<'a, Http> {
        self.attributes_to_retrieve = Some(attributes_to_retrieve);
        self
    }

    pub fn with_show_ranking_score<'b>(
        &'b mut self,
        show_ranking_score: bool,
    ) -> &'b mut SimilarQuery<'a, Http> {
        self.show_ranking_score = Some(show_ranking_score);
        self
    }

    pub fn with_show_ranking_score_details<'b>(
        &'b mut self,
        show_ranking_score_details: bool,
    ) -> &'b mut SimilarQuery<'a, Http> {
        self.show_ranking_score_details = Some(show_ranking_score_details);
        self
    }

    pub fn with_ranking_score_threshold<'b>(
        &'b mut self,
        ranking_score_threshold: f64,
    ) -> &'b mut SimilarQuery<'a, Http> {
        self.ranking_score_threshold = Some(ranking_score_threshold);
        self
    }
    pub fn build(&mut self) -> SimilarQuery<'a, Http> {
        self.clone()
    }
    /// Execute the query and fetch the results.
    pub async fn execute<T: 'static + DeserializeOwned + Send + Sync>(
        &'a self,
    ) -> Result<SimilarResults<T>, Error> {
        self.index.similar_query::<T>(self).await
    }
}

// TODO: set UserProvided EembdderConfig
// Embedder have not been implemented
// But Now It does't work
// #[cfg(test)]
// mod tests {
//     use std::vec;

//     use super::*;
//     use crate::{client::*, search::*};
//     use meilisearch_test_macro::meilisearch_test;
//     use serde::{Deserialize, Serialize};
//     use std::collections::HashMap;

//     #[derive(Debug, Serialize, Deserialize, PartialEq)]
//     struct Nested {
//         child: String,
//     }

//     #[derive(Debug, Serialize, Deserialize, PartialEq)]
//     struct Document {
//         id: usize,
//         title: String,
//         _vectors: HashMap<String, Vec<f64>>,
//     }

//     async fn setup_test_vector_index(client: &Client, index: &Index) -> Result<(), Error> {
//         let v = vec![0.5, 0.5];
//         let mut vectors = HashMap::new();

//         vectors.insert("default".to_string(), v.clone());

//         let t0 = index
//             .add_documents(
//                 &[
//                     Document {
//                         id: 0,
//                         title: "text".into(),
//                         _vectors: vectors.clone(),
//                     },
//                     Document {
//                         id: 1,
//                         title: "text".into(),
//                         _vectors: vectors.clone(),
//                     },
//                     Document {
//                         id: 2,
//                         title: "title".into(),
//                         _vectors: vectors.clone(),
//                     },
//                     Document {
//                         id: 3,
//                         title: "title".into(),
//                         _vectors: vectors.clone(),
//                     },
//                     Document {
//                         id: 4,
//                         title: "title".into(),
//                         _vectors: vectors.clone(),
//                     },
//                     Document {
//                         id: 5,
//                         title: "title".into(),
//                         _vectors: vectors.clone(),
//                     },
//                     Document {
//                         id: 6,
//                         title: "title".into(),
//                         _vectors: vectors.clone(),
//                     },
//                     Document {
//                         id: 7,
//                         title: "title".into(),
//                         _vectors: vectors.clone(),
//                     },
//                     Document {
//                         id: 8,
//                         title: "title".into(),
//                         _vectors: vectors.clone(),
//                     },
//                     Document {
//                         id: 9,
//                         title: "title".into(),
//                         _vectors: vectors.clone(),
//                     },
//                 ],
//                 None,
//             )
//             .await?;

//         let t1 = index.set_filterable_attributes(["title"]).await?;
//         t1.wait_for_completion(client, None, None).await?;
//         t0.wait_for_completion(client, None, None).await?;
//         Ok(())
//     }

//     #[meilisearch_test]
//     async fn test_similar_builder(_client: Client, index: Index) -> Result<(), Error> {
//         let mut query = SimilarQuery::new(&index, "1", "default");
//         query.with_offset(1).with_limit(1);

//         Ok(())
//     }

//     #[meilisearch_test]
//     async fn test_query_limit(client: Client, index: Index) -> Result<(), Error> {
//         setup_test_vector_index(&client, &index).await?;

//         let mut query = SimilarQuery::new(&index, "1", "default");
//         query.with_limit(5);

//         let results: SimilarResults<Document> = query.execute().await?;
//         assert_eq!(results.hits.len(), 5);
//         Ok(())
//     }

//     #[meilisearch_test]
//     async fn test_query_offset(client: Client, index: Index) -> Result<(), Error> {
//         setup_test_vector_index(&client, &index).await?;
//         let mut query = SimilarQuery::new(&index, "1", "default");
//         query.with_offset(6);

//         let results: SimilarResults<Document> = query.execute().await?;
//         assert_eq!(results.hits.len(), 3);
//         Ok(())
//     }

//     #[meilisearch_test]
//     async fn test_query_filter(client: Client, index: Index) -> Result<(), Error> {
//         setup_test_vector_index(&client, &index).await?;

//         let mut query = SimilarQuery::new(&index, "1", "default");

//         let results: SimilarResults<Document> =
//             query.with_filter("title = \"title\"").execute().await?;
//         assert_eq!(results.hits.len(), 8);

//         let results: SimilarResults<Document> =
//             query.with_filter("NOT title = \"title\"").execute().await?;
//         assert_eq!(results.hits.len(), 2);
//         Ok(())
//     }

//     #[meilisearch_test]
//     async fn test_query_filter_with_array(client: Client, index: Index) -> Result<(), Error> {
//         setup_test_vector_index(&client, &index).await?;
//         let mut query = SimilarQuery::new(&index, "1", "default");
//         let results: SimilarResults<Document> = query
//             .with_array_filter(vec!["title = \"title\"", "title = \"text\""])
//             .execute()
//             .await?;
//         assert_eq!(results.hits.len(), 10);

//         Ok(())
//     }

//     #[meilisearch_test]
//     async fn test_query_attributes_to_retrieve(client: Client, index: Index) -> Result<(), Error> {
//         setup_test_vector_index(&client, &index).await?;
//         let mut query = SimilarQuery::new(&index, "1", "default");
//         let results: SimilarResults<Document> = query
//             .with_attributes_to_retrieve(Selectors::All)
//             .execute()
//             .await?;
//         assert_eq!(results.hits.len(), 10);

//         let mut query = SimilarQuery::new(&index, "1", "default");
//         query.with_attributes_to_retrieve(Selectors::Some(&["title", "id"])); // omit the "value" field
//         assert!(query.execute::<Document>().await.is_err()); // error: missing "value" field
//         Ok(())
//     }

//     #[meilisearch_test]
//     async fn test_query_show_ranking_score(client: Client, index: Index) -> Result<(), Error> {
//         setup_test_vector_index(&client, &index).await?;

//         let mut query = SimilarQuery::new(&index, "1", "default");
//         query.with_show_ranking_score(true);
//         let results: SimilarResults<Document> = query.execute().await?;
//         assert!(results.hits[0].ranking_score.is_some());
//         Ok(())
//     }

//     #[meilisearch_test]
//     async fn test_query_show_ranking_score_details(
//         client: Client,
//         index: Index,
//     ) -> Result<(), Error> {
//         setup_test_vector_index(&client, &index).await?;

//         let mut query = SimilarQuery::new(&index, "1", "default");
//         query.with_show_ranking_score_details(true);
//         let results: SimilarResults<Document> = query.execute().await?;
//         assert!(results.hits[0].ranking_score_details.is_some());
//         Ok(())
//     }

//     #[meilisearch_test]
//     async fn test_query_show_ranking_score_threshold(
//         client: Client,
//         index: Index,
//     ) -> Result<(), Error> {
//         setup_test_vector_index(&client, &index).await?;
//         let mut query = SimilarQuery::new(&index, "1", "default");
//         query.with_ranking_score_threshold(1.0);
//         let results: SimilarResults<Document> = query.execute().await?;
//         assert!(results.hits.is_empty());
//         Ok(())
//     }
// }
