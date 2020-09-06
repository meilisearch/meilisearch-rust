use crate::{errors::Error, indexes::Index};
use serde::{de::DeserializeOwned, Deserialize, Serialize, Serializer};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct MatchRange {
    start: usize,
    length: usize
}

/// A single result.  
/// Contains the complete object, optionally the formatted object, and optionaly an object that contains information about the matches.
#[derive(Deserialize, Debug)]
pub struct SearchResult<T> {
    /// The full result.
    #[serde(flatten)]
    pub result: T,
    /// The formatted result.
    #[serde(rename = "_formatted")]
    pub formatted_result: Option<T>,
    /// The object that contains information about the matches.
    #[serde(rename = "_matchesInfo")]
    pub matches_info: Option<HashMap<String, Vec<MatchRange>>>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A struct containing search results and other information about the search.
pub struct SearchResults<T> {
    /// Results of the query
    pub hits: Vec<SearchResult<T>>,
    /// Number of documents skipped
    pub offset: usize,
    /// Number of results returned
    pub limit: usize,
    /// Total number of matches
    pub nb_hits: usize,
    /// Whether nb_hits is exhaustive
    pub exhaustive_nb_hits: bool,
    /// Distribution of the given facets
    pub facets_distribution: Option<HashMap<String, HashMap<String, usize>>>,
    /// Whether facet_distribution is exhaustive
    pub exhaustive_facets_count: Option<bool>,
    /// Processing time of the query
    pub processing_time_ms: usize,
    /// Query originating the response
    pub query: String,
}

fn serialize_with_wildcard<S, T>(data: &Option<Selectors<T>>, s: S) -> Result<S::Ok, S::Error> where S: Serializer, T: Serialize {
    match data {
        Some(Selectors::All) => s.serialize_str("*"),
        Some(Selectors::Some(data)) => data.serialize(s),
        None => s.serialize_none(),
    }
}

/// Some list fields in a `Query` can be set to a wildcard value.
/// This structure allows you to choose between the wildcard value and an exhaustive list of selectors.
#[derive(Debug, Clone)]
pub enum Selectors<T> {
    /// A list of selectors
    Some(T),
    /// The wildcard
    All,
}

type AttributeToCrop<'a> = (&'a str, Option<usize>);

/// A struct representing a query.
/// You can add search parameters using the builder syntax.
/// See [this page](https://docs.meilisearch.com/guides/advanced_guides/search_parameters.html#query-q) for the official list and description of all parameters.
///
/// # Example
///
/// ```
/// # use meilisearch_sdk::search::Query;
/// let query = Query::new("space")
///     .with_offset(42)
///     .with_limit(21)
///     .build();
/// ```
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")] 
pub struct Query<'a> {
    /// The text that will be searched for among the documents.  
    /// This is the only mandatory parameter.  
    #[serde(rename = "q")]
    pub query: &'a str,
    /// The number of documents to skip.  
    /// If the value of the parameter `offset` is `n`, the `n` first documents (ordered by relevance) will not be returned.  
    /// This is helpful for pagination.  
    ///   
    /// Example: If you want to skip the first document, set offset to `1`.  
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub offset: Option<usize>,
    /// The maximum number of documents returned.  
    /// If the value of the parameter `limit` is `n`, there will never be more than `n` documents in the response.  
    /// This is helpful for pagination.  
    ///   
    /// Example: If you don't want to get more than two documents, set limit to `2`.  
    /// Default: `20`  
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub limit: Option<usize>,
    /// Filters applied to documents.  
    /// Read the [dedicated guide](https://docs.meilisearch.com/guides/advanced_guides/filtering.html) to learn the syntax.
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub filters: Option<&'a str>,
    /// Facet names and values to filter on.  
    /// Read [this page](https://docs.meilisearch.com/guides/advanced_guides/search_parameters.html#facet-filters) for a complete explanation.
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub facet_filters: Option<&'a [&'a [&'a str]]>,
    /// Facets for which to retrieve the matching count.  
    ///   
    /// Can be set to a [wildcard value](enum.Selectors.html#variant.All) that will select all existing attributes.  
    /// Default: all attributes found in the documents.
    #[serde(skip_serializing_if = "Option::is_none")] 
    #[serde(serialize_with = "serialize_with_wildcard")]
    pub facets_distribution: Option<Selectors<&'a [&'a str]>>,
    /// Attributes to display in the returned documents.  
    ///   
    /// Default: all attributes found in the documents.
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub attributes_to_retrieve: Option<&'a [&'a str]>,
    /// Attributes whose values have to be cropped.  
    /// Attributes are composed by the attribute name and an optional `usize` that overwrites the `crop_length` parameter.  
    ///   
    /// Can be set to a [wildcard value](enum.Selectors.html#variant.All) that will select all existing attributes.
    #[serde(skip_serializing_if = "Option::is_none")] 
    #[serde(serialize_with = "serialize_with_wildcard")]
    pub attributes_to_crop: Option<Selectors<&'a [AttributeToCrop<'a>]>>,
    /// Number of characters to keep on each side of the start of the matching word.  
    /// See [attributes_to_crop](#structfield.attributes_to_crop).  
    ///   
    /// Default: `200`
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub crop_length: Option<usize>,
    /// Attributes whose values will contain **highlighted matching terms**.  
    ///   
    /// Can be set to a [wildcard value](enum.Selectors.html#variant.All) that will select all existing attributes.
    #[serde(skip_serializing_if = "Option::is_none")] 
    #[serde(serialize_with = "serialize_with_wildcard")]
    pub attributes_to_highlight: Option<Selectors<&'a [&'a str]>>,
    /// Defines whether an object that contains information about the matches should be returned or not.
    ///   
    /// Default: `false`
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub matches: Option<bool>
}

#[allow(missing_docs)]
impl<'a> Query<'a> {
    pub fn new(query: &'a str) -> Query<'a> {
        Query {
            query,
            offset: None,
            limit: None,
            filters: None,
            facet_filters: None,
            facets_distribution: None,
            attributes_to_retrieve: None,
            attributes_to_crop: None,
            attributes_to_highlight: None,
            crop_length: None,
            matches: None,
        }
    }
    pub fn with_offset<'b>(&'b mut self, offset: usize) -> &'b mut Query<'a> {
        self.offset = Some(offset);
        self
    }
    pub fn with_limit<'b>(&'b mut self, limit: usize) -> &'b mut Query<'a> {
        self.limit = Some(limit);
        self
    }
    pub fn with_filters<'b>(&'b mut self, filters: &'a str) -> &'b mut Query<'a> {
        self.filters = Some(filters);
        self
    }
    pub fn with_facet_filters<'b>(&'b mut self, facet_filters: &'a[&'a[&'a str]]) -> &'b mut Query<'a> {
        self.facet_filters = Some(facet_filters);
        self
    }
    pub fn with_facets_distribution<'b>(&'b mut self, facets_distribution: Selectors<&'a[&'a str]>) -> &'b mut Query<'a> {
        self.facets_distribution = Some(facets_distribution);
        self
    }
    pub fn with_attributes_to_retrieve<'b>(&'b mut self, attributes_to_retrieve: &'a [&'a str]) -> &'b mut Query<'a> {
        self.attributes_to_retrieve = Some(attributes_to_retrieve);
        self
    }
    pub fn with_attributes_to_crop<'b>(&'b mut self, attributes_to_crop: Selectors<&'a [(&'a str, Option<usize>)]>) -> &'b mut Query<'a> {
        self.attributes_to_crop = Some(attributes_to_crop);
        self
    }
    pub fn with_attributes_to_highlight<'b>(&'b mut self, attributes_to_highlight: Selectors<&'a [&'a str]>) -> &'b mut Query<'a> {
        self.attributes_to_highlight = Some(attributes_to_highlight);
        self
    }
    pub fn with_crop_length<'b>(&'b mut self, crop_length: usize) -> &'b mut Query<'a> {
        self.crop_length = Some(crop_length);
        self
    }
    pub fn with_matches<'b>(&'b mut self, matches: bool) -> &'b mut Query<'a> {
        self.matches = Some(matches);
        self
    }
    pub fn build(&mut self) -> Query<'a> {
        self.clone()
    }
}

impl<'a> Query<'a> {
    /// Alias for [the Index method](../indexes/struct.Index.html#method.search).
    pub async fn execute<T: 'static + DeserializeOwned>(
        &'a self,
        index: &Index<'a>,
    ) -> Result<SearchResults<T>, Error> {
        index.search::<T>(&self).await
    }
}
