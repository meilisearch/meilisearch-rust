use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteConfig {
    pub url: String,
    #[serde(rename = "searchApiKey")]
    pub search_api_key: String,
    #[serde(rename = "writeApiKey", skip_serializing_if = "Option::is_none")]
    // present in responses since 1.19
    pub write_api_key: Option<String>,
}

pub type RemotesMap = HashMap<String, RemoteConfig>;
pub type RemotesUpdateMap = HashMap<String, Option<RemoteConfig>>;

/// Full network state returned by GET /network
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkState {
    pub remotes: Option<RemotesMap>,
    #[serde(rename = "self")]
    pub self_name: Option<String>,
    pub leader: Option<String>,
    pub version: Option<Uuid>,
}

/// Partial update body for PATCH /network
#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remotes: Option<RemotesUpdateMap>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Uuid>,
}
