use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::HashMap, time::Duration};
use time::OffsetDateTime;

use crate::{
    errors::*,
    indexes::*,
    key::{Key, KeyBuilder, KeyUpdater, KeysQuery, KeysResults},
    request::*,
    task_info::TaskInfo,
    tasks::{Task, TasksCancelQuery, TasksDeleteQuery, TasksResults, TasksSearchQuery},
    utils::async_sleep,
};

/// The top-level struct of the SDK, representing a client containing [indexes](../indexes/struct.Index.html).
#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) host: String,
    pub(crate) api_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapIndexes {
    pub indexes: (String, String),
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
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// // create the client
    /// let client = Client::new(MEILISEARCH_URL, MEILISEARCH_API_KEY);
    /// ```
    pub fn new(host: impl Into<String>, api_key: impl Into<String>) -> Client {
        Client {
            host: host.into(),
            api_key: api_key.into(),
        }
    }

    fn parse_indexes_results_from_value(&self, value: Value) -> Result<IndexesResults, Error> {
        let raw_indexes = value["results"].as_array().unwrap();

        let indexes_results = IndexesResults {
            limit: value["limit"].as_u64().unwrap() as u32,
            offset: value["offset"].as_u64().unwrap() as u32,
            total: value["total"].as_u64().unwrap() as u32,
            results: raw_indexes
                .iter()
                .map(|raw_index| Index::from_value(raw_index.clone(), self.clone()))
                .collect::<Result<_, _>>()?,
        };

        Ok(indexes_results)
    }

    /// List all [Index]es with query parameters and returns values as instances of [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// // create the client
    /// let client = Client::new(MEILISEARCH_URL, MEILISEARCH_API_KEY);
    ///
    /// let indexes: IndexesResults = client.list_all_indexes().await.unwrap();
    /// println!("{:?}", indexes);
    /// # });
    /// ```
    pub async fn list_all_indexes(&self) -> Result<IndexesResults, Error> {
        let value = self.list_all_indexes_raw().await?;
        let indexes_results = self.parse_indexes_results_from_value(value)?;
        Ok(indexes_results)
    }

    /// List all [Index]es and returns values as instances of [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// // create the client
    /// let client = Client::new(MEILISEARCH_URL, MEILISEARCH_API_KEY);
    /// let mut query = IndexesQuery::new(&client);
    /// query.with_limit(1);
    /// let indexes: IndexesResults = client.list_all_indexes_with(&query).await.unwrap();
    ///
    /// assert_eq!(indexes.limit, 1);
    /// # });
    /// ```
    pub async fn list_all_indexes_with(
        &self,
        indexes_query: &IndexesQuery<'_>,
    ) -> Result<IndexesResults, Error> {
        let value = self.list_all_indexes_raw_with(indexes_query).await?;
        let indexes_results = self.parse_indexes_results_from_value(value)?;

        Ok(indexes_results)
    }

    /// List all [Index]es and returns as Json.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// // create the client
    /// let client = Client::new(MEILISEARCH_URL, MEILISEARCH_API_KEY);
    ///
    /// let json_indexes = client.list_all_indexes_raw().await.unwrap();
    /// println!("{:?}", json_indexes);
    /// # });
    /// ```
    pub async fn list_all_indexes_raw(&self) -> Result<Value, Error> {
        let json_indexes = request::<(), (), Value>(
            &format!("{}/indexes", self.host),
            &self.api_key,
            Method::Get { query: () },
            200,
        )
        .await?;

        Ok(json_indexes)
    }

    /// List all [Index]es with query parameters and returns as Json.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// // create the client
    /// let client = Client::new(MEILISEARCH_URL, MEILISEARCH_API_KEY);
    ///
    /// let mut query = IndexesQuery::new(&client);
    /// query.with_limit(1);
    /// let json_indexes = client.list_all_indexes_raw_with(&query).await.unwrap();
    ///
    /// println!("{:?}", json_indexes);
    /// # });
    /// ```
    pub async fn list_all_indexes_raw_with(
        &self,
        indexes_query: &IndexesQuery<'_>,
    ) -> Result<Value, Error> {
        let json_indexes = request::<&IndexesQuery, (), Value>(
            &format!("{}/indexes", self.host),
            &self.api_key,
            Method::Get {
                query: indexes_query,
            },
            200,
        )
        .await?;

        Ok(json_indexes)
    }

    /// Get an [Index], this index should already exist.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// // create the client
    /// let client = Client::new(MEILISEARCH_URL, MEILISEARCH_API_KEY);
    /// # let index = client.create_index("get_index", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap().try_make_index(&client).unwrap();
    ///
    /// // get the index named "get_index"
    /// let index = client.get_index("get_index").await.unwrap();
    /// assert_eq!(index.as_ref(), "get_index");
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_index(&self, uid: impl AsRef<str>) -> Result<Index, Error> {
        let mut idx = self.index(uid.as_ref());
        idx.fetch_info().await?;
        Ok(idx)
    }

    /// Get a raw JSON [Index], this index should already exist.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// // create the client
    /// let client = Client::new(MEILISEARCH_URL, MEILISEARCH_API_KEY);
    /// # let index = client.create_index("get_raw_index", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap().try_make_index(&client).unwrap();
    ///
    /// // get the index named "get_raw_index"
    /// let raw_index = client.get_raw_index("get_raw_index").await.unwrap();
    /// assert_eq!(raw_index.get("uid").unwrap().as_str().unwrap(), "get_raw_index");
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    /// If you use it directly from an [Index], you can use the method [Index::fetch_info], which is the equivalent method from an index.
    pub async fn get_raw_index(&self, uid: impl AsRef<str>) -> Result<Value, Error> {
        request::<(), (), Value>(
            &format!("{}/indexes/{}", self.host, uid.as_ref()),
            &self.api_key,
            Method::Get { query: () },
            200,
        )
        .await
    }

    /// Create a corresponding object of an [Index] without any check or doing an HTTP call.
    pub fn index(&self, uid: impl Into<String>) -> Index {
        Index::new(uid, self.clone())
    }

    /// Create an [Index].
    /// The second parameter will be used as the primary key of the new index.
    /// If it is not specified, Meilisearch will **try** to infer the primary key.
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// // Create the client
    /// let client = Client::new(MEILISEARCH_URL, MEILISEARCH_API_KEY);
    ///
    /// // Create a new index called movies and access it
    /// let task = client.create_index("create_index", None).await.unwrap();
    ///
    /// // Wait for the task to complete
    /// let task = task.wait_for_completion(&client, None, None).await.unwrap();
    ///
    /// // Try to get the inner index if the task succeeded
    /// let index = task.try_make_index(&client).unwrap();
    ///
    /// assert_eq!(index.as_ref(), "create_index");
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn create_index(
        &self,
        uid: impl AsRef<str>,
        primary_key: Option<&str>,
    ) -> Result<TaskInfo, Error> {
        request::<(), Value, TaskInfo>(
            &format!("{}/indexes", self.host),
            &self.api_key,
            Method::Post {
                query: (),
                body: json!({
                    "uid": uid.as_ref(),
                    "primaryKey": primary_key,
                }),
            },
            202,
        )
        .await
    }

    /// Delete an index from its UID.
    /// To delete an [Index], use the [Index::delete] method.
    pub async fn delete_index(&self, uid: impl AsRef<str>) -> Result<TaskInfo, Error> {
        request::<(), (), TaskInfo>(
            &format!("{}/indexes/{}", self.host, uid.as_ref()),
            &self.api_key,
            Method::Delete { query: () },
            202,
        )
        .await
    }

    /// Alias for [Client::list_all_indexes].
    pub async fn get_indexes(&self) -> Result<IndexesResults, Error> {
        self.list_all_indexes().await
    }

    /// Alias for [Client::list_all_indexes_with].
    pub async fn get_indexes_with(
        &self,
        indexes_query: &IndexesQuery<'_>,
    ) -> Result<IndexesResults, Error> {
        self.list_all_indexes_with(indexes_query).await
    }

    /// Alias for [Client::list_all_indexes_raw].
    pub async fn get_indexes_raw(&self) -> Result<Value, Error> {
        self.list_all_indexes_raw().await
    }

    /// Alias for [Client::list_all_indexes_raw_with].
    pub async fn get_indexes_raw_with(
        &self,
        indexes_query: &IndexesQuery<'_>,
    ) -> Result<Value, Error> {
        self.list_all_indexes_raw_with(indexes_query).await
    }

    /// Swaps a list of two [Index]'es.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// // Create the client
    /// let client = Client::new(MEILISEARCH_URL, MEILISEARCH_API_KEY);
    ///
    /// let task_index_1 = client.create_index("swap_index_1", None).await.unwrap();
    /// let task_index_2 = client.create_index("swap_index_2", None).await.unwrap();
    ///
    /// // Wait for the task to complete
    /// task_index_2.wait_for_completion(&client, None, None).await.unwrap();
    ///
    /// let task = client
    /// .swap_indexes([&SwapIndexes {
    ///     indexes: (
    ///         "swap_index_1".to_string(),
    ///         "swap_index_2".to_string(),
    ///     ),
    /// }])
    /// .await
    /// .unwrap();
    ///
    /// # client.index("swap_index_1").delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # client.index("swap_index_2").delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn swap_indexes(
        &self,
        indexes: impl IntoIterator<Item = &SwapIndexes>,
    ) -> Result<TaskInfo, Error> {
        request::<(), Vec<&SwapIndexes>, TaskInfo>(
            &format!("{}/swap-indexes", self.host),
            &self.api_key,
            Method::Post {
                query: (),
                body: indexes.into_iter().collect(),
            },
            202,
        )
        .await
    }

    /// Get stats of all indexes.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// let client = Client::new(MEILISEARCH_URL, MEILISEARCH_API_KEY);
    /// let stats = client.get_stats().await.unwrap();
    /// # });
    /// ```
    pub async fn get_stats(&self) -> Result<ClientStats, Error> {
        request::<(), (), ClientStats>(
            &format!("{}/stats", self.host),
            &self.api_key,
            Method::Get { query: () },
            200,
        )
        .await
    }

    /// Get health of Meilisearch server.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, errors::{Error, ErrorCode}};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// let client = Client::new(MEILISEARCH_URL, MEILISEARCH_API_KEY);
    /// let health = client.health().await.unwrap();
    /// assert_eq!(health.status, "available");
    /// # });
    /// ```
    pub async fn health(&self) -> Result<Health, Error> {
        request::<(), (), Health>(
            &format!("{}/health", self.host),
            &self.api_key,
            Method::Get { query: () },
            200,
        )
        .await
    }

    /// Get health of Meilisearch server, return true or false.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::client::*;
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// let client = Client::new(MEILISEARCH_URL, MEILISEARCH_API_KEY);
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

    /// Get the API [Key]s from Meilisearch with parameters.
    /// See the [meilisearch documentation](https://docs.meilisearch.com/reference/api/keys.html#get-all-keys).
    ///
    /// See also [Client::create_key] and [Client::get_key].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, errors::Error, key::KeysQuery};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// let client = Client::new(MEILISEARCH_URL, MEILISEARCH_API_KEY);
    /// let mut query = KeysQuery::new();
    /// query.with_limit(1);
    /// let keys = client.get_keys_with(&query).await.unwrap();
    ///
    /// assert_eq!(keys.results.len(), 1);
    /// # });
    /// ```
    pub async fn get_keys_with(&self, keys_query: &KeysQuery) -> Result<KeysResults, Error> {
        let keys = request::<&KeysQuery, (), KeysResults>(
            &format!("{}/keys", self.host),
            &self.api_key,
            Method::Get { query: keys_query },
            200,
        )
        .await?;

        Ok(keys)
    }

    /// Get the API [Key]s from Meilisearch.
    /// See the [meilisearch documentation](https://docs.meilisearch.com/reference/api/keys.html#get-all-keys).
    ///
    /// See also [Client::create_key] and [Client::get_key].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, errors::Error, key::KeyBuilder};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// let client = Client::new(MEILISEARCH_URL, MEILISEARCH_API_KEY);
    /// let keys = client.get_keys().await.unwrap();
    ///
    /// assert_eq!(keys.limit, 20);
    /// # });
    /// ```
    pub async fn get_keys(&self) -> Result<KeysResults, Error> {
        let keys = request::<(), (), KeysResults>(
            &format!("{}/keys", self.host),
            &self.api_key,
            Method::Get { query: () },
            200,
        )
        .await?;

        Ok(keys)
    }

    /// Get one API [Key] from Meilisearch.
    /// See the [meilisearch documentation](https://docs.meilisearch.com/reference/api/keys.html#get-one-key).
    ///
    /// See also [Client::create_key] and [Client::get_keys].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, errors::Error, key::KeyBuilder};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// let client = Client::new(MEILISEARCH_URL, MEILISEARCH_API_KEY);
    /// # let key = client.get_keys().await.unwrap().results.into_iter()
    ///     .find(|k| k.name.as_ref().map_or(false, |name| name.starts_with("Default Search API Key")))
    ///     .unwrap();
    ///
    /// let key = client.get_key(key).await.unwrap();
    ///
    /// assert_eq!(key.name, Some("Default Search API Key".to_string()));
    /// # });
    /// ```
    pub async fn get_key(&self, key: impl AsRef<str>) -> Result<Key, Error> {
        request::<(), (), Key>(
            &format!("{}/keys/{}", self.host, key.as_ref()),
            &self.api_key,
            Method::Get { query: () },
            200,
        )
        .await
    }

    /// Delete an API [Key] from Meilisearch.
    /// See the [meilisearch documentation](https://docs.meilisearch.com/reference/api/keys.html#delete-a-key).
    ///
    /// See also [Client::create_key], [Client::update_key] and [Client::get_key].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, errors::Error, key::KeyBuilder};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// let client = Client::new(MEILISEARCH_URL, MEILISEARCH_API_KEY);
    /// let key = KeyBuilder::new();
    /// let key = client.create_key(key).await.unwrap();
    /// let inner_key = key.key.clone();
    ///
    /// client.delete_key(key).await.unwrap();
    ///
    /// let keys = client.get_keys().await.unwrap();
    /// assert!(keys.results.iter().all(|key| key.key != inner_key));
    /// # });
    /// ```
    pub async fn delete_key(&self, key: impl AsRef<str>) -> Result<(), Error> {
        request::<(), (), ()>(
            &format!("{}/keys/{}", self.host, key.as_ref()),
            &self.api_key,
            Method::Delete { query: () },
            204,
        )
        .await
    }

    /// Create an API [Key] in Meilisearch.
    /// See the [meilisearch documentation](https://docs.meilisearch.com/reference/api/keys.html#create-a-key).
    ///
    /// See also [Client::update_key], [Client::delete_key] and [Client::get_key].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, errors::Error, key::KeyBuilder, key::Action};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// let client = Client::new(MEILISEARCH_URL, MEILISEARCH_API_KEY);
    /// let name = "create_key".to_string();
    /// let mut key = KeyBuilder::new();
    /// key.with_name(&name);
    ///
    /// let key = client.create_key(key).await.unwrap();
    /// assert_eq!(key.name, Some(name));
    /// # client.delete_key(key).await.unwrap();
    /// # });
    /// ```
    pub async fn create_key(&self, key: impl AsRef<KeyBuilder>) -> Result<Key, Error> {
        request::<(), &KeyBuilder, Key>(
            &format!("{}/keys", self.host),
            &self.api_key,
            Method::Post {
                query: (),
                body: key.as_ref(),
            },
            201,
        )
        .await
    }

    /// Update an API [Key] in Meilisearch.
    /// See the [meilisearch documentation](https://docs.meilisearch.com/reference/api/keys.html#update-a-key).
    ///
    /// See also [Client::create_key], [Client::delete_key] and [Client::get_key].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, errors::Error, key::KeyBuilder, key::KeyUpdater};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// let client = Client::new(MEILISEARCH_URL, MEILISEARCH_API_KEY);
    /// let new_key = KeyBuilder::new();
    /// let name = "my name".to_string();
    /// let mut new_key = client.create_key(new_key).await.unwrap();
    /// let mut key_update = KeyUpdater::new(new_key);
    /// key_update.with_name(&name);
    ///
    /// let key = client.update_key(key_update).await.unwrap();
    /// assert_eq!(key.name, Some(name));
    /// # client.delete_key(key).await.unwrap();
    /// # });
    /// ```
    pub async fn update_key(&self, key: impl AsRef<KeyUpdater>) -> Result<Key, Error> {
        request::<(), &KeyUpdater, Key>(
            &format!("{}/keys/{}", self.host, key.as_ref().key),
            &self.api_key,
            Method::Patch {
                body: key.as_ref(),
                query: (),
            },
            200,
        )
        .await
    }

    /// Get version of the Meilisearch server.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// let client = Client::new(MEILISEARCH_URL, MEILISEARCH_API_KEY);
    /// let version = client.get_version().await.unwrap();
    /// # });
    /// ```
    pub async fn get_version(&self) -> Result<Version, Error> {
        request::<(), (), Version>(
            &format!("{}/version", self.host),
            &self.api_key,
            Method::Get { query: () },
            200,
        )
        .await
    }

    /// Wait until Meilisearch processes a [Task], and get its status.
    ///
    /// `interval` = The frequency at which the server should be polled. Default = 50ms
    /// `timeout` = The maximum time to wait for processing to complete. Default = 5000ms
    ///
    /// If the waited time exceeds `timeout` then an [Error::Timeout] will be returned.
    ///
    /// See also [Index::wait_for_task, Task::wait_for_completion, TaskInfo::wait_for_completion].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, tasks::Task};
    /// # use serde::{Serialize, Deserialize};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// #
    /// # #[derive(Debug, Serialize, Deserialize, PartialEq)]
    /// # struct Document {
    /// #    id: usize,
    /// #    value: String,
    /// #    kind: String,
    /// # }
    /// #
    /// #
    /// # futures::executor::block_on(async move {
    /// let client = Client::new(MEILISEARCH_URL, MEILISEARCH_API_KEY);
    /// let movies = client.index("movies_client_wait_for_task");
    ///
    /// let task = movies.add_documents(&[
    ///     Document { id: 0, kind: "title".into(), value: "The Social Network".to_string() },
    ///     Document { id: 1, kind: "title".into(), value: "Harry Potter and the Sorcerer's Stone".to_string() },
    /// ], None).await.unwrap();
    ///
    /// let status = client.wait_for_task(task, None, None).await.unwrap();
    ///
    /// assert!(matches!(status, Task::Succeeded { .. }));
    /// # movies.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn wait_for_task(
        &self,
        task_id: impl AsRef<u32>,
        interval: Option<Duration>,
        timeout: Option<Duration>,
    ) -> Result<Task, Error> {
        let interval = interval.unwrap_or_else(|| Duration::from_millis(50));
        let timeout = timeout.unwrap_or_else(|| Duration::from_millis(5000));

        let mut elapsed_time = Duration::new(0, 0);
        let mut task_result: Result<Task, Error>;

        while timeout > elapsed_time {
            task_result = self.get_task(&task_id).await;
            match task_result {
                Ok(status) => match status {
                    Task::Failed { .. } | Task::Succeeded { .. } => {
                        return self.get_task(task_id).await;
                    }
                    Task::Enqueued { .. } | Task::Processing { .. } => {
                        elapsed_time += interval;
                        async_sleep(interval).await;
                    }
                },
                Err(error) => return Err(error),
            };
        }

        Err(Error::Timeout)
    }

    /// Get a task from the server given a task id.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::*;
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = client::Client::new(MEILISEARCH_URL, MEILISEARCH_API_KEY);
    /// # let index = client.create_index("movies_get_task", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap().try_make_index(&client).unwrap();
    /// let task = index.delete_all_documents().await.unwrap();
    /// let task = client.get_task(task).await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_task(&self, task_id: impl AsRef<u32>) -> Result<Task, Error> {
        request::<(), (), Task>(
            &format!("{}/tasks/{}", self.host, task_id.as_ref()),
            &self.api_key,
            Method::Get { query: () },
            200,
        )
        .await
    }

    /// Get all tasks with query parameters from the server.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::*;
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = client::Client::new(MEILISEARCH_URL, MEILISEARCH_API_KEY);
    ///
    /// let mut query = tasks::TasksSearchQuery::new(&client);
    /// query.with_index_uids(["get_tasks_with"]);
    /// let tasks = client.get_tasks_with(&query).await.unwrap();
    /// # });
    /// ```
    pub async fn get_tasks_with(
        &self,
        tasks_query: &TasksSearchQuery<'_>,
    ) -> Result<TasksResults, Error> {
        let tasks = request::<&TasksSearchQuery, (), TasksResults>(
            &format!("{}/tasks", self.host),
            &self.api_key,
            Method::Get { query: tasks_query },
            200,
        )
        .await?;

        Ok(tasks)
    }

    /// Cancel tasks with filters [TasksCancelQuery]
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::*;
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = client::Client::new(MEILISEARCH_URL, MEILISEARCH_API_KEY);
    ///
    /// let mut query = tasks::TasksCancelQuery::new(&client);
    /// query.with_index_uids(["movies"]);
    ///
    /// let res = client.cancel_tasks_with(&query).await.unwrap();
    /// # });
    /// ```
    pub async fn cancel_tasks_with(
        &self,
        filters: &TasksCancelQuery<'_>,
    ) -> Result<TaskInfo, Error> {
        let tasks = request::<&TasksCancelQuery, (), TaskInfo>(
            &format!("{}/tasks/cancel", self.host),
            &self.api_key,
            Method::Post {
                query: filters,
                body: (),
            },
            200,
        )
        .await?;

        Ok(tasks)
    }

    /// Delete tasks with filters [TasksDeleteQuery]
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::*;
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = client::Client::new(MEILISEARCH_URL, MEILISEARCH_API_KEY);
    ///
    /// let mut query = tasks::TasksDeleteQuery::new(&client);
    /// query.with_index_uids(["movies"]);
    ///
    /// let res = client.delete_tasks_with(&query).await.unwrap();
    /// # });
    /// ```
    pub async fn delete_tasks_with(
        &self,
        filters: &TasksDeleteQuery<'_>,
    ) -> Result<TaskInfo, Error> {
        let tasks = request::<&TasksDeleteQuery, (), TaskInfo>(
            &format!("{}/tasks", self.host),
            &self.api_key,
            Method::Delete { query: filters },
            200,
        )
        .await?;

        Ok(tasks)
    }

    /// Get all tasks from the server.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::*;
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = client::Client::new(MEILISEARCH_URL, MEILISEARCH_API_KEY);
    /// let tasks = client.get_tasks().await.unwrap();
    ///
    /// # assert!(tasks.results.len() > 0);
    /// # });
    /// ```
    pub async fn get_tasks(&self) -> Result<TasksResults, Error> {
        let tasks = request::<(), (), TasksResults>(
            &format!("{}/tasks", self.host),
            &self.api_key,
            Method::Get { query: () },
            200,
        )
        .await?;

        Ok(tasks)
    }

    /// Generates a new tenant token.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::*;
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = client::Client::new(MEILISEARCH_URL, MEILISEARCH_API_KEY);
    /// let api_key_uid = "76cf8b87-fd12-4688-ad34-260d930ca4f4".to_string();
    /// let token = client.generate_tenant_token(api_key_uid, serde_json::json!(["*"]), None, None).unwrap();
    /// let client = client::Client::new(MEILISEARCH_URL, token);
    /// # });
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub fn generate_tenant_token(
        &self,
        api_key_uid: String,
        search_rules: serde_json::Value,
        api_key: Option<&str>,
        expires_at: Option<OffsetDateTime>,
    ) -> Result<String, Error> {
        let api_key = api_key.unwrap_or(&self.api_key);

        crate::tenant_tokens::generate_tenant_token(api_key_uid, search_rules, api_key, expires_at)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientStats {
    pub database_size: usize,
    #[serde(with = "time::serde::rfc3339::option")]
    pub last_update: Option<OffsetDateTime>,
    pub indexes: HashMap<String, IndexStats>,
}

/// Health of the Meilisearch server.
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

/// Version of a Meilisearch server.
///
/// Example:
///
/// ```
/// # use meilisearch_sdk::{client::*, indexes::*, errors::Error};
/// Version {
///    commit_sha: "b46889b5f0f2f8b91438a08a358ba8f05fc09fc1".to_string(),
///    commit_date: "2019-11-15T09:51:54.278247+00:00".to_string(),
///    pkg_version: "0.1.1".to_string(),
/// };
/// ```
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub commit_sha: String,
    pub commit_date: String,
    pub pkg_version: String,
}

#[cfg(test)]
mod tests {
    use std::mem;

    use big_s::S;
    use mockito::mock;
    use time::OffsetDateTime;

    use meilisearch_test_macro::meilisearch_test;

    use crate::{
        client::*,
        key::{Action, KeyBuilder},
        tasks::TasksSearchQuery,
    };

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Document {
        id: String,
    }

    #[meilisearch_test]
    async fn test_swapping_two_indexes(client: Client) {
        let index_1 = client.index("test_swapping_two_indexes_1");
        let index_2 = client.index("test_swapping_two_indexes_2");

        let t0 = index_1
            .add_documents(
                &[Document {
                    id: "1".to_string(),
                }],
                None,
            )
            .await
            .unwrap();

        index_2
            .add_documents(
                &[Document {
                    id: "2".to_string(),
                }],
                None,
            )
            .await
            .unwrap();

        t0.wait_for_completion(&client, None, None).await.unwrap();

        let task = client
            .swap_indexes([&SwapIndexes {
                indexes: (
                    "test_swapping_two_indexes_1".to_string(),
                    "test_swapping_two_indexes_2".to_string(),
                ),
            }])
            .await
            .unwrap();
        task.wait_for_completion(&client, None, None).await.unwrap();

        let document = index_1.get_document("2").await.unwrap();

        assert_eq!(
            Document {
                id: "2".to_string()
            },
            document
        );
    }

    #[meilisearch_test]
    async fn test_methods_has_qualified_version_as_header() {
        let mock_server_url = &mockito::server_url();
        let path = "/hello";
        let address = &format!("{}{}", mock_server_url, path);
        let user_agent = &*qualified_version();

        let assertions = vec![
            (
                mock("GET", path)
                    .match_header("User-Agent", user_agent)
                    .create(),
                request::<(), (), ()>(address, "", Method::Get { query: () }, 200),
            ),
            (
                mock("POST", path)
                    .match_header("User-Agent", user_agent)
                    .create(),
                request::<(), (), ()>(
                    address,
                    "",
                    Method::Post {
                        query: (),
                        body: {},
                    },
                    200,
                ),
            ),
            (
                mock("DELETE", path)
                    .match_header("User-Agent", user_agent)
                    .create(),
                request::<(), (), ()>(address, "", Method::Delete { query: () }, 200),
            ),
            (
                mock("PUT", path)
                    .match_header("User-Agent", user_agent)
                    .create(),
                request::<(), (), ()>(
                    address,
                    "",
                    Method::Put {
                        query: (),
                        body: (),
                    },
                    200,
                ),
            ),
            (
                mock("PATCH", path)
                    .match_header("User-Agent", user_agent)
                    .create(),
                request::<(), (), ()>(
                    address,
                    "",
                    Method::Patch {
                        query: (),
                        body: (),
                    },
                    200,
                ),
            ),
        ];

        for (m, req) in assertions {
            let _ = req.await;

            m.assert();
            mem::drop(m);
        }
    }

    #[meilisearch_test]
    async fn test_get_tasks(client: Client) {
        let tasks = client.get_tasks().await.unwrap();
        assert!(tasks.limit == 20);
    }

    #[meilisearch_test]
    async fn test_get_tasks_with_params(client: Client) {
        let query = TasksSearchQuery::new(&client);
        let tasks = client.get_tasks_with(&query).await.unwrap();

        assert!(tasks.limit == 20);
    }

    #[meilisearch_test]
    async fn test_get_keys(client: Client) {
        let keys = client.get_keys().await.unwrap();

        assert!(keys.results.len() >= 2);
    }

    #[meilisearch_test]
    async fn test_delete_key(client: Client, name: String) {
        let mut key = KeyBuilder::new();
        key.with_name(&name);
        let key = client.create_key(key).await.unwrap();

        client.delete_key(&key).await.unwrap();
        let keys = KeysQuery::new()
            .with_limit(10000)
            .execute(&client)
            .await
            .unwrap();

        assert!(keys.results.iter().all(|k| k.key != key.key));
    }

    #[meilisearch_test]
    async fn test_error_delete_key(mut client: Client, name: String) {
        // ==> accessing a key that does not exist
        let error = client.delete_key("invalid_key").await.unwrap_err();
        assert!(matches!(
            error,
            Error::Meilisearch(MeilisearchError {
                error_code: ErrorCode::ApiKeyNotFound,
                error_type: ErrorType::InvalidRequest,
                ..
            })
        ));

        // ==> executing the action without enough right
        let mut key = KeyBuilder::new();

        key.with_name(&name);
        let key = client.create_key(key).await.unwrap();
        let master_key = client.api_key.clone();
        // this key has no right
        client.api_key = key.key.clone();
        // with a wrong key
        let error = client.delete_key("invalid_key").await.unwrap_err();
        assert!(matches!(
            error,
            Error::Meilisearch(MeilisearchError {
                error_code: ErrorCode::InvalidApiKey,
                error_type: ErrorType::Auth,
                ..
            })
        ));
        // with a good key
        let error = client.delete_key(&key.key).await.unwrap_err();
        assert!(matches!(
            error,
            Error::Meilisearch(MeilisearchError {
                error_code: ErrorCode::InvalidApiKey,
                error_type: ErrorType::Auth,
                ..
            })
        ));

        // cleanup
        client.api_key = master_key;
        client.delete_key(key).await.unwrap();
    }

    #[meilisearch_test]
    async fn test_create_key(client: Client, name: String) {
        let expires_at = OffsetDateTime::now_utc() + time::Duration::HOUR;
        let mut key = KeyBuilder::new();
        key.with_action(Action::DocumentsAdd)
            .with_name(&name)
            .with_expires_at(expires_at.clone())
            .with_description("a description")
            .with_index("*");
        let key = client.create_key(key).await.unwrap();

        assert_eq!(key.actions, vec![Action::DocumentsAdd]);
        assert_eq!(&key.name, &Some(name));
        // We can't compare the two timestamp directly because of some nanoseconds imprecision with the floats
        assert_eq!(
            key.expires_at.unwrap().unix_timestamp(),
            expires_at.unix_timestamp()
        );
        assert_eq!(key.indexes, vec![S("*")]);

        client.delete_key(key).await.unwrap();
    }

    #[meilisearch_test]
    async fn test_error_create_key(mut client: Client, name: String) {
        // ==> Invalid index name
        /* TODO: uncomment once meilisearch fix this bug: https://github.com/meilisearch/meilisearch/issues/2158
        let mut key = KeyBuilder::new();
        key.with_index("invalid index # / \\name with spaces");
        let error = client.create_key(key).await.unwrap_err();

        assert!(matches!(
            error,
            Error::MeilisearchError {
                error_code: ErrorCode::InvalidApiKeyIndexes,
                error_type: ErrorType::InvalidRequest,
                ..
            }
        ));
        */

        // ==> executing the action without enough right
        let mut no_right_key = KeyBuilder::new();
        no_right_key.with_name(&format!("{name}_1"));
        let no_right_key = client.create_key(no_right_key).await.unwrap();

        // backup the master key for cleanup at the end of the test
        let master_client = client.clone();
        client.api_key = no_right_key.key.clone();

        let mut key = KeyBuilder::new();
        key.with_name(format!("{name}_2"));
        let error = client.create_key(key).await.unwrap_err();

        assert!(matches!(
            error,
            Error::Meilisearch(MeilisearchError {
                error_code: ErrorCode::InvalidApiKey,
                error_type: ErrorType::Auth,
                ..
            })
        ));

        // cleanup
        master_client.delete_key(&*client.api_key).await.unwrap();
    }

    #[meilisearch_test]
    async fn test_update_key(client: Client, description: String) {
        let mut key = KeyBuilder::new();
        key.with_name("test_update_key");
        let mut key = client.create_key(key).await.unwrap();

        let name = S("new name");
        key.with_description(&description);
        key.with_name(&name);

        let key = key.update(&client).await.unwrap();

        assert_eq!(key.description, Some(description));
        assert_eq!(key.name, Some(name));

        client.delete_key(key).await.unwrap();
    }

    #[meilisearch_test]
    async fn test_get_index(client: Client, index_uid: String) -> Result<(), Error> {
        let task = client.create_index(&index_uid, None).await?;
        let index = client
            .wait_for_task(task, None, None)
            .await?
            .try_make_index(&client)
            .unwrap();

        assert_eq!(index.uid, index_uid);
        index
            .delete()
            .await?
            .wait_for_completion(&client, None, None)
            .await?;
        Ok(())
    }

    #[meilisearch_test]
    async fn test_error_create_index(client: Client, index: Index) -> Result<(), Error> {
        let error = client
            .create_index("Wrong index name", None)
            .await
            .unwrap_err();

        assert!(matches!(
            error,
            Error::Meilisearch(MeilisearchError {
                error_code: ErrorCode::InvalidIndexUid,
                error_type: ErrorType::InvalidRequest,
                ..
            })
        ));

        // we try to create an index with the same uid of an already existing index
        let error = client
            .create_index(&*index.uid, None)
            .await?
            .wait_for_completion(&client, None, None)
            .await?
            .unwrap_failure();

        assert!(matches!(
            error,
            MeilisearchError {
                error_code: ErrorCode::IndexAlreadyExists,
                error_type: ErrorType::InvalidRequest,
                ..
            }
        ));
        Ok(())
    }

    #[meilisearch_test]
    async fn test_list_all_indexes(client: Client) {
        let all_indexes = client.list_all_indexes().await.unwrap();

        assert_eq!(all_indexes.limit, 20);
        assert_eq!(all_indexes.offset, 0);
    }

    #[meilisearch_test]
    async fn test_list_all_indexes_with_params(client: Client) {
        let mut query = IndexesQuery::new(&client);
        query.with_limit(1);
        let all_indexes = client.list_all_indexes_with(&query).await.unwrap();

        assert_eq!(all_indexes.limit, 1);
        assert_eq!(all_indexes.offset, 0);
    }

    #[meilisearch_test]
    async fn test_list_all_indexes_raw(client: Client) {
        let all_indexes_raw = client.list_all_indexes_raw().await.unwrap();

        assert_eq!(all_indexes_raw["limit"], json!(20));
        assert_eq!(all_indexes_raw["offset"], json!(0));
    }

    #[meilisearch_test]
    async fn test_list_all_indexes_raw_with_params(client: Client) {
        let mut query = IndexesQuery::new(&client);
        query.with_limit(1);
        let all_indexes_raw = client.list_all_indexes_raw_with(&query).await.unwrap();

        assert_eq!(all_indexes_raw["limit"], json!(1));
        assert_eq!(all_indexes_raw["offset"], json!(0));
    }

    #[meilisearch_test]
    async fn test_get_primary_key_is_none(mut index: Index) {
        let primary_key = index.get_primary_key().await;

        assert!(primary_key.is_ok());
        assert!(primary_key.unwrap().is_none());
    }

    #[meilisearch_test]
    async fn test_get_primary_key(client: Client, index_uid: String) -> Result<(), Error> {
        let mut index = client
            .create_index(index_uid, Some("primary_key"))
            .await?
            .wait_for_completion(&client, None, None)
            .await?
            .try_make_index(&client)
            .unwrap();

        let primary_key = index.get_primary_key().await;
        assert!(primary_key.is_ok());
        assert_eq!(primary_key?.unwrap(), "primary_key");

        index
            .delete()
            .await?
            .wait_for_completion(&client, None, None)
            .await?;

        Ok(())
    }
}
