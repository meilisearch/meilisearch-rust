use crate::{
    client::Client,
    errors::Error,
    request::{HttpClient, Method},
};
use serde::{Deserialize, Serialize};

/// Struct representing the experimental features result from the API.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExperimentalFeaturesResult {
    pub metrics: bool,
    pub logs_route: bool,
    pub contains_filter: bool,
    pub network: bool,
    pub edit_documents_by_function: bool,
    #[serde(default)]
    pub multimodal: bool,
}

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains_filter: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs_route: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_documents_by_function: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multimodal: Option<bool>,
}

impl<'a, Http: HttpClient> ExperimentalFeatures<'a, Http> {
    #[must_use]
    pub fn new(client: &'a Client<Http>) -> Self {
        ExperimentalFeatures {
            client,
            metrics: None,
            logs_route: None,
            network: None,
            contains_filter: None,
            edit_documents_by_function: None,
            multimodal: None,
        }
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
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// let features = ExperimentalFeatures::new(&client);
    /// features.get().await.unwrap();
    /// # });
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

    pub fn set_metrics(&mut self, metrics: bool) -> &mut Self {
        self.metrics = Some(metrics);
        self
    }

    pub fn set_logs_route(&mut self, logs_route: bool) -> &mut Self {
        self.logs_route = Some(logs_route);
        self
    }

    pub fn set_contains_filter(&mut self, contains_filter: bool) -> &mut Self {
        self.contains_filter = Some(contains_filter);
        self
    }

    pub fn set_edit_documents_by_function(
        &mut self,
        edit_documents_by_function: bool,
    ) -> &mut Self {
        self.edit_documents_by_function = Some(edit_documents_by_function);
        self
    }

    pub fn set_network(&mut self, network: bool) -> &mut Self {
        self.network = Some(network);
        self
    }

    pub fn set_multimodal(&mut self, multimodal: bool) -> &mut Self {
        self.multimodal = Some(multimodal);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use meilisearch_test_macro::meilisearch_test;

    #[meilisearch_test]
    async fn test_experimental_features(client: Client) {
        let mut features = ExperimentalFeatures::new(&client);
        features.set_metrics(true);
        features.set_logs_route(true);
        features.set_contains_filter(true);
        features.set_network(true);
        features.set_edit_documents_by_function(true);
        features.set_multimodal(true);
        let _ = features.update().await.unwrap();

        let res = features.get().await.unwrap();
        assert!(res.metrics);
        assert!(res.logs_route);
        assert!(res.contains_filter);
        assert!(res.network);
        assert!(res.edit_documents_by_function);
        assert!(res.multimodal);
    }
}
