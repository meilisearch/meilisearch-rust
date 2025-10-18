use serde::Deserialize;
use serde::{ser::SerializeMap, Serialize, Serializer};
use serde_json::Value;
use std::collections::BTreeMap;
use uuid::Uuid;

/// Representation of a webhook configuration in Meilisearch.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Webhook {
    pub url: String,
    #[serde(default)]
    pub headers: BTreeMap<String, String>,
}

/// Metadata returned for each webhook by the Meilisearch API.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WebhookInfo {
    pub uuid: Uuid,
    pub is_editable: bool,
    #[serde(flatten)]
    pub webhook: Webhook,
}

/// Results wrapper returned by the `GET /webhooks` endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WebhookList {
    pub results: Vec<WebhookInfo>,
}

/// Payload used to create a new webhook.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WebhookCreate {
    pub url: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub headers: BTreeMap<String, String>,
}

impl WebhookCreate {
    /// Creates a new webhook payload with the given target URL.
    #[must_use]
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            headers: BTreeMap::new(),
        }
    }

    /// Adds or replaces an HTTP header that will be sent with the webhook request.
    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    /// Adds or replaces an HTTP header in-place.
    pub fn insert_header(
        &mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> &mut Self {
        self.headers.insert(name.into(), value.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum HeadersUpdate {
    NotSet,
    Reset,
    Set(BTreeMap<String, Option<String>>),
}

impl Default for HeadersUpdate {
    fn default() -> Self {
        Self::NotSet
    }
}

/// Payload used to update or delete settings of an existing webhook.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WebhookUpdate {
    url: Option<String>,
    headers: HeadersUpdate,
}

impl WebhookUpdate {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Updates the webhook target URL.
    pub fn with_url(&mut self, url: impl Into<String>) -> &mut Self {
        self.url = Some(url.into());
        self
    }

    /// Adds or replaces an HTTP header to be sent with the webhook request.
    pub fn set_header(&mut self, name: impl Into<String>, value: impl Into<String>) -> &mut Self {
        match &mut self.headers {
            HeadersUpdate::Set(map) => {
                map.insert(name.into(), Some(value.into()));
            }
            _ => {
                let mut map = BTreeMap::new();
                map.insert(name.into(), Some(value.into()));
                self.headers = HeadersUpdate::Set(map);
            }
        }
        self
    }

    /// Removes a specific HTTP header from the webhook configuration.
    pub fn remove_header(&mut self, name: impl Into<String>) -> &mut Self {
        match &mut self.headers {
            HeadersUpdate::Set(map) => {
                map.insert(name.into(), None);
            }
            _ => {
                let mut map = BTreeMap::new();
                map.insert(name.into(), None);
                self.headers = HeadersUpdate::Set(map);
            }
        }
        self
    }

    /// Clears all HTTP headers associated with this webhook.
    pub fn reset_headers(&mut self) -> &mut Self {
        self.headers = HeadersUpdate::Reset;
        self
    }
}

impl Serialize for WebhookUpdate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut field_count = 0;
        if self.url.is_some() {
            field_count += 1;
        }
        if !matches!(self.headers, HeadersUpdate::NotSet) {
            field_count += 1;
        }

        let mut map = serializer.serialize_map(Some(field_count))?;
        if let Some(url) = &self.url {
            map.serialize_entry("url", url)?;
        }
        match &self.headers {
            HeadersUpdate::NotSet => {}
            HeadersUpdate::Reset => {
                map.serialize_entry("headers", &Value::Null)?;
            }
            HeadersUpdate::Set(values) => {
                map.serialize_entry("headers", values)?;
            }
        }
        map.end()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::client::Client;
    use crate::errors::Error;
    use meilisearch_test_macro::meilisearch_test;

    #[test]
    fn serialize_update_variants() {
        let mut update = WebhookUpdate::new();
        update.set_header("authorization", "token");
        update.remove_header("referer");

        let json = serde_json::to_value(&update).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "headers": {
                    "authorization": "token",
                    "referer": null
                }
            })
        );

        let mut reset = WebhookUpdate::new();
        reset.reset_headers();
        let json = serde_json::to_value(&reset).unwrap();
        assert_eq!(json, serde_json::json!({ "headers": null }));
    }

    #[meilisearch_test]
    async fn webhook_crud(client: Client) -> Result<(), Error> {
        let initial = client.get_webhooks().await?.results.len();

        let unique_url = format!("https://example.com/webhooks/{}", Uuid::new_v4());

        let mut create = WebhookCreate::new(unique_url.clone());
        create
            .insert_header("authorization", "SECURITY_KEY")
            .insert_header("referer", "https://example.com");

        let created = client.create_webhook(&create).await?;
        assert_eq!(created.webhook.url, unique_url);
        assert!(created.is_editable);
        assert_eq!(created.webhook.headers.len(), 2);

        let fetched = client.get_webhook(&created.uuid.to_string()).await?;
        assert_eq!(fetched.uuid, created.uuid);

        let mut update = WebhookUpdate::new();
        update.remove_header("referer");
        update.set_header("x-extra", "value");

        let updated = client
            .update_webhook(&created.uuid.to_string(), &update)
            .await?;
        assert!(!updated.webhook.headers.contains_key("referer"));
        assert_eq!(
            updated.webhook.headers.get("x-extra"),
            Some(&"value".to_string())
        );

        client.delete_webhook(&created.uuid.to_string()).await?;

        let remaining = client.get_webhooks().await?;
        assert!(
            remaining.results.len() == initial
                || !remaining.results.iter().any(|w| w.uuid == created.uuid)
        );

        Ok(())
    }
}
