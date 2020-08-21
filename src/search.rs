use crate::{errors::Error, indexes::Index};
use serde::{de::DeserializeOwned, Deserialize};
use std::collections::HashMap;
use serde_json::to_string;

#[derive(Deserialize, Debug)]
pub struct MatchRange {
    start: usize,
    length: usize
}

/// A single result.  
/// Contains the complete object, optionally the formatted object, and optionnaly an object that contains information about the matches.
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
    /// results of the query
    pub hits: Vec<SearchResult<T>>,
    /// number of documents skipped
    pub offset: usize,
    /// number of documents to take
    pub limit: usize,
    /// total number of matches
    pub nb_hits: usize,
    /// whether nbHits is exhaustive
    pub exhaustive_nb_hits: bool,
    /// Distribution of the given facets.
    pub facets_distribution: Option<HashMap<String, HashMap<String, usize>>>,
    /// Whether facet_distribution is exhaustive
    pub exhaustive_facets_count: Option<bool>,
    /// processing time of the query
    pub processing_time_ms: usize,
    /// query originating the response
    pub query: String,
}

type AttributeToCrop<'a> = (&'a str, Option<usize>);

/// A struct representing a query.
/// You can add search parameters using the builder syntax.
/// See [here](https://docs.meilisearch.com/guides/advanced_guides/search_parameters.html#query-q) for the list and description of all parameters.
///
/// # Example
///
/// ```
/// # use meilisearch_sdk::search::Query;
/// let query = Query::new("space")
///     .with_offset(42)
///     .with_limit(21);
/// ```
pub struct Query<'a> {
    /// The query parameter is the only mandatory parameter.
    /// This is the string used by the search engine to find relevant documents.
    pub query: &'a str,
    /// A number of documents to skip. If the value of the parameter offset is n, n first documents to skip. This is helpful for pagination.
    ///
    /// Example: If you want to skip the first document, set offset to 1.
    /// Default: 0
    pub offset: Option<usize>,
    /// Set a limit to the number of documents returned by search queries. If the value of the parameter limit is n, there will be n documents in the search query response. This is helpful for pagination.
    ///
    /// Example: If you want to get only two documents, set limit to 2.
    /// Default: 20
    pub limit: Option<usize>,
    /// Specify a filter to be used with the query. See the [dedicated guide](https://docs.meilisearch.com/guides/advanced_guides/filtering.html).
    pub filters: Option<&'a str>,
    /// Facet names and values to filter on. See [this page](https://docs.meilisearch.com/guides/advanced_guides/search_parameters.html#facet-filters).
    pub facet_filters: Option<&'a [&'a [&'a str]]>,
    /// Facets for which to retrieve the matching count. The value `Some(None)` is the wildcard.
    pub facets_distribution: Option<Option<&'a [&'a str]>>,
    /// Attributes to **display** in the returned documents.
    pub attributes_to_retrieve: Option<&'a [&'a str]>,
    /// Attributes to crop. The value `Some(None)` is the wildcard. Attributes are composed by the attribute name and an optional `usize` that overwrites the `crop_length` parameter.
    pub attributes_to_crop: Option<Option<&'a [AttributeToCrop<'a>]>>,
    /// Number of characters to keep on each side of the start of the matching word. See [attributes_to_crop](#structfield.attributes_to_crop).
    ///
    /// Default: 200
    pub crop_length: Option<usize>,
    /// Attributes whose values will contain **highlighted matching query words**. The value `Some(None)` is the wildcard.
    pub attributes_to_highlight: Option<Option<&'a [&'a str]>>,
    /// Defines whether an object that contains information about the matches should be returned or not
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
    pub fn with_offset(self, offset: usize) -> Query<'a> {
        Query {
            offset: Some(offset),
            ..self
        }
    }
    pub fn with_limit(self, limit: usize) -> Query<'a> {
        Query {
            limit: Some(limit),
            ..self
        }
    }
    pub fn with_filters(self, filters: &'a str) -> Query<'a> {
        Query {
            filters: Some(filters),
            ..self
        }
    }
    pub fn with_facet_filters(self, facet_filters: &'a[&'a[&'a str]]) -> Query<'a> {
        Query {
            facet_filters: Some(facet_filters),
            ..self
        }
    }
    pub fn with_facets_distribution(self, facets_distribution: Option<&'a[&'a str]>) -> Query<'a> {
        Query {
            facets_distribution: Some(facets_distribution),
            ..self
        }
    }
    pub fn with_attributes_to_retrieve(self, attributes_to_retrieve: &'a [&'a str]) -> Query<'a> {
        Query {
            attributes_to_retrieve: Some(attributes_to_retrieve),
            ..self
        }
    }
    pub fn with_attributes_to_crop(self, attributes_to_crop: Option<&'a [(&'a str, Option<usize>)]>) -> Query<'a> {
        Query {
            attributes_to_crop: Some(attributes_to_crop),
            ..self
        }
    }
    pub fn with_attributes_to_highlight(self, attributes_to_highlight: Option<&'a [&'a str]>) -> Query<'a> {
        Query {
            attributes_to_highlight: Some(attributes_to_highlight),
            ..self
        }
    }
    pub fn with_crop_length(self, crop_length: usize) -> Query<'a> {
        Query {
            crop_length: Some(crop_length),
            ..self
        }
    }
    pub fn with_matches(self, matches: bool) -> Query<'a> {
        Query {
            matches: Some(matches),
            ..self
        }
    }
}

impl<'a> Query<'a> {
    pub(crate) fn to_url(&self) -> String {
        use urlencoding::encode;
        let mut url = format!("?q={}", encode(self.query));

        if let Some(offset) = self.offset {
            url.push_str("&offset=");
            url.push_str(offset.to_string().as_str());
        }
        if let Some(limit) = self.limit {
            url.push_str("&limit=");
            url.push_str(limit.to_string().as_str());
        }
        if let Some(filters) = self.filters {
            url.push_str("&filters=");
            url.push_str(encode(filters).as_str());
        }
        if let Some(matches) = self.matches {
            url.push_str("&matches=");
            url.push_str(matches.to_string().as_str());
        }
        if let Some(facet_filters) = &self.facet_filters {
            url.push_str("&facetFilters=");
            url.push_str(encode(&to_string(&facet_filters).unwrap()).as_str());
        }
        if let Some(facets_distribution) = &self.facets_distribution {
            url.push_str("&facetsDistribution=");
            match facets_distribution {
                Some(facets_distribution) => url.push_str(encode(&to_string(&facets_distribution).unwrap()).as_str()),
                None => url.push_str("*")
            }
        }
        if let Some(attributes_to_retrieve) = self.attributes_to_retrieve {
            url.push_str("&attributesToRetrieve=");
            let mut first = true;
            for attribute_to_retrieve in attributes_to_retrieve {
                if first {
                    first = false;
                } else {
                    url.push(',');
                }
                url.push_str(encode(attribute_to_retrieve).as_str());
            }
        }
        match self.attributes_to_crop {
            Some(None) => url.push_str("&attributesToCrop=*"),
            Some(Some(attributes_to_crop)) => {
                url.push_str("&attributesToCrop=");
                let mut first = true;
                for (attribute_to_crop, crop_length) in attributes_to_crop {
                    if first {
                        first = false;
                    } else {
                        url.push(',');
                    }
                    url.push_str(encode(attribute_to_crop).as_str());
                    if let Some(crop_length) = crop_length {
                        url.push(':');
                        url.push_str(crop_length.to_string().as_str());
                    }
                }
            }
            None => (),
        }
        if let Some(crop_length) = self.crop_length {
            url.push_str("&cropLength=");
            url.push_str(crop_length.to_string().as_str());
        }
        match self.attributes_to_highlight {
            Some(None) => url.push_str("&attributesToHighlight=*"),
            Some(Some(attributes_to_highlight)) => {
                url.push_str("&attributesToHighlight=");
                let mut first = true;
                for attribute_to_highlight in attributes_to_highlight {
                    if first {
                        first = false;
                    } else {
                        url.push(',');
                    }
                    url.push_str(encode(attribute_to_highlight).as_str());
                }
            }
            None => ()
        }

        url
    }

    /// Alias for [the Index method](../indexes/struct.Index.html#method.search).
    pub async fn execute<T: 'static + DeserializeOwned>(
        &'a self,
        index: &Index<'a>,
    ) -> Result<SearchResults<T>, Error> {
        index.search::<T>(&self).await
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialize_query_to_url() {
        let query = Query::new("test")
            .with_attributes_to_crop(Some(&[("first", None)]));
        assert_eq!(query.to_url(), "?q=test&attributesToCrop=first");

        let query = Query::new("test")
            .with_attributes_to_crop(Some(&[("first", None), ("second", None)]));
        assert_eq!(query.to_url(), "?q=test&attributesToCrop=first,second");

        let query = Query::new("test")
            .with_attributes_to_crop(Some(&[("first", Some(5)), ("second", Some(8))]));
        assert_eq!(query.to_url(), "?q=test&attributesToCrop=first:5,second:8");

        let query = Query::new("test")
            .with_attributes_to_crop(Some(&[("firs🤔t", Some(5)), ("second", Some(8))]));
        assert_eq!(query.to_url(), "?q=test&attributesToCrop=firs%F0%9F%A4%94t:5,second:8");

        let query = Query::new("test")
            .with_attributes_to_retrieve(&["first"]);
        assert_eq!(query.to_url(), "?q=test&attributesToRetrieve=first");

        let query = Query::new("test")
            .with_attributes_to_retrieve(&["first", "second"]);
        assert_eq!(query.to_url(), "?q=test&attributesToRetrieve=first,second");

        let query = Query::new("test")
            .with_attributes_to_highlight(Some(&["first"]));
        assert_eq!(query.to_url(), "?q=test&attributesToHighlight=first");

        let query = Query::new("test")
            .with_attributes_to_highlight(Some(&["first", "second"]));
        assert_eq!(query.to_url(), "?q=test&attributesToHighlight=first,second");

        let query = Query::new("test")
            .with_attributes_to_highlight(None);
        assert_eq!(query.to_url(), "?q=test&attributesToHighlight=*");
    }
}