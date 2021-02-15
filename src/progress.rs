#![allow(missing_docs)]

use crate::{errors::Error, indexes::Index, request::*};
use serde::Deserialize;
use serde_json::{from_value, Value};
use std::collections::{BTreeMap, BTreeSet, HashSet};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProgressJson {
    pub(crate) update_id: usize,
}

impl ProgressJson {
    pub(crate) fn into_progress<'a>(self, index: &'a Index) -> Progress<'a> {
        Progress {
            id: self.update_id,
            index,
        }
    }
}

/// A struct used to track the progress of some async operations.
pub struct Progress<'a> {
    id: usize,
    index: &'a Index<'a>,
}

impl<'a> Progress<'a> {
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movies_index = client.get_or_create("movies").await.unwrap();
    /// let progress = movies_index.delete_all_documents().await.unwrap();
    /// let status = progress.get_status().await.unwrap();
    /// # });
    /// ```
    pub async fn get_status(&self) -> Result<Status, Error> {
        let value = request::<(), Value>(
            &format!(
                "{}/indexes/{}/updates/{}",
                self.index.client.host, self.index.uid, self.id
            ),
            self.index.client.apikey,
            Method::Get,
            200,
        )
        .await?;

        if let Ok(status) = from_value::<ProcessedStatus>(value.clone()) {
            Ok(Status::Processed(status))
        } else {
            let result = from_value::<EnqueuedStatus>(value);
            match result {
                Ok(status) => Ok(Status::Enqueued(status)),
                Err(e) => Err(Error::ParseError(e)),
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum RankingRule {
    Typo,
    Words,
    Proximity,
    Attribute,
    WordsPosition,
    Exactness,
    Asc(String),
    Dsc(String),
}

#[derive(Debug, Clone, Deserialize)]
pub enum UpdateState<T> {
    Update(T),
    Clear,
    Nothing,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsUpdate {
    pub ranking_rules: UpdateState<Vec<RankingRule>>,
    pub distinct_attribute: UpdateState<String>,
    pub identifier: UpdateState<String>,
    pub searchable_attributes: UpdateState<Vec<String>>,
    pub displayed_attributes: UpdateState<HashSet<String>>,
    pub stop_words: UpdateState<BTreeSet<String>>,
    pub synonyms: UpdateState<BTreeMap<String, Vec<String>>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "name")]
#[allow(clippy::large_enum_variant)] // would be great to correct but it's not my code it's from meilisearch/Meilisearch
pub enum UpdateType {
    ClearAll,
    Customs,
    DocumentsAddition { number: usize },
    DocumentsPartial { number: usize },
    DocumentsDeletion { number: usize },
    Settings { settings: SettingsUpdate },
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProcessedStatus {
    pub update_id: u64,
    #[serde(rename = "type")]
    pub update_type: UpdateType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,        // in seconds
    pub enqueued_at: String,  // TODO deserialize to datatime
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at: Option<String>, // TODO deserialize to datatime
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnqueuedStatus {
    pub update_id: u64,
    #[serde(rename = "type")]
    pub update_type: UpdateType,
    pub enqueued_at: String, // TODO deserialize to datatime
}

#[derive(Debug)]
pub enum Status {
    Processed(ProcessedStatus),
    Enqueued(EnqueuedStatus),
}
