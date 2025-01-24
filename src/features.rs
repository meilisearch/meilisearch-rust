use crate::{
    client::Client,
    errors::Error,
    request::{HttpClient, Method},
};
use serde::{Deserialize, Serialize};

/// Struct representing the experimental features result from the API.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExperimentalFeaturesResult {}

/// Struct representing the experimental features request.
///
/// You can build this struct using the builder pattern.
///
/// # Example
///
/// ```
/// # use meilisearch_sdk::{client::Client, features::ExperimentalFeatures};
/// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
/// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
/// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
/// let mut features = ExperimentalFeatures::new(&client);
/// ```
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExperimentalFeatures<'a, Http: HttpClient> {
    #[serde(skip_serializing)]
    client: &'a Client<Http>,
}

impl<'a, Http: HttpClient> ExperimentalFeatures<'a, Http> {
    #[must_use]
    pub fn new(client: &'a Client<Http>) -> Self {
        ExperimentalFeatures { client }
    }

    /// Get all the experimental features
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::Client, features::ExperimentalFeatures};
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    ///     let features = ExperimentalFeatures::new(&client);
    ///     features.get().await.unwrap();
    /// });
    /// ```
    pub async fn get(&self) -> Result<ExperimentalFeaturesResult, Error> {
        self.client
            .http_client
            .request::<(), (), ExperimentalFeaturesResult>(
                &format!("{}/experimental-features", self.client.host),
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
    /// # use meilisearch_sdk::{client::Client, features::ExperimentalFeatures};
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// let features = ExperimentalFeatures::new(&client);
    /// features.update().await.unwrap();
    /// # });
    /// ```
    pub async fn update(&self) -> Result<ExperimentalFeaturesResult, Error> {
        self.client
            .http_client
            .request::<(), &Self, ExperimentalFeaturesResult>(
                &format!("{}/experimental-features", self.client.host),
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
        let features = ExperimentalFeatures::new(&client);
        // set feature here, once some exist again
        let _ = features.update().await.unwrap();

        let _res = features.get().await.unwrap();
        // assert that the feature has been set once they exist again
    }
}
