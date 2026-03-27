use crate::{
    errors::Error,
    indexes::Index,
    request::HttpClient,
    search::{Filter, Selectors},
};
use either::Either;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Deserialize, Debug, Clone)]
pub struct SimilarResult<T> {
    #[serde(flatten)]
    pub result: T,
    #[serde(rename = "_rankingScore")]
    pub ranking_score: Option<f64>,
    #[serde(rename = "_rankingScoreDetails")]
    pub ranking_score_details: Option<Map<String, Value>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SimilarResults<T> {
    /// Results of the query
    pub hits: Vec<SimilarResult<T>>,
    /// Number of documents skipped
    pub offset: Option<usize>,
    /// Number of results returned
    pub limit: Option<usize>,
    /// Estimated total number of matches
    pub estimated_total_hits: Option<usize>,
    /// Performance trace of the query
    pub performance_details: Option<Value>,
    /// Processing time of the query
    pub processing_time_ms: usize,
    /// Identifier of the target document
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
/// ```no_run
/// # use serde::{Serialize, Deserialize};
/// # use meilisearch_sdk::{client::Client, search::*, indexes::Index};
/// #
/// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
/// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
/// #
/// # #[derive(Serialize, Deserialize, Debug)]
/// # struct Movie {
/// #    name: String,
/// #    description: String,
/// # }
/// #
/// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
/// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
/// # let index = client.index("similar_query_builder");
/// #
/// let mut res = index.similar_search("100", "default")
///     .execute::<Movie>()
///     .await
///     .unwrap();
/// #
/// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
/// # });
/// ```
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SimilarQuery<'a, Http: HttpClient> {
    #[serde(skip_serializing)]
    index: &'a Index<Http>,

    /// Identifier of the target document
    pub id: &'a str,

    /// Embedder to use when computing recommendations
    pub embedder: &'a str,

    /// Number of documents to skip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,

    /// Maximum number of documents returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,

    /// Filter queries by an attribute’s value
    ///
    /// Read the [dedicated guide](https://www.meilisearch.com/docs/learn/filtering_and_sorting) to learn the syntax.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Filter<'a>>,

    /// Attributes to display in the returned documents.
    ///
    /// Can be set to a [wildcard value](enum.Selectors.html#variant.All) that will select all existing attributes.
    ///
    /// **Default: all attributes found in the documents.**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_retrieve: Option<Selectors<&'a [&'a str]>>,

    /// Defines whether to display the global ranking score of a document
    ///
    /// **Default: `false`**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_ranking_score: Option<bool>,

    /// Defines whether to display the detailed ranking score information
    ///
    /// **Default: `false`**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_ranking_score_details: Option<bool>,

    /// Defines whether to exclude results with low ranking scores
    ///
    /// **Default: `None`**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranking_score_threshold: Option<f64>,

    /// Defines whether to return document vector data
    ///
    /// **Default: `false`**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieve_vectors: Option<bool>,

    /// Defines whether to return performance trace
    ///
    /// **Default: `false`**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_performance_details: Option<bool>,
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
            show_performance_details: None,
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

    pub fn with_retrieve_vectors<'b>(
        &'b mut self,
        retrieve_vectors: bool,
    ) -> &'b mut SimilarQuery<'a, Http> {
        self.retrieve_vectors = Some(retrieve_vectors);
        self
    }

    /// Request performance trace in the response.
    pub fn with_show_performance_details<'b>(
        &'b mut self,
        show_performance_details: bool,
    ) -> &'b mut SimilarQuery<'a, Http> {
        self.show_performance_details = Some(show_performance_details);
        self
    }

    /// Execute the query and fetch the results.
    pub async fn execute<T: 'static + DeserializeOwned + Send + Sync>(
        &'a self,
    ) -> Result<SimilarResults<T>, Error> {
        self.index.execute_similar_query::<T>(self).await
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use crate::{
        client::*,
        search::{
            tests::{setup_embedder, setup_test_index, Document},
            *,
        },
    };
    use meilisearch_test_macro::meilisearch_test;

    #[meilisearch_test]
    async fn test_similar_results(client: Client, index: Index) -> Result<(), Error> {
        setup_embedder(&client, &index).await?;
        setup_test_index(&client, &index).await?;

        // Test on a non-harry-potter document
        let mut query = SimilarQuery::new(&index, "0", "default");
        query.with_limit(1);
        let results: SimilarResults<Document> = query.execute().await?;
        let result = results.hits.first().unwrap();
        assert_eq!(result.result.id, 1);

        // Test on a harry-potter document
        let mut query = SimilarQuery::new(&index, "3", "default");
        query.with_limit(1);
        let results: SimilarResults<Document> = query.execute().await?;
        let result = results.hits.first().unwrap();
        assert_eq!(result.result.id, 4);

        Ok(())
    }

    #[meilisearch_test]
    async fn test_query_limit(client: Client, index: Index) -> Result<(), Error> {
        setup_embedder(&client, &index).await?;
        setup_test_index(&client, &index).await?;

        let mut query = SimilarQuery::new(&index, "1", "default");
        query.with_limit(3);

        let results: SimilarResults<Document> = query.execute().await?;
        assert_eq!(results.hits.len(), 3);
        Ok(())
    }

    #[meilisearch_test]
    async fn test_query_offset(client: Client, index: Index) -> Result<(), Error> {
        setup_embedder(&client, &index).await?;
        setup_test_index(&client, &index).await?;

        let mut query = SimilarQuery::new(&index, "1", "default");
        query.with_offset(6);

        let results: SimilarResults<Document> = query.execute().await?;
        assert_eq!(results.hits.len(), 3);
        Ok(())
    }

    #[meilisearch_test]
    async fn test_query_filter(client: Client, index: Index) -> Result<(), Error> {
        setup_embedder(&client, &index).await?;
        setup_test_index(&client, &index).await?;

        let mut query = SimilarQuery::new(&index, "1", "default");

        let results: SimilarResults<Document> =
            query.with_filter("kind = \"title\"").execute().await?;
        assert_eq!(results.hits.len(), 8);

        let results: SimilarResults<Document> =
            query.with_filter("NOT kind = \"title\"").execute().await?;
        assert_eq!(results.hits.len(), 1);
        Ok(())
    }

    #[meilisearch_test]
    async fn test_query_filter_with_array(client: Client, index: Index) -> Result<(), Error> {
        setup_embedder(&client, &index).await?;
        setup_test_index(&client, &index).await?;

        let mut query = SimilarQuery::new(&index, "1", "default");
        let results: SimilarResults<Document> = query
            .with_array_filter(vec!["kind = \"title\"", "kind = \"text\""])
            .execute()
            .await?;
        assert_eq!(results.hits.len(), 0);

        let mut query = SimilarQuery::new(&index, "1", "default");
        let results: SimilarResults<Document> = query
            .with_array_filter(vec!["kind = \"title\"", "number <= 50"])
            .execute()
            .await?;
        assert_eq!(results.hits.len(), 4);

        Ok(())
    }

    #[meilisearch_test]
    async fn test_query_attributes_to_retrieve(client: Client, index: Index) -> Result<(), Error> {
        setup_embedder(&client, &index).await?;
        setup_test_index(&client, &index).await?;

        let mut query = SimilarQuery::new(&index, "1", "default");
        let results: SimilarResults<Document> = query
            .with_attributes_to_retrieve(Selectors::All)
            .execute()
            .await?;
        assert_eq!(results.hits.len(), 9);

        let mut query = SimilarQuery::new(&index, "1", "default");
        query.with_attributes_to_retrieve(Selectors::Some(&["title", "id"])); // omit the "value" field
        assert!(query.execute::<Document>().await.is_err()); // error: missing "value" field
        Ok(())
    }

    #[meilisearch_test]
    async fn test_query_show_ranking_score(client: Client, index: Index) -> Result<(), Error> {
        setup_embedder(&client, &index).await?;
        setup_test_index(&client, &index).await?;

        let mut query = SimilarQuery::new(&index, "1", "default");
        query.with_show_ranking_score(true);
        let results: SimilarResults<Document> = query.execute().await?;
        assert!(results.hits[0].ranking_score.is_some());
        Ok(())
    }

    #[meilisearch_test]
    async fn test_query_show_ranking_score_details(
        client: Client,
        index: Index,
    ) -> Result<(), Error> {
        setup_embedder(&client, &index).await?;
        setup_test_index(&client, &index).await?;

        let mut query = SimilarQuery::new(&index, "1", "default");
        query.with_show_ranking_score_details(true);
        let results: SimilarResults<Document> = query.execute().await?;
        assert!(results.hits[0].ranking_score_details.is_some());
        Ok(())
    }

    #[meilisearch_test]
    async fn test_query_show_ranking_score_threshold(
        client: Client,
        index: Index,
    ) -> Result<(), Error> {
        setup_embedder(&client, &index).await?;
        setup_test_index(&client, &index).await?;

        let mut query = SimilarQuery::new(&index, "1", "default");
        query.with_ranking_score_threshold(1.0);
        let results: SimilarResults<Document> = query.execute().await?;
        assert!(results.hits.is_empty());
        Ok(())
    }

    #[meilisearch_test]
    async fn test_query_retrieve_vectors(client: Client, index: Index) -> Result<(), Error> {
        setup_embedder(&client, &index).await?;
        setup_test_index(&client, &index).await?;

        let mut query = SimilarQuery::new(&index, "1", "default");
        query.with_retrieve_vectors(true);
        let results: SimilarResults<Document> = query.execute().await?;
        assert!(results.hits[0].result._vectors.is_some());
        Ok(())
    }

    #[meilisearch_test]
    async fn test_query_show_performance_details(
        client: Client,
        index: Index,
    ) -> Result<(), Error> {
        setup_embedder(&client, &index).await?;
        setup_test_index(&client, &index).await?;

        let mut query = SimilarQuery::new(&index, "1", "default");
        query.with_show_performance_details(true);
        let results: SimilarResults<Document> = query.execute().await?;
        assert!(results.performance_details.is_some());
        Ok(())
    }
}
