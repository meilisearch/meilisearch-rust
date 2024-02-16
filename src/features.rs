use crate::{
    request::{request, Method},
    Client, Error,
};
use serde::{Deserialize, Serialize};

/// Struct representing the experimental features result from the API.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExperimentalFeaturesResult {
    pub vector_store: bool,
}

/// Struct representing the experimental features request.
///
/// You can build this struct using the builder pattern.
///
/// # Example
///
/// ```
/// # use meilisearch_sdk::{Client, ExperimentalFeatures};
/// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
/// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
/// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY));
/// let mut features = ExperimentalFeatures::new(&client);
/// features.set_vector_store(true);
/// ```
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExperimentalFeatures<'a> {
    #[serde(skip_serializing)]
    client: &'a Client,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_store: Option<bool>,
}

impl<'a> ExperimentalFeatures<'a> {
    pub fn new(client: &'a Client) -> Self {
        ExperimentalFeatures {
            client,
            vector_store: None,
        }
    }

    pub fn set_vector_store(&mut self, vector_store: bool) -> &mut Self {
        self.vector_store = Some(vector_store);
        self
    }

    /// Get all the experimental features
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{Client, ExperimentalFeatures};
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY));
    /// futures::executor::block_on(async move {
    ///     let features = ExperimentalFeatures::new(&client);
    ///     features.get().await.unwrap();
    /// });
    /// ```
    pub async fn get(&self) -> Result<ExperimentalFeaturesResult, Error> {
        request::<(), (), ExperimentalFeaturesResult>(
            &format!("{}/experimental-features", self.client.host),
            self.client.get_api_key(),
            Method::Get { query: () },
            200,
        )
        .await
    }

    /// Update the experimental features
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{Client, ExperimentalFeatures};
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY));
    /// futures::executor::block_on(async move {
    ///     let mut features = ExperimentalFeatures::new(&client);
    ///     features.set_vector_store(true);
    ///     features.update().await.unwrap();
    /// });
    /// ```
    pub async fn update(&self) -> Result<ExperimentalFeaturesResult, Error> {
        request::<(), &Self, ExperimentalFeaturesResult>(
            &format!("{}/experimental-features", self.client.host),
            self.client.get_api_key(),
            Method::Patch {
                query: (),
                body: self,
            },
            200,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use meilisearch_test_macro::meilisearch_test;

    #[meilisearch_test]
    async fn test_experimental_features_get(client: Client) {
        let mut features = ExperimentalFeatures::new(&client);
        features.set_vector_store(false);
        let _ = features.update().await.unwrap();

        let res = features.get().await.unwrap();

        assert!(!res.vector_store);
    }

    #[meilisearch_test]
    async fn test_experimental_features_enable_vector_store(client: Client) {
        let mut features = ExperimentalFeatures::new(&client);
        features.set_vector_store(true);

        let res = features.update().await.unwrap();

        assert!(res.vector_store);
    }
}
