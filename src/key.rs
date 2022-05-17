use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{client::Client, errors::Error};

/// Represent a [meilisearch key](https://docs.meilisearch.com/reference/api/keys.html#returned-fields)
/// You can get a [Key] from the [Client::get_key] method.
/// Or you can create a [Key] with the [KeyBuilder::create] or [Client::create_key] methods.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Key {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<Action>,
    #[serde(skip_serializing, with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    pub description: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub expires_at: Option<OffsetDateTime>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub indexes: Vec<String>,
    #[serde(skip_serializing)]
    pub key: String,
    #[serde(skip_serializing, with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl Key {
    /// Update the description of the key.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{key::KeyBuilder, key::Action, client::Client};
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new("http://localhost:7700", "masterKey");
    ///
    ///  let mut key = KeyBuilder::new("My little lovely test key")
    ///   .with_action(Action::DocumentsAdd)
    ///   .with_index("*")
    ///   .create(&client).await.unwrap();
    ///
    /// key.with_description("My not so little lovely test key");
    /// assert_eq!(key.description, "My not so little lovely test key".to_string());
    /// # client.delete_key(key).await.unwrap();
    /// # });
    /// ```
    pub fn with_description(&mut self, desc: impl AsRef<str>) -> &mut Self {
        self.description = desc.as_ref().to_string();
        self
    }

    /// Add a set of actions the [Key] will be able to execute.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{key::KeyBuilder, key::Action, client::Client};
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new("http://localhost:7700", "masterKey");
    ///
    ///  let mut key = KeyBuilder::new("My little lovely test key")
    ///   .with_action(Action::DocumentsAdd)
    ///   .with_index("*")
    ///   .create(&client).await.unwrap();
    ///
    /// key.with_actions([Action::DocumentsGet, Action::DocumentsDelete]);
    /// # client.delete_key(key).await.unwrap();
    /// # });
    /// ```
    pub fn with_actions(&mut self, actions: impl IntoIterator<Item = Action>) -> &mut Self {
        self.actions.extend(actions);
        self
    }

    /// Add one action the [Key] will be able to execute.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{key::KeyBuilder, key::Action, client::Client};
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new("http://localhost:7700", "masterKey");
    ///
    ///  let mut key = KeyBuilder::new("My little lovely test key")
    ///   .with_action(Action::DocumentsAdd)
    ///   .with_index("*")
    ///   .create(&client).await.unwrap();
    ///
    /// key.with_action(Action::DocumentsGet);
    /// # client.delete_key(key).await.unwrap();
    /// # });
    /// ```
    pub fn with_action(&mut self, action: Action) -> &mut Self {
        self.actions.push(action);
        self
    }

    /// Update the expiration date of the [Key].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{key::KeyBuilder, key::Action, client::Client};
    /// use time::{OffsetDateTime, Duration};
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new("http://localhost:7700", "masterKey");
    ///
    ///  let mut key = KeyBuilder::new("My little lovely test key")
    ///   .with_action(Action::DocumentsAdd)
    ///   .with_index("*")
    ///   .create(&client).await.unwrap();
    ///
    /// // update the epiry date of the key to two weeks from now
    /// key.with_expires_at(OffsetDateTime::now_utc() + Duration::WEEK * 2);
    /// # client.delete_key(key).await.unwrap();
    /// # });
    /// ```
    pub fn with_expires_at(&mut self, expires_at: OffsetDateTime) -> &mut Self {
        self.expires_at = Some(expires_at);
        self
    }

    /// Update the indexes the [Key] can manage.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{key::KeyBuilder, key::Action, client::Client};
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new("http://localhost:7700", "masterKey");
    ///
    ///  let mut key = KeyBuilder::new("My little lovely test key")
    ///   .with_action(Action::DocumentsAdd)
    ///   .with_index("*")
    ///   .create(&client).await.unwrap();
    ///
    /// key.with_indexes(vec!["test", "movies"]);
    /// # client.delete_key(key).await.unwrap();
    /// # });
    /// ```
    pub fn with_indexes(
        &mut self,
        indexes: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> &mut Self {
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
    /// # use meilisearch_sdk::{key::KeyBuilder, key::Action, client::Client};
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new("http://localhost:7700", "masterKey");
    ///
    ///  let mut key = KeyBuilder::new("My little lovely test key")
    ///   .with_action(Action::DocumentsAdd)
    ///   .with_index("*")
    ///   .create(&client).await.unwrap();
    ///
    /// key.with_index("test");
    /// # client.delete_key(key).await.unwrap();
    /// # });
    /// ```
    pub fn with_index(&mut self, index: impl AsRef<str>) -> &mut Self {
        self.indexes.push(index.as_ref().to_string());
        self
    }

    /// Update the [Key].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{key::KeyBuilder, client::Client};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut key = KeyBuilder::new("My little lovely test key")
    ///   .create(&client).await.unwrap();
    ///
    /// # assert_eq!(key.description, "My little lovely test key");
    ///
    /// key.with_description("My not so little lovely test key");
    /// let key = key.update(&client).await.unwrap();
    ///
    /// # assert_eq!(key.description, "My not so little lovely test key".to_string());
    ///
    /// # client.delete_key(key).await.unwrap();
    /// # });
    /// ```
    pub async fn update(&self, client: &Client) -> Result<Key, Error> {
        client.update_key(self).await
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

/// The [KeyBuilder] is an analog to the [Key] type but without all the fields managed by Meilisearch.
/// It's used to create [Key].
///
/// # Example
///
/// ```
/// # use meilisearch_sdk::{key::KeyBuilder, key::Action, client::Client};
/// # futures::executor::block_on(async move {
/// let client = Client::new("http://localhost:7700", "masterKey");
///
/// let key = KeyBuilder::new("My little lovely test key")
///   .with_action(Action::DocumentsAdd)
///   .with_index("*")
///   .create(&client).await.unwrap();
///
/// assert_eq!(key.description, "My little lovely test key");
/// # client.delete_key(key).await.unwrap();
/// # });
/// ```
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyBuilder {
    pub actions: Vec<Action>,
    pub description: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub expires_at: Option<OffsetDateTime>,
    pub indexes: Vec<String>,
}

impl KeyBuilder {
    /// Create a [KeyBuilder] with only a description.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{key::KeyBuilder};
    /// let builder = KeyBuilder::new("My little lovely test key");
    /// ```
    pub fn new(description: impl AsRef<str>) -> KeyBuilder {
        Self {
            actions: Vec::new(),
            description: description.as_ref().to_string(),
            expires_at: None,
            indexes: Vec::new(),
        }
    }

    /// Declare a set of actions the [Key] will be able to execute.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::key::{KeyBuilder, Action};
    /// let mut builder = KeyBuilder::new("My little lovely test key");
    /// builder.with_actions(vec![Action::Search, Action::DocumentsAdd]);
    /// ```
    pub fn with_actions(&mut self, actions: impl IntoIterator<Item = Action>) -> &mut Self {
        self.actions.extend(actions);
        self
    }

    /// Add one action the [Key] will be able to execute.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::key::{KeyBuilder, Action};
    /// let mut builder = KeyBuilder::new("My little lovely test key");
    /// builder.with_action(Action::DocumentsAdd);
    /// ```
    pub fn with_action(&mut self, action: Action) -> &mut Self {
        self.actions.push(action);
        self
    }

    /// Set the expiration date of the [Key].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{key::KeyBuilder};
    /// use time::{OffsetDateTime, Duration};
    /// let mut builder = KeyBuilder::new("My little lovely test key");
    /// // create a key that expires in two weeks from now
    /// builder.with_expires_at(OffsetDateTime::now_utc() + Duration::WEEK * 2);
    /// ```
    pub fn with_expires_at(&mut self, expires_at: OffsetDateTime) -> &mut Self {
        self.expires_at = Some(expires_at);
        self
    }

    /// Set the indexes the [Key] can manage.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{key::KeyBuilder};
    /// let mut builder = KeyBuilder::new("My little lovely test key");
    /// builder.with_indexes(vec!["test", "movies"]);
    /// ```
    pub fn with_indexes(
        &mut self,
        indexes: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> &mut Self {
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
    /// # use meilisearch_sdk::{key::KeyBuilder};
    /// let mut builder = KeyBuilder::new("My little lovely test key");
    /// builder.with_index("test");
    /// ```
    pub fn with_index(&mut self, index: impl AsRef<str>) -> &mut Self {
        self.indexes.push(index.as_ref().to_string());
        self
    }

    /// Create a [Key] from the builder.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{key::KeyBuilder, client::Client};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let key = KeyBuilder::new("My little lovely test key")
    ///   .create(&client).await.unwrap();
    ///
    /// assert_eq!(key.description, "My little lovely test key");
    /// # client.delete_key(key).await.unwrap();
    /// # });
    /// ```
    pub async fn create(&self, client: &Client) -> Result<Key, Error> {
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
    /// Provides access to both [`POST`](https://docs.meilisearch.com/reference/api/search.md#search-in-an-index-with-post-route) and [`GET`](https://docs.meilisearch.com/reference/api/search.md#search-in-an-index-with-get-route) search endpoints on authorized indexes.
    #[serde(rename = "search")]
    Search,
    /// Provides access to the [add documents](https://docs.meilisearch.com/reference/api/documents.md#add-or-replace-documents) and [update documents](https://docs.meilisearch.com/reference/api/documents.md#add-or-update-documents) endpoints on authorized indexes.
    #[serde(rename = "documents.add")]
    DocumentsAdd,
    /// Provides access to the [get one document](https://docs.meilisearch.com/reference/api/documents.md#get-one-document) and [get documents](https://docs.meilisearch.com/reference/api/documents.md#get-documents) endpoints on authorized indexes.
    #[serde(rename = "documents.get")]
    DocumentsGet,
    /// Provides access to the [delete one document](https://docs.meilisearch.com/reference/api/documents.md#delete-one-document), [delete all documents](https://docs.meilisearch.com/reference/api/documents.md#delete-all-documents), and [batch delete](https://docs.meilisearch.com/reference/api/documents.md#delete-documents-by-batch) endpoints on authorized indexes.
    #[serde(rename = "documents.delete")]
    DocumentsDelete,
    /// Provides access to the [create index](https://docs.meilisearch.com/reference/api/indexes.md#create-an-index) endpoint.
    #[serde(rename = "indexes.create")]
    IndexesCreate,
    /// Provides access to the [get one index](https://docs.meilisearch.com/reference/api/indexes.md#get-one-index) and [list all indexes](https://docs.meilisearch.com/reference/api/indexes.md#list-all-indexes) endpoints. **Non-authorized `indexes` will be omitted from the response**.
    #[serde(rename = "indexes.get")]
    IndexesGet,
    /// Provides access to the [update index](https://docs.meilisearch.com/reference/api/indexes.md#update-an-index) endpoint.
    #[serde(rename = "indexes.update")]
    IndexesUpdate,
    /// Provides access to the [delete index](https://docs.meilisearch.com/reference/api/indexes.md#delete-an-index) endpoint.
    #[serde(rename = "indexes.delete")]
    IndexesDelete,
    /// Provides access to the [get one task](https://docs.meilisearch.com/reference/api/tasks.md#get-task) and [get all tasks](https://docs.meilisearch.com/reference/api/tasks.md#get-all-tasks) endpoints. **Tasks from non-authorized `indexes` will be omitted from the response**. Also provides access to the [get one task by index](https://docs.meilisearch.com/reference/api/tasks.md#get-task-by-index) and [get all tasks by index](https://docs.meilisearch.com/reference/api/tasks.md#get-all-tasks-by-index) endpoints on authorized indexes.
    #[serde(rename = "tasks.get")]
    TasksGet,
    /// Provides access to the [get settings](https://docs.meilisearch.com/reference/api/settings.md#get-settings) endpoint and equivalents for all subroutes on authorized indexes.
    #[serde(rename = "settings.get")]
    SettingsGet,
    /// Provides access to the [update settings](https://docs.meilisearch.com/reference/api/settings.md#update-settings) and [reset settings](https://docs.meilisearch.com/reference/api/settings.md#reset-settings) endpoints and equivalents for all subroutes on authorized indexes.
    #[serde(rename = "settings.update")]
    SettingsUpdate,
    /// Provides access to the [get stats of an index](https://docs.meilisearch.com/reference/api/stats.md#get-stats-of-an-index) endpoint and the [get stats of all indexes](https://docs.meilisearch.com/reference/api/stats.md#get-stats-of-all-indexes) endpoint. For the latter, **non-authorized `indexes` are omitted from the response**.
    #[serde(rename = "stats.get")]
    StatsGet,
    /// Provides access to the [create dump](https://docs.meilisearch.com/reference/api/dump.md#create-a-dump) endpoint. **Not restricted by `indexes`.**
    #[serde(rename = "dumps.create")]
    DumpsCreate,
    /// Provides access to the [get dump status](https://docs.meilisearch.com/reference/api/dump.md#get-dump-status) endpoint. **Not restricted by `indexes`.**
    #[serde(rename = "dumps.get")]
    DumpsGet,
    /// Provides access to the [get Meilisearch version](https://docs.meilisearch.com/reference/api/version.md#get-version-of-meilisearch) endpoint.
    #[serde(rename = "version")]
    Version,
}
