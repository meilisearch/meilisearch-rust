use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Struct reprensenting a set of settings.  
/// You can build this struct using the builder syntax.  
///
/// # Example
///
/// ```
/// # use meilisearch_sdk::settings::Settings;
/// let stop_words = vec![String::from("a"), String::from("the"), String::from("of")];
///
/// let settings = Settings::new()
///     .with_stop_words(stop_words.clone())
///     .with_accept_new_fields(false);
///
/// // OR
///
/// let mut settings = Settings::new();
/// settings.stop_words = Some(stop_words.clone());
/// settings.accept_new_fields = Some(false);
///
/// // OR
///
/// let settings = Settings {
///     stop_words: Some(stop_words.clone()),
///     accept_new_fields: Some(false),
///     ..Settings::new()
/// };
/// ```
#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    /// List of associated words treated similarly
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synonyms: Option<HashMap<String, Vec<String>>>,
    /// List of words ignored by MeiliSearch when present in search queries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_words: Option<Vec<String>>,
    /// List of [ranking rules](https://docs.meilisearch.com/guides/main_concepts/relevancy.html#order-of-the-rules) sorted by order of importance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranking_rules: Option<Vec<String>>,
    /// Search returns documents with distinct (different) values of the given field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distinct_attribute: Option<String>,
    /// Fields in which to search for matching query words sorted by order of importance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchable_attributes: Option<Vec<String>>,
    /// Fields displayed in the returned documents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub displayed_attributes: Option<Vec<String>>,
    /// Defines whether new fields should be searchable and displayed or not
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_new_fields: Option<bool>,
}

#[allow(missing_docs)]
impl Settings {
    /// Create undefined settings
    pub fn new() -> Settings {
        Settings {
            synonyms: None,
            stop_words: None,
            ranking_rules: None,
            distinct_attribute: None,
            searchable_attributes: None,
            displayed_attributes: None,
            accept_new_fields: None,
        }
    }
    pub fn with_synonyms(self, synonyms: HashMap<String, Vec<String>>) -> Settings {
        Settings {
            synonyms: Some(synonyms),
            ..self
        }
    }
    pub fn with_stop_words(self, stop_words: Vec<String>) -> Settings {
        Settings {
            stop_words: Some(stop_words),
            ..self
        }
    }
    pub fn with_ranking_rules(self, ranking_rules: Vec<String>) -> Settings {
        Settings {
            ranking_rules: Some(ranking_rules),
            ..self
        }
    }
    pub fn with_distinct_attribute(self, distinct_attribute: String) -> Settings {
        Settings {
            distinct_attribute: Some(distinct_attribute),
            ..self
        }
    }
    pub fn with_searchable_attributes(self, searchable_attributes: Vec<String>) -> Settings {
        Settings {
            searchable_attributes: Some(searchable_attributes),
            ..self
        }
    }
    pub fn with_displayed_attributes(self, displayed_attributes: Vec<String>) -> Settings {
        Settings {
            displayed_attributes: Some(displayed_attributes),
            ..self
        }
    }
    pub fn with_accept_new_fields(self, accept_new_fields: bool) -> Settings {
        Settings {
            accept_new_fields: Some(accept_new_fields),
            ..self
        }
    }
}
