use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{Client, Error};

/// Represents a [meilisearch key](https://www.meilisearch.com/docs/reference/api/keys#returned-fields).
///
/// You can get a [Key] from the [`Client::get_key`] method, or you can create a [Key] with the [`KeyBuilder::new`] or [`Client::create_key`] methods.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Key {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<Action>,
    #[serde(skip_serializing, with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    pub description: Option<String>,
    pub name: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub expires_at: Option<OffsetDateTime>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub indexes: Vec<String>,
    #[serde(skip_serializing)]
    pub key: String,
    #[serde(skip_serializing)]
    pub uid: String,
    #[serde(skip_serializing, with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl Key {
    /// Update the description of the [Key].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{KeyBuilder, Action, Client};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY));
    /// let description = "My not so little lovely test key".to_string();
    /// let mut key = KeyBuilder::new()
    ///     .with_action(Action::DocumentsAdd)
    ///     .with_index("*")
    ///     .with_description(&description)
    ///     .execute(&client).await.unwrap();
    ///
    /// assert_eq!(key.description, Some(description));
    /// # client.delete_key(key).await.unwrap();
    /// # });
    /// ```
    pub fn with_description(&mut self, desc: impl AsRef<str>) -> &mut Key {
        self.description = Some(desc.as_ref().to_string());
        self
    }

    /// Update the name of the [Key].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{KeyBuilder, Action, Client};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY));
    /// let name = "lovely key".to_string();
    /// let mut key = KeyBuilder::new()
    ///     .with_action(Action::DocumentsAdd)
    ///     .with_index("*")
    ///     .execute(&client)
    ///     .await
    ///     .unwrap();
    ///
    /// key.with_name(&name);
    ///
    /// assert_eq!(key.name, Some(name));
    /// # client.delete_key(key).await.unwrap();
    /// # });
    /// ```
    pub fn with_name(&mut self, desc: impl AsRef<str>) -> &mut Key {
        self.name = Some(desc.as_ref().to_string());
        self
    }

    /// Update the [Key].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{KeyBuilder, Client};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY));
    /// let mut key = KeyBuilder::new()
    ///     .execute(&client)
    ///     .await
    ///     .unwrap();
    ///
    /// let description = "My not so little lovely test key".to_string();
    /// key.with_description(&description);
    ///
    /// let key = key.update(&client).await.unwrap();
    ///
    /// assert_eq!(key.description, Some(description));
    /// # client.delete_key(key).await.unwrap();
    /// # });
    /// ```
    pub async fn update(&self, client: &Client) -> Result<Key, Error> {
        // only send description and name
        let mut key_update = KeyUpdater::new(self);

        if let Some(ref description) = self.description {
            key_update.with_description(description);
        }
        if let Some(ref name) = self.name {
            key_update.with_name(name);
        }

        key_update.execute(client).await
    }

    /// Delete the [Key].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{KeyBuilder, Client};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY));
    /// let mut key = KeyBuilder::new()
    ///     .execute(&client).await.unwrap();
    ///
    /// client.delete_key(key).await.unwrap();
    /// # });
    /// ```
    pub async fn delete(&self, client: &Client) -> Result<(), Error> {
        client.delete_key(self).await
    }
}

impl AsRef<str> for Key {
    fn as_ref(&self) -> &str {
        &self.key
    }
}

impl AsRef<Key> for Key {
    fn as_ref(&self) -> &Key {
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KeyUpdater {
    pub description: Option<String>,
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub key: String,
}

impl KeyUpdater {
    pub fn new(key_or_uid: impl AsRef<str>) -> KeyUpdater {
        KeyUpdater {
            description: None,
            name: None,
            key: key_or_uid.as_ref().to_string(),
        }
    }

    /// Update the description of the [Key].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{KeyBuilder, Action, Client, KeyUpdater};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY));
    /// let mut new_key = KeyBuilder::new()
    ///     .execute(&client)
    ///     .await
    ///     .unwrap();
    ///
    /// let description = "My not so little lovely test key".to_string();
    /// let mut key_update = KeyUpdater::new(new_key)
    ///     .with_description(&description)
    ///     .execute(&client)
    ///     .await
    ///     .unwrap();
    ///
    /// assert_eq!(key_update.description, Some(description));
    /// # client.delete_key(key_update).await.unwrap();
    /// # });
    /// ```
    pub fn with_description(&mut self, desc: impl AsRef<str>) -> &mut KeyUpdater {
        self.description = Some(desc.as_ref().to_string());
        self
    }

    /// Update the name of the [Key].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{KeyBuilder, Action, Client, KeyUpdater};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY));
    /// let mut new_key = KeyBuilder::new()
    ///     .execute(&client)
    ///     .await
    ///     .unwrap();
    ///
    /// let name = "lovely key".to_string();
    /// let mut key_update = KeyUpdater::new(new_key)
    ///     .with_name(&name)
    ///     .execute(&client)
    ///     .await
    ///     .unwrap();
    ///
    /// assert_eq!(key_update.name, Some(name));
    /// # client.delete_key(key_update).await.unwrap();
    /// # });
    /// ```
    pub fn with_name(&mut self, desc: impl AsRef<str>) -> &mut KeyUpdater {
        self.name = Some(desc.as_ref().to_string());
        self
    }

    /// Update a [Key] using the [`KeyUpdater`].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{KeyBuilder, KeyUpdater, Client};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY));
    /// let description = "My little lovely test key".to_string();
    /// let key = KeyBuilder::new()
    ///     .execute(&client).await.unwrap();
    ///
    /// let mut key_update = KeyUpdater::new(&key.key);
    /// key_update.with_description(&description).execute(&client).await;
    ///
    /// assert_eq!(key_update.description, Some(description));
    /// # client.delete_key(key).await.unwrap();
    /// # });
    /// ```
    pub async fn execute(&self, client: &Client) -> Result<Key, Error> {
        client.update_key(self).await
    }
}

impl AsRef<str> for KeyUpdater {
    fn as_ref(&self) -> &str {
        &self.key
    }
}

impl AsRef<KeyUpdater> for KeyUpdater {
    fn as_ref(&self) -> &KeyUpdater {
        self
    }
}

#[derive(Debug, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct KeysQuery {
    /// The number of documents to skip.
    ///
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
}

impl KeysQuery {
    /// Create a [`KeysQuery`] with only a description.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{key::KeysQuery};
    /// let builder = KeysQuery::new();
    /// ```
    pub fn new() -> KeysQuery {
        Self::default()
    }

    /// Specify the offset.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{KeysQuery, Action, Client};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY));
    /// let mut keys = KeysQuery::new()
    ///     .with_offset(1)
    ///     .execute(&client).await.unwrap();
    ///
    /// assert_eq!(keys.offset, 1);
    /// # });
    /// ```
    pub fn with_offset(&mut self, offset: usize) -> &mut KeysQuery {
        self.offset = Some(offset);
        self
    }

    /// Specify the maximum number of keys to return.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{KeysQuery, Action, Client};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY));
    /// let mut keys = KeysQuery::new()
    ///     .with_limit(1)
    ///     .execute(&client).await.unwrap();
    ///
    /// assert_eq!(keys.results.len(), 1);
    /// # });
    /// ```
    pub fn with_limit(&mut self, limit: usize) -> &mut KeysQuery {
        self.limit = Some(limit);
        self
    }

    /// Get [Key]'s.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{KeysQuery, Action, Client};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY));
    /// let mut keys = KeysQuery::new()
    ///     .with_limit(1)
    ///     .execute(&client).await.unwrap();
    ///
    /// assert_eq!(keys.results.len(), 1);
    /// # });
    /// ```
    pub async fn execute(&self, client: &Client) -> Result<KeysResults, Error> {
        client.get_keys_with(self).await
    }
}

/// The [`KeyBuilder`] is an analog to the [Key] type but without all the fields managed by Meilisearch.
///
/// It's used to create [Key].
///
/// # Example
///
/// ```
/// # use meilisearch_sdk::{KeyBuilder, Action, Client};
/// #
/// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
/// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
/// #
/// # futures::executor::block_on(async move {
/// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY));
/// let description = "My little lovely test key".to_string();
/// let key = KeyBuilder::new()
///     .with_description(&description)
///     .execute(&client).await.unwrap();
///
/// assert_eq!(key.description, Some(description));
/// # client.delete_key(key).await.unwrap();
/// # });
/// ```
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct KeyBuilder {
    pub actions: Vec<Action>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub expires_at: Option<OffsetDateTime>,
    pub indexes: Vec<String>,
}

impl KeyBuilder {
    /// Create a [`KeyBuilder`].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::KeyBuilder;
    /// let builder = KeyBuilder::new();
    /// ```
    pub fn new() -> KeyBuilder {
        Self::default()
    }

    /// Declare a set of actions the [Key] will be able to execute.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{KeyBuilder, Action};
    /// let mut builder = KeyBuilder::new();
    /// builder.with_actions(vec![Action::Search, Action::DocumentsAdd]);
    /// ```
    pub fn with_actions(&mut self, actions: impl IntoIterator<Item = Action>) -> &mut KeyBuilder {
        self.actions.extend(actions);
        self
    }

    /// Add one action the [Key] will be able to execute.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{KeyBuilder, Action};
    /// let mut builder = KeyBuilder::new();
    /// builder.with_action(Action::DocumentsAdd);
    /// ```
    pub fn with_action(&mut self, action: Action) -> &mut KeyBuilder {
        self.actions.push(action);
        self
    }

    /// Set the expiration date of the [Key].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::KeyBuilder;
    /// # use time::{OffsetDateTime, Duration};
    /// let mut builder = KeyBuilder::new();
    /// // create a key that expires in two weeks from now
    /// builder.with_expires_at(OffsetDateTime::now_utc() + Duration::WEEK * 2);
    /// ```
    pub fn with_expires_at(&mut self, expires_at: OffsetDateTime) -> &mut KeyBuilder {
        self.expires_at = Some(expires_at);
        self
    }

    /// Set the indexes the [Key] can manage.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{KeyBuilder, Client};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY));
    /// let mut key = KeyBuilder::new()
    ///     .with_indexes(vec!["test", "movies"])
    ///     .execute(&client)
    ///     .await
    ///     .unwrap();
    ///
    /// assert_eq!(vec!["test", "movies"], key.indexes);
    /// # client.delete_key(key).await.unwrap();
    /// # });
    /// ```
    pub fn with_indexes(
        &mut self,
        indexes: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> &mut KeyBuilder {
        self.indexes = indexes
            .into_iter()
            .map(|index| index.as_ref().to_string())
            .collect();
        self
    }

    /// Add one index the [Key] can manage.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::KeyBuilder;
    /// let mut builder = KeyBuilder::new();
    /// builder.with_index("test");
    /// ```
    pub fn with_index(&mut self, index: impl AsRef<str>) -> &mut KeyBuilder {
        self.indexes.push(index.as_ref().to_string());
        self
    }

    /// Add a description to the [Key].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{KeyBuilder, Action, Client};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY));
    /// let description = "My not so little lovely test key".to_string();
    /// let mut key = KeyBuilder::new()
    ///     .with_description(&description)
    ///     .execute(&client).await.unwrap();
    ///
    /// assert_eq!(key.description, Some(description));
    /// # client.delete_key(key).await.unwrap();
    /// # });
    /// ```
    pub fn with_description(&mut self, desc: impl AsRef<str>) -> &mut KeyBuilder {
        self.description = Some(desc.as_ref().to_string());
        self
    }

    /// Add a name to the [Key].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{KeyBuilder, Action, Client};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY));
    /// let name = "lovely key".to_string();
    /// let mut key = KeyBuilder::new()
    ///     .with_name(&name)
    ///     .execute(&client).await.unwrap();
    ///
    /// assert_eq!(key.name, Some(name));
    /// # client.delete_key(key).await.unwrap();
    /// # });
    /// ```
    pub fn with_name(&mut self, desc: impl AsRef<str>) -> &mut KeyBuilder {
        self.name = Some(desc.as_ref().to_string());
        self
    }

    /// Add an uid to the [Key].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{KeyBuilder, Action, Client};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY));
    /// let uid = "93bcd7fb-2196-4fd9-acb7-3fca8a96e78f".to_string();
    /// let mut key = KeyBuilder::new()
    ///     .with_uid(&uid)
    ///     .execute(&client).await.unwrap();
    ///
    /// assert_eq!(key.uid, uid);
    /// # client.delete_key(key).await.unwrap();
    /// # });
    /// ```
    pub fn with_uid(&mut self, desc: impl AsRef<str>) -> &mut KeyBuilder {
        self.uid = Some(desc.as_ref().to_string());
        self
    }

    /// Create a [Key] from the builder.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{KeyBuilder, Client};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY));
    /// let description = "My little lovely test key".to_string();
    /// let key = KeyBuilder::new()
    ///     .with_description(&description)
    ///     .execute(&client).await.unwrap();
    ///
    /// assert_eq!(key.description, Some(description));
    /// # client.delete_key(key).await.unwrap();
    /// # });
    /// ```
    pub async fn execute(&self, client: &Client) -> Result<Key, Error> {
        client.create_key(self).await
    }
}

impl AsRef<KeyBuilder> for KeyBuilder {
    fn as_ref(&self) -> &KeyBuilder {
        self
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Action {
    /// Provides access to everything.
    #[serde(rename = "*")]
    All,
    /// Provides access to both [`POST`](https://www.meilisearch.com/docs/reference/api/search.md#search-in-an-index-with-post-route) and [`GET`](https://www.meilisearch.com/docs/reference/api/search.md#search-in-an-index-with-get-route) search endpoints on authorized indexes.
    #[serde(rename = "search")]
    Search,
    /// Provides access to the [add documents](https://www.meilisearch.com/docs/reference/api/documents.md#add-or-replace-documents) and [update documents](https://www.meilisearch.com/docs/reference/api/documents.md#add-or-update-documents) endpoints on authorized indexes.
    #[serde(rename = "documents.add")]
    DocumentsAdd,
    /// Provides access to the [get one document](https://www.meilisearch.com/docs/reference/api/documents.md#get-one-document) and [get documents](https://www.meilisearch.com/docs/reference/api/documents.md#get-documents) endpoints on authorized indexes.
    #[serde(rename = "documents.get")]
    DocumentsGet,
    /// Provides access to the [delete one document](https://www.meilisearch.com/docs/reference/api/documents.md#delete-one-document), [delete all documents](https://www.meilisearch.com/docs/reference/api/documents.md#delete-all-documents), and [batch delete](https://www.meilisearch.com/docs/reference/api/documents.md#delete-documents-by-batch) endpoints on authorized indexes.
    #[serde(rename = "documents.delete")]
    DocumentsDelete,
    /// Provides access to the [create index](https://www.meilisearch.com/docs/reference/api/indexes.md#create-an-index) endpoint.
    #[serde(rename = "indexes.create")]
    IndexesCreate,
    /// Provides access to the [get one index](https://www.meilisearch.com/docs/reference/api/indexes.md#get-one-index) and [list all indexes](https://www.meilisearch.com/docs/reference/api/indexes.md#list-all-indexes) endpoints. **Non-authorized `indexes` will be omitted from the response**.
    #[serde(rename = "indexes.get")]
    IndexesGet,
    /// Provides access to the [update index](https://www.meilisearch.com/docs/reference/api/indexes.md#update-an-index) endpoint.
    #[serde(rename = "indexes.update")]
    IndexesUpdate,
    /// Provides access to the [delete index](https://www.meilisearch.com/docs/reference/api/indexes.md#delete-an-index) endpoint.
    #[serde(rename = "indexes.delete")]
    IndexesDelete,
    /// Provides access to the [get one task](https://www.meilisearch.com/docs/reference/api/tasks.md#get-task) and [get all tasks](https://www.meilisearch.com/docs/reference/api/tasks.md#get-all-tasks) endpoints. **Tasks from non-authorized `indexes` will be omitted from the response**. Also provides access to the [get one task by index](https://www.meilisearch.com/docs/reference/api/tasks.md#get-task-by-index) and [get all tasks by index](https://www.meilisearch.com/docs/reference/api/tasks.md#get-all-tasks-by-index) endpoints on authorized indexes.
    #[serde(rename = "tasks.get")]
    TasksGet,
    /// Provides access to the [get settings](https://www.meilisearch.com/docs/reference/api/settings.md#get-settings) endpoint and equivalents for all subroutes on authorized indexes.
    #[serde(rename = "settings.get")]
    SettingsGet,
    /// Provides access to the [update settings](https://www.meilisearch.com/docs/reference/api/settings.md#update-settings) and [reset settings](https://www.meilisearch.com/docs/reference/api/settings.md#reset-settings) endpoints and equivalents for all subroutes on authorized indexes.
    #[serde(rename = "settings.update")]
    SettingsUpdate,
    /// Provides access to the [get stats of an index](https://www.meilisearch.com/docs/reference/api/stats.md#get-stats-of-an-index) endpoint and the [get stats of all indexes](https://www.meilisearch.com/docs/reference/api/stats.md#get-stats-of-all-indexes) endpoint. For the latter, **non-authorized `indexes` are omitted from the response**.
    #[serde(rename = "stats.get")]
    StatsGet,
    /// Provides access to the [create dump](https://www.meilisearch.com/docs/reference/api/dump.md#create-a-dump) endpoint. **Not restricted by `indexes`.**
    #[serde(rename = "dumps.create")]
    DumpsCreate,
    /// Provides access to the [get dump status](https://www.meilisearch.com/docs/reference/api/dump.md#get-dump-status) endpoint. **Not restricted by `indexes`.**
    #[serde(rename = "dumps.get")]
    DumpsGet,
    /// Provides access to the [get Meilisearch version](https://www.meilisearch.com/docs/reference/api/version.md#get-version-of-meilisearch) endpoint.
    #[serde(rename = "version")]
    Version,
    /// Provides access to the [get Key](https://www.meilisearch.com/docs/reference/api/keys#get-one-key) and [get Keys](https://www.meilisearch.com/docs/reference/api/keys#get-all-keys) endpoints.
    #[serde(rename = "keys.get")]
    KeyGet,
    /// Provides access to the [create key](https://www.meilisearch.com/docs/reference/api/keys#create-a-key) endpoint.
    #[serde(rename = "keys.create")]
    KeyCreate,
    /// Provides access to the [update key](https://www.meilisearch.com/docs/reference/api/keys#update-a-key) endpoint.
    #[serde(rename = "keys.update")]
    KeyUpdate,
    /// Provides access to the [delete key](https://www.meilisearch.com/docs/reference/api/keys#delete-a-key) endpoint.
    #[serde(rename = "keys.delete")]
    KeyDelete,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KeysResults {
    pub results: Vec<Key>,
    pub limit: u32,
    pub offset: u32,
}
