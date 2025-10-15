use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{to_vec, Value};

use crate::{
    client::Client,
    errors::Error,
    request::{HttpClient, Method},
};

/// Representation of a chat workspace.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ChatWorkspace {
    pub uid: String,
}

/// Paginated chat workspace results.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatWorkspacesResults {
    pub results: Vec<ChatWorkspace>,
    pub offset: u32,
    pub limit: u32,
    pub total: u32,
}

/// Chat workspace prompts payload.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ChatPrompts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_description: Option<String>,
    #[serde(rename = "searchQParam", skip_serializing_if = "Option::is_none")]
    pub search_q_param: Option<String>,
    #[serde(
        rename = "searchIndexUidParam",
        skip_serializing_if = "Option::is_none"
    )]
    pub search_index_uid_param: Option<String>,
    /// Any additional provider-specific prompt values.
    #[serde(default, flatten, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, String>,
}

impl ChatPrompts {
    /// Creates a new `ChatPrompts` with default (empty) fields.
    ///
    /// # Examples
    ///
    /// ```
    /// let prompts = ChatPrompts::new();
    /// assert!(prompts.system.is_none());
    /// assert!(prompts.extra.is_empty());
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the system prompt text for the chat prompts, returning a mutable reference for chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut prompts = ChatPrompts::new();
    /// prompts.set_system("You are a helpful assistant.");
    /// assert_eq!(prompts.system.as_deref(), Some("You are a helpful assistant."));
    /// ```
    pub fn set_system(&mut self, value: impl Into<String>) -> &mut Self {
        self.system = Some(value.into());
        self
    }

    /// Sets the search description for the prompts.
    ///
    /// Returns a mutable reference to `self` for method chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut prompts = ChatPrompts::new();
    /// prompts.set_search_description("Find relevant docs");
    /// assert_eq!(prompts.search_description.as_deref(), Some("Find relevant docs"));
    /// ```
    pub fn set_search_description(&mut self, value: impl Into<String>) -> &mut Self {
        self.search_description = Some(value.into());
        self
    }

    /// Set the name of the query parameter that will be substituted into prompts.
    ///
    /// The provided `value` becomes the `searchQParam` field in serialized payloads.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut p = crate::chats::ChatPrompts::new();
    /// p.set_search_q_param("q");
    /// assert_eq!(p.search_q_param.as_deref(), Some("q"));
    /// ```
    ///
    /// @param value The parameter name to use for the user's search query.
    /// @returns `&mut Self` to allow method chaining.
    pub fn set_search_q_param(&mut self, value: impl Into<String>) -> &mut Self {
        self.search_q_param = Some(value.into());
        self
    }

    /// Sets the chat prompt's search index UID parameter.
    ///
    /// This value is serialized as `searchIndexUidParam` when the prompts are sent.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut prompts = crate::chats::ChatPrompts::new();
    /// prompts.set_search_index_uid_param("my_index");
    /// assert_eq!(prompts.search_index_uid_param.as_deref(), Some("my_index"));
    /// ```
    pub fn set_search_index_uid_param(&mut self, value: impl Into<String>) -> &mut Self {
        self.search_index_uid_param = Some(value.into());
        self
    }

    /// Inserts a key/value pair into the prompts' extra map and returns a mutable reference for chaining.
    ///
    /// The provided `key` and `value` are converted to `String` and stored in the `extra` map,
    /// replacing any existing value for the same key.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut prompts = ChatPrompts::new();
    /// prompts.insert("tone", "friendly").insert("length", "short");
    /// assert_eq!(prompts.extra.get("tone").map(String::as_str), Some("friendly"));
    /// assert_eq!(prompts.extra.get("length").map(String::as_str), Some("short"));
    /// ```
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.extra.insert(key.into(), value.into());
        self
    }
}

/// Chat workspace settings payload.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ChatWorkspaceSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompts: Option<ChatPrompts>,
}

impl ChatWorkspaceSettings {
    /// Creates a new `ChatPrompts` with default (empty) fields.
    ///
    /// # Examples
    ///
    /// ```
    /// let prompts = ChatPrompts::new();
    /// assert!(prompts.system.is_none());
    /// assert!(prompts.extra.is_empty());
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the source identifier for the chat workspace settings.
    ///
    /// Sets the `source` field to the provided value and returns a mutable reference to enable method chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut s = ChatWorkspaceSettings::new();
    /// s.set_source("remote");
    /// assert_eq!(s.source.unwrap(), "remote");
    /// ```
    pub fn set_source(&mut self, source: impl Into<String>) -> &mut Self {
        self.source = Some(source.into());
        self
    }

    /// Sets the organization identifier for these workspace settings.
    ///
    /// # Parameters
    ///
    /// - `org_id`: The organization identifier to associate with the workspace.
    ///
    /// # Returns
    ///
    /// A mutable reference to the modified `ChatWorkspaceSettings` for chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut settings = ChatWorkspaceSettings::new();
    /// settings.set_org_id("org_123");
    /// assert_eq!(settings.org_id.as_deref(), Some("org_123"));
    /// ```
    pub fn set_org_id(&mut self, org_id: impl Into<String>) -> &mut Self {
        self.org_id = Some(org_id.into());
        self
    }

    /// Sets the workspace project identifier.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut s = ChatWorkspaceSettings::new();
    /// s.set_project_id("my-project");
    /// assert_eq!(s.project_id.as_deref(), Some("my-project"));
    /// ```
    ///
    /// Returns a mutable reference to `self` for chaining.
    pub fn set_project_id(&mut self, project_id: impl Into<String>) -> &mut Self {
        self.project_id = Some(project_id.into());
        self
    }

    /// Set the API version to use for the workspace.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut settings = ChatWorkspaceSettings::new();
    /// settings.set_api_version("2023-01");
    /// assert_eq!(settings.api_version.as_deref(), Some("2023-01"));
    /// ```
    pub fn set_api_version(&mut self, api_version: impl Into<String>) -> &mut Self {
        self.api_version = Some(api_version.into());
        self
    }

    /// Sets the deployment identifier for the workspace settings and returns a mutable reference for chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut s = ChatWorkspaceSettings::new();
    /// s.set_deployment_id("dep-123");
    /// assert_eq!(s.deployment_id.as_deref(), Some("dep-123"));
    /// ```
    pub fn set_deployment_id(&mut self, deployment_id: impl Into<String>) -> &mut Self {
        self.deployment_id = Some(deployment_id.into());
        self
    }

    /// Sets the base URL for the chat workspace settings.
    ///
    /// The provided value is stored in the `base_url` field.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut settings = ChatWorkspaceSettings::new();
    /// settings.set_base_url("https://example.com");
    /// assert_eq!(settings.base_url.as_deref(), Some("https://example.com"));
    /// ```
    pub fn set_base_url(&mut self, base_url: impl Into<String>) -> &mut Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Set the API key for the workspace settings.
    ///
    /// # Returns
    ///
    /// `self` as a mutable reference so calls can be chained.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut s = crate::ChatWorkspaceSettings::new();
    /// s.set_api_key("my-secret-key");
    /// assert_eq!(s.api_key.as_deref(), Some("my-secret-key"));
    /// ```
    pub fn set_api_key(&mut self, api_key: impl Into<String>) -> &mut Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Sets the prompts for the chat workspace settings.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut settings = ChatWorkspaceSettings::new();
    /// let mut prompts = ChatPrompts::new();
    /// prompts.set_system("You are a helpful assistant.");
    /// settings.set_prompts(prompts);
    /// assert_eq!(settings.prompts.unwrap().system.unwrap(), "You are a helpful assistant.");
    /// ```
    ///
    /// @returns `&mut Self` to allow method chaining.
    pub fn set_prompts(&mut self, prompts: impl Into<ChatPrompts>) -> &mut Self {
        self.prompts = Some(prompts.into());
        self
    }
}

/// Query builder for listing chat workspaces.
#[derive(Debug, Serialize)]
pub struct ChatWorkspacesQuery<'a, Http: HttpClient> {
    #[serde(skip_serializing)]
    pub client: &'a Client<Http>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

impl<'a, Http: HttpClient> ChatWorkspacesQuery<'a, Http> {
    /// Create a `ChatWorkspacesQuery` bound to the given client.
    ///
    /// The returned query has neither `offset` nor `limit` set.
    ///
    /// # Examples
    ///
    /// ```
    /// // assuming `client` is a `&Client<_>` available in scope
    /// let query = ChatWorkspacesQuery::new(client);
    /// assert!(query.offset.is_none() && query.limit.is_none());
    /// ```
    #[must_use]
    pub fn new(client: &'a Client<Http>) -> Self {
        Self {
            client,
            offset: None,
            limit: None,
        }
    }

    /// Sets the starting index for the query results.
    ///
    /// # Parameters
    ///
    /// - `offset`: The zero-based index of the first result to return.
    ///
    /// # Returns
    ///
    /// A mutable reference to the query builder for chaining.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let mut q = ChatWorkspacesQuery::new(&client);
    /// q.with_offset(5);
    /// assert_eq!(q.offset, Some(5));
    /// ```
    pub fn with_offset(&mut self, offset: usize) -> &mut Self {
        self.offset = Some(offset);
        self
    }

    /// Sets the maximum number of workspaces to return for the query.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use crate::chats::ChatWorkspacesQuery;
    /// # use crate::client::Client;
    /// # fn example<Http: crate::request::HttpClient>(client: &Client<Http>) {
    /// let mut query = ChatWorkspacesQuery::new(client);
    /// query.with_limit(10);
    /// # }
    /// ```
    pub fn with_limit(&mut self, limit: usize) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    /// Executes the query and fetches chat workspaces matching the query parameters.
    ///
    /// # Returns
    ///
    /// `Ok(ChatWorkspacesResults)` with the matching workspaces and pagination metadata on success, `Err(Error)` on failure.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use crate::{Client, ChatWorkspacesQuery};
    /// # async fn run(client: &Client<_>) -> Result<(), crate::errors::Error> {
    /// let mut query = ChatWorkspacesQuery::new(client);
    /// query.with_limit(1);
    /// let results = query.execute().await?;
    /// assert!(results.results.len() <= 1);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn execute(&self) -> Result<ChatWorkspacesResults, Error> {
        self.client.list_chat_workspaces_with(self).await
    }
}

impl<Http: HttpClient> Client<Http> {
    /// List all chat workspaces.
    ///
    /// Returns a `ChatWorkspacesResults` containing the matching workspaces and pagination metadata.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example(client: &crate::client::Client<_>) -> Result<(), crate::errors::Error> {
    /// let results = client.list_chat_workspaces().await?;
    /// // `results.results` contains the workspaces; `results.limit`, `results.offset`, and `results.total` contain pagination info.
    /// assert!(results.results.len() as u32 <= results.limit);
    /// # Ok(()) }
    /// ```
    pub async fn list_chat_workspaces(&self) -> Result<ChatWorkspacesResults, Error> {
        self.http_client
            .request::<(), (), ChatWorkspacesResults>(
                &format!("{}/chats", self.host),
                Method::Get { query: () },
                200,
            )
            .await
    }

    /// Lists chat workspaces using the provided query builder.
    ///
    /// The `query` can include optional pagination parameters such as `offset` and `limit`.
    ///
    /// # Parameters
    ///
    /// - `query`: Query builder containing optional pagination parameters and a reference to the client.
    ///
    /// # Returns
    ///
    /// `ChatWorkspacesResults` containing the matching workspaces and pagination metadata.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::{Client, chats::ChatWorkspacesQuery};
    /// # async fn _example<Http: crate::request::HttpClient>(client: &Client<Http>) -> Result<(), crate::errors::Error> {
    /// let mut q = ChatWorkspacesQuery::new(client);
    /// q.with_limit(10);
    /// let results = client.list_chat_workspaces_with(&q).await?;
    /// assert!(results.results.len() <= 10);
    /// # Ok(()) }
    /// ```
    pub async fn list_chat_workspaces_with(
        &self,
        query: &ChatWorkspacesQuery<'_, Http>,
    ) -> Result<ChatWorkspacesResults, Error> {
        self.http_client
            .request::<&ChatWorkspacesQuery<'_, Http>, (), ChatWorkspacesResults>(
                &format!("{}/chats", self.host),
                Method::Get { query },
                200,
            )
            .await
    }

    /// Retrieve a chat workspace by its UID.
    ///
    /// Returns the workspace identified by `uid`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = /* construct Client */;
    /// let workspace = client.get_chat_workspace("workspace_uid").await?;
    /// assert_eq!(workspace.uid, "workspace_uid");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_chat_workspace(&self, uid: impl AsRef<str>) -> Result<ChatWorkspace, Error> {
        self.http_client
            .request::<(), (), ChatWorkspace>(
                &format!("{}/chats/{}", self.host, uid.as_ref()),
                Method::Get { query: () },
                200,
            )
            .await
    }

    /// Retrieve the settings for a chat workspace.
    ///
    /// # Parameters
    ///
    /// - `uid`: The chat workspace UID.
    ///
    /// # Returns
    ///
    /// `ChatWorkspaceSettings` for the specified workspace.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn example(client: &crate::client::Client<impl crate::request::HttpClient>) {
    /// let settings = client.get_chat_workspace_settings("my-workspace").await.unwrap();
    /// assert!(settings.source.is_some() || settings.prompts.is_some() || settings.api_key.is_none());
    /// # }
    /// ```
    pub async fn get_chat_workspace_settings(
        &self,
        uid: impl AsRef<str>,
    ) -> Result<ChatWorkspaceSettings, Error> {
        self.http_client
            .request::<(), (), ChatWorkspaceSettings>(
                &format!("{}/chats/{}/settings", self.host, uid.as_ref()),
                Method::Get { query: () },
                200,
            )
            .await
    }

    /// Create or update the settings for a chat workspace identified by `uid`.
    ///
    /// The server-side settings are replaced with the provided `settings` payload and the
    /// resulting saved `ChatWorkspaceSettings` is returned.
    ///
    /// # Returns
    ///
    /// `ChatWorkspaceSettings` containing the stored settings as returned by the server.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run_example() -> Result<(), Box<dyn std::error::Error>> {
    /// use crate::client::Client;
    /// use crate::chats::ChatWorkspaceSettings;
    ///
    /// let client = Client::new("http://localhost:7700", None::<String>);
    /// let mut settings = ChatWorkspaceSettings::new();
    /// settings.set_source("example");
    ///
    /// let saved = client.update_chat_workspace_settings("my-workspace", &settings).await?;
    /// assert_eq!(saved.source.as_deref(), Some("example"));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_chat_workspace_settings(
        &self,
        uid: impl AsRef<str>,
        settings: &ChatWorkspaceSettings,
    ) -> Result<ChatWorkspaceSettings, Error> {
        self.http_client
            .request::<(), &ChatWorkspaceSettings, ChatWorkspaceSettings>(
                &format!("{}/chats/{}/settings", self.host, uid.as_ref()),
                Method::Patch {
                    query: (),
                    body: settings,
                },
                200,
            )
            .await
    }

    /// Resets the settings for the chat workspace identified by `uid` to the server defaults.
    ///
    /// On success, returns the workspace settings object as returned by the server.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn example(client: &crate::client::Client<impl crate::request::HttpClient>) -> Result<(), Box<dyn std::error::Error>> {
    /// let settings = client.reset_chat_workspace_settings("my-workspace").await?;
    /// println!("{:?}", settings);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn reset_chat_workspace_settings(
        &self,
        uid: impl AsRef<str>,
    ) -> Result<ChatWorkspaceSettings, Error> {
        self.http_client
            .request::<(), (), ChatWorkspaceSettings>(
                &format!("{}/chats/{}/settings", self.host, uid.as_ref()),
                Method::Delete { query: () },
                200,
            )
            .await
    }
}

#[cfg(feature = "reqwest")]
impl Client<crate::reqwest::ReqwestClient> {
    /// Stream chat completions for the specified chat workspace.
    ///
    /// On success returns the streaming HTTP response which yields server-sent events for the completion.
    /// If the HTTP status is not successful, the response body is parsed into an `Error` and returned.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use serde_json::json;
    ///
    /// # async fn run_example(client: &crate::Client<crate::reqwest::ReqwestClient>) -> Result<(), crate::Error> {
    /// let body = json!({
    ///     "model": "gpt-4o-mini",
    ///     "messages": [ { "role": "user", "content": "Hello" } ],
    ///     "stream": true
    /// });
    ///
    /// let resp = client.stream_chat_completion("workspace_uid", &body).await?;
    /// assert!(resp.status().is_success());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn stream_chat_completion<S: Serialize + ?Sized>(
        &self,
        uid: impl AsRef<str>,
        body: &S,
    ) -> Result<reqwest::Response, Error> {
        let request = self.build_stream_chat_request(uid.as_ref(), body)?;

        let response = self.http_client.inner().execute(request).await?;

        let status = response.status();
        if !status.is_success() {
            let url = response.url().to_string();
            let mut body = response.text().await?;
            if body.is_empty() {
                body = "null".to_string();
            }
            let err =
                match crate::request::parse_response::<Value>(status.as_u16(), 200, &body, url) {
                    Ok(_) => unreachable!("parse_response succeeded on a non-successful status"),
                    Err(err) => err,
                };
            return Err(err);
        }

        Ok(response)
    }

    /// Constructs an HTTP POST request for streaming chat completions to a chat workspace.
    ///
    /// The request targets "{host}/chats/{uid}/chat/completions", sets `Accept: text/event-stream` and
    /// `Content-Type: application/json`, and attaches the JSON-serialized `body` as the request payload.
    /// If the client has an API key configured, an `Authorization: Bearer {key}` header is added.
    ///
    /// # Arguments
    ///
    /// * `uid` - The chat workspace identifier to which the completion request will be sent.
    /// * `body` - A serializable payload that will be encoded as the JSON request body.
    ///
    /// # Returns
    ///
    /// A `reqwest::Request` ready to be executed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct Body { model: &'static str, stream: bool }
    ///
    /// // `client` represents the surrounding client type that exposes `build_stream_chat_request`.
    /// // let req = client.build_stream_chat_request("workspace-uid", &Body { model: "gpt", stream: true }).unwrap();
    /// // assert_eq!(req.method(), reqwest::Method::POST);
    /// ```
    fn build_stream_chat_request<S: Serialize + ?Sized>(
        &self,
        uid: &str,
        body: &S,
    ) -> Result<reqwest::Request, Error> {
        use reqwest::header::{HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};

        let payload = to_vec(body).map_err(Error::ParseError)?;

        let mut request = self
            .http_client
            .inner()
            .post(format!("{}/chats/{}/chat/completions", self.host, uid))
            .header(ACCEPT, HeaderValue::from_static("text/event-stream"))
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .body(payload)
            .build()?;

        if let Some(key) = self.api_key.as_deref() {
            request.headers_mut().insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {key}")).unwrap(),
            );
        }

        Ok(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use meilisearch_test_macro::meilisearch_test;
    use serde_json::json;
    #[meilisearch_test]
    async fn chat_workspace_lifecycle(client: Client, name: String) -> Result<(), Error> {
        let _: serde_json::Value = client
            .http_client
            .request(
                &format!("{}/experimental-features", client.host),
                Method::Patch {
                    query: (),
                    body: &json!({ "chatCompletions": true }),
                },
                200,
            )
            .await?;

        let workspace = format!("{name}-workspace");

        let mut prompts = ChatPrompts::new();
        prompts.set_system("You are a helpful assistant.");
        prompts.set_search_description("Use search to fetch relevant documents.");

        let mut settings = ChatWorkspaceSettings::new();
        settings
            .set_source("openAi")
            .set_api_key("sk-test")
            .set_prompts(prompts.clone());

        let updated = client
            .update_chat_workspace_settings(&workspace, &settings)
            .await?;
        assert_eq!(updated.source.as_deref(), Some("openAi"));
        let updated_prompts = updated
            .prompts
            .expect("updated settings should contain prompts");
        assert_eq!(updated_prompts.system.as_deref(), prompts.system.as_deref());
        assert_eq!(
            updated_prompts.search_description.as_deref(),
            prompts.search_description.as_deref()
        );
        if let Some(masked_key) = updated.api_key.as_ref() {
            assert_ne!(
                masked_key, "sk-test",
                "API key should not be returned in clear text"
            );
        }

        let workspace_info = client.get_chat_workspace(&workspace).await?;
        assert_eq!(workspace_info.uid, workspace);

        let fetched_settings = client.get_chat_workspace_settings(&workspace).await?;
        assert_eq!(fetched_settings.source.as_deref(), Some("openAi"));
        let fetched_prompts = fetched_settings
            .prompts
            .expect("workspace should have prompts configured");
        assert_eq!(fetched_prompts.system.as_deref(), prompts.system.as_deref());
        assert_eq!(
            fetched_prompts.search_description.as_deref(),
            prompts.search_description.as_deref()
        );

        let list = client.list_chat_workspaces().await?;
        assert!(list.results.iter().any(|w| w.uid == workspace));

        let mut query = ChatWorkspacesQuery::new(&client);
        query.with_limit(1);
        let limited = query.execute().await?;
        assert_eq!(limited.limit, 1);

        let _ = client.reset_chat_workspace_settings(&workspace).await?;

        Ok(())
    }

    #[test]
    fn chat_prompts_builder_helpers() {
        let mut prompts = ChatPrompts::new();
        prompts
            .set_system("system")
            .set_search_description("desc")
            .set_search_q_param("q")
            .set_search_index_uid_param("idx")
            .insert("custom", "value");

        assert_eq!(prompts.system.as_deref(), Some("system"));
        assert_eq!(prompts.search_description.as_deref(), Some("desc"));
        assert_eq!(prompts.search_q_param.as_deref(), Some("q"));
        assert_eq!(prompts.search_index_uid_param.as_deref(), Some("idx"));
        assert_eq!(
            prompts.extra.get("custom").map(String::as_str),
            Some("value")
        );
    }

    #[test]
    fn chat_workspace_settings_builder_helpers() {
        let mut settings = ChatWorkspaceSettings::new();
        settings
            .set_source("openAi")
            .set_org_id("org")
            .set_project_id("project")
            .set_api_version("2024-01-01")
            .set_deployment_id("deployment")
            .set_base_url("http://example.com")
            .set_api_key("secret")
            .set_prompts({
                let mut prompts = ChatPrompts::new();
                prompts.set_system("hi");
                prompts
            });

        assert_eq!(settings.source.as_deref(), Some("openAi"));
        assert_eq!(settings.org_id.as_deref(), Some("org"));
        assert_eq!(settings.project_id.as_deref(), Some("project"));
        assert_eq!(settings.api_version.as_deref(), Some("2024-01-01"));
        assert_eq!(settings.deployment_id.as_deref(), Some("deployment"));
        assert_eq!(settings.base_url.as_deref(), Some("http://example.com"));
        assert_eq!(settings.api_key.as_deref(), Some("secret"));
        assert_eq!(
            settings.prompts.and_then(|p| p.system).as_deref(),
            Some("hi")
        );
    }

    #[test]
    #[cfg(feature = "reqwest")]
    fn stream_chat_completion_request_includes_expected_headers() {
        use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

        let client = Client::new("http://localhost:7700", Some("secret")).unwrap();
        let body = json!({
            "model": "gpt-3.5-turbo",
            "messages": [{ "role": "user", "content": "Hello" }],
            "stream": true
        });

        let request = client
            .build_stream_chat_request("workspace", &body)
            .expect("request should be built");

        assert_eq!(request.method(), reqwest::Method::POST);
        assert_eq!(
            request.url().as_str(),
            "http://localhost:7700/chats/workspace/chat/completions"
        );

        let headers = request.headers();
        assert_eq!(
            headers
                .get(reqwest::header::ACCEPT)
                .map(|h| h.to_str().unwrap()),
            Some("text/event-stream")
        );
        assert_eq!(
            headers.get(CONTENT_TYPE).map(|h| h.to_str().unwrap()),
            Some("application/json")
        );
        assert_eq!(
            headers.get(AUTHORIZATION).map(|h| h.to_str().unwrap()),
            Some("Bearer secret")
        );

        let expected_body = body.to_string();
        let request_body = request
            .body()
            .and_then(|b| b.as_bytes())
            .expect("request has body");
        assert_eq!(request_body, expected_body.as_bytes());
    }
}