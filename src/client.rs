use crate::{config::*, errors::*, indexes::*, request::*};
use serde_json::{json, Value};
use serde::{Deserialize};
use std::collections::HashMap;

/// The top-level struct of the SDK, representing a client containing [indexes](../indexes/struct.Index.html).
#[derive(Debug)]
pub struct Client {
    pub(crate) config: Config,
}

impl Client {
    /// Create a client using the specified server.
    /// Don't put a '/' at the end of the host.
    /// In production mode, see [the documentation about authentication](https://docs.meilisearch.com/reference/features/authentication.html#authentication).
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// // create the client
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// ```
    pub fn new<S: AsRef<str>>(host: S, api_key: S) -> Self {
        Self { config: Config::new(host, api_key) }
    }

    /// List all [indexes](../indexes/struct.Index.html).
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// # futures::executor::block_on(async move {
    /// // create the client
    /// let client = Client::new("http://localhost:7700", "masterKey");
    ///
    /// let indexes: Vec<Index> = client.list_all_indexes().await.unwrap();
    /// println!("{:?}", indexes);
    /// # });
    /// ```
    pub async fn list_all_indexes(&self) -> Result<Vec<Index>, Error> {
        let json_indexes = request::<(), Vec<JsonIndex>>(
            &format!("{}/indexes", self.config.host),
            &self.config.api_key,
            Method::Get,
            200,
        ).await?;

        let mut indexes = Vec::with_capacity(json_indexes.len());
        for json_index in json_indexes {
            indexes.push(json_index.into_index(self.config.clone()))
        }

        Ok(indexes)
    }

    /// Get an [index](../indexes/struct.Index.html).
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    ///
    /// # futures::executor::block_on(async move {
    /// // create the client
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// # client.create_index("movies", None).await;
    ///
    /// // get the index named "movies"
    /// let movies = client.get_index("movies").await.unwrap();
    /// # });
    /// ```
    pub async fn get_index<S: AsRef<str>>(&self, uid: S) -> Result<Index, Error> {
        Ok(request::<(), JsonIndex>(
            &format!("{}/indexes/{}", self.config.host, uid.as_ref()),
            &self.config.api_key,
            Method::Get,
            200,
        ).await?
        .into_index(self.config.clone()))
    }

    /// Assume that an [index](../indexes/struct.Index.html) exist and create a corresponding object without any check.
    pub fn assume_index<S: AsRef<str>>(&self, uid: S) -> Index {
        Index {
            config: self.config.clone(),
            uid: uid.as_ref().into(),
        }
    }

    /// Create an [index](../indexes/struct.Index.html).
    /// The second parameter will be used as the primary key of the new index. If it is not specified, MeiliSearch will **try** to infer the primary key.
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # futures::executor::block_on(async move {
    /// // create the client
    /// let client = Client::new("http://localhost:7700", "masterKey");
    ///
    /// # if let Ok(mut movies) = client.get_index("movies").await {
    /// #   movies.delete().await.unwrap();
    /// # }
    /// // create a new index called movies and access it
    /// let movies = client.create_index("movies", None).await;
    /// # });
    /// ```
    pub async fn create_index<S: AsRef<str>>(
        &self,
        uid: S,
        primary_key: Option<S>,
    ) -> Result<Index, Error> {
        Ok(request::<Value, JsonIndex>(
            &format!("{}/indexes", self.config.host),
            &self.config.api_key,
            Method::Post(json!({
                "uid": uid.as_ref(),
                "primaryKey": primary_key.map(|v|v.as_ref().to_string()),
            })),
            201,
        ).await?
        .into_index(self.config.clone()))
    }

    /// Delete an index from its UID.
    /// To delete an index from the [index object](../indexes/struct.Index.html), use [the delete method](../indexes/struct.Index.html#method.delete).
    pub async fn delete_index<S: AsRef<str>>(&self, uid: S) -> Result<(), Error> {
        Ok(request::<(), ()>(
            &format!("{}/indexes/{}", self.config.host, uid.as_ref()),
            &self.config.api_key,
            Method::Delete,
            204,
        ).await?)
    }

    /// This will try to get an index and create the index if it does not exist.
    pub async fn get_or_create<S: AsRef<str>>(&self, uid: S) -> Result<Index, Error> {
        let uid: String = uid.as_ref().to_string();
        if let Ok(index) = self.get_index(&uid).await {
            Ok(index)
        } else {
            self.create_index(&uid, None).await
        }
    }

    /// Alias for [list_all_indexes](#method.list_all_indexes).
    pub async fn get_indexes(&self) -> Result<Vec<Index>, Error> {
        self.list_all_indexes().await
    }

    /// Get stats of all indexes.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let stats = client.get_stats().await.unwrap();
    /// # });
    /// ```
    pub async fn get_stats(&self) -> Result<ClientStats, Error> {
        request::<serde_json::Value, ClientStats>(
            &format!("{}/stats", self.config.host),
            &self.config.api_key,
            Method::Get,
            200,
        ).await
    }

    /// Get health of MeiliSearch server.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, errors::{Error, ErrorCode}};
    /// #
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let health = client.health().await.unwrap();
    /// # });
    /// ```
    pub async fn health(&self) -> Result<Health, Error> {
        request::<serde_json::Value, Health>(
            &format!("{}/health", self.config.host),
            &self.config.api_key,
            Method::Get,
            200,
        )
        .await
    }

    /// Get health of MeiliSearch server, return true or false.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::client::*;
    /// #
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let health = client.is_healthy().await;
    /// assert_eq!(health, true);
    /// # });
    /// ```
    pub async fn is_healthy(&self) -> bool {
        if let Ok(health) = self.health().await {
            health.status.as_str() == "available"
        } else {
            false
        }
    }

    /// Get the private and public key.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, errors::Error};
    /// #
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let keys = client.get_keys().await.unwrap();
    /// # });
    /// ```
    pub async fn get_keys(&self) -> Result<Keys, Error> {
        request::<(), Keys>(
            &format!("{}/keys", self.config.host),
            &self.config.api_key,
            Method::Get,
            200,
        ).await
    }

    /// Get version of the MeiliSearch server.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, errors::Error};
    /// #
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let version = client.get_version().await.unwrap();
    /// # });
    /// ```
    pub async fn get_version(&self) -> Result<Version, Error> {
        request::<(), Version>(
            &format!("{}/version", self.config.host),
            &self.config.api_key,
            Method::Get,
            200,
        ).await
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientStats {
    pub database_size: usize,
    pub last_update: Option<String>,
    pub indexes: HashMap<String, IndexStats>,
}

/// Health of the MeiliSearch server.
///
/// Example:
///
/// ```
/// # use meilisearch_sdk::{client::*, indexes::*, errors::Error};
/// Health {
///    status: "available".to_string(),
/// };
/// ```
#[derive(Deserialize)]
pub struct Health {
    pub status: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Keys {
    pub public: Option<String>,
    pub private: Option<String>,
}

/// Version of a MeiliSearch server.
///
/// Example:
///
/// ```
/// # use meilisearch_sdk::{client::*, indexes::*, errors::Error};
/// Version {
///    commit_sha: "b46889b5f0f2f8b91438a08a358ba8f05fc09fc1".to_string(),
///    build_date: "2019-11-15T09:51:54.278247+00:00".to_string(),
///    pkg_version: "0.1.1".to_string(),
/// };
/// ```
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub commit_sha: String,
    pub build_date: String,
    pub pkg_version: String,
}

#[cfg(test)]
mod tests {
    use crate::{client::*};
    use futures_await_test::async_test;

    #[async_test]
    async fn test_get_keys() {
        let client = Client::new("http://localhost:7700", "masterKey");
        client.get_keys().await.unwrap();
    }
}
