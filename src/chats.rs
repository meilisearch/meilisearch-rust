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
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_system(&mut self, value: impl Into<String>) -> &mut Self {
        self.system = Some(value.into());
        self
    }

    pub fn set_search_description(&mut self, value: impl Into<String>) -> &mut Self {
        self.search_description = Some(value.into());
        self
    }

    pub fn set_search_q_param(&mut self, value: impl Into<String>) -> &mut Self {
        self.search_q_param = Some(value.into());
        self
    }

    pub fn set_search_index_uid_param(&mut self, value: impl Into<String>) -> &mut Self {
        self.search_index_uid_param = Some(value.into());
        self
    }

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
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_source(&mut self, source: impl Into<String>) -> &mut Self {
        self.source = Some(source.into());
        self
    }

    pub fn set_org_id(&mut self, org_id: impl Into<String>) -> &mut Self {
        self.org_id = Some(org_id.into());
        self
    }

    pub fn set_project_id(&mut self, project_id: impl Into<String>) -> &mut Self {
        self.project_id = Some(project_id.into());
        self
    }

    pub fn set_api_version(&mut self, api_version: impl Into<String>) -> &mut Self {
        self.api_version = Some(api_version.into());
        self
    }

    pub fn set_deployment_id(&mut self, deployment_id: impl Into<String>) -> &mut Self {
        self.deployment_id = Some(deployment_id.into());
        self
    }

    pub fn set_base_url(&mut self, base_url: impl Into<String>) -> &mut Self {
        self.base_url = Some(base_url.into());
        self
    }

    pub fn set_api_key(&mut self, api_key: impl Into<String>) -> &mut Self {
        self.api_key = Some(api_key.into());
        self
    }

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
    #[must_use]
    pub fn new(client: &'a Client<Http>) -> Self {
        Self {
            client,
            offset: None,
            limit: None,
        }
    }

    pub fn with_offset(&mut self, offset: usize) -> &mut Self {
        self.offset = Some(offset);
        self
    }

    pub fn with_limit(&mut self, limit: usize) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    pub async fn execute(&self) -> Result<ChatWorkspacesResults, Error> {
        self.client.list_chat_workspaces_with(self).await
    }
}

impl<Http: HttpClient> Client<Http> {
    /// List all chat workspaces.
    pub async fn list_chat_workspaces(&self) -> Result<ChatWorkspacesResults, Error> {
        self.http_client
            .request::<(), (), ChatWorkspacesResults>(
                &format!("{}/chats", self.host),
                Method::Get { query: () },
                200,
            )
            .await
    }

    /// List chat workspaces using query parameters.
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

    /// Retrieve a chat workspace by uid.
    pub async fn get_chat_workspace(&self, uid: impl AsRef<str>) -> Result<ChatWorkspace, Error> {
        self.http_client
            .request::<(), (), ChatWorkspace>(
                &format!("{}/chats/{}", self.host, uid.as_ref()),
                Method::Get { query: () },
                200,
            )
            .await
    }

    /// Retrieve chat workspace settings.
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

    /// Create or update chat workspace settings.
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

    /// Reset chat workspace settings to defaults.
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
    /// Stream chat completions for a workspace.
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
