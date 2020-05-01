use serde::{Deserialize, de::DeserializeOwned};
use crate::{indexes::Index, errors::Error};

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct SearchResults<T> {
    pub hits: Vec<T>,
    pub offset: usize,
    pub limit: usize,
    pub nbHits: usize,
    pub exhaustiveNbHits: bool,
    pub processingTimeMs: usize,
    pub query: String,
}

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
    pub query: &'a str,
    pub offset: Option<usize>,
    pub limit: Option<usize>,
    pub attributes_to_retrieve: Option<&'a str>,
    pub attributes_to_crop: Option<&'a str>,
    pub crop_lenght: Option<usize>,
    pub attributes_to_highlight: Option<&'a str>,
    pub filters: Option<&'a str>,
}

impl<'a> Query<'a> {
    pub fn new(query: &'a str) -> Query<'a> {
        Query {
            query,
            offset: None,
            limit: None,
            attributes_to_retrieve: None,
            attributes_to_crop: None,
            attributes_to_highlight: None,
            crop_lenght: None,
            filters: None,
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
    pub fn with_attributes_to_retrieve(self, attributes_to_retrieve: &'a str) -> Query<'a> {
        Query {
            attributes_to_retrieve: Some(attributes_to_retrieve),
            ..self
        }
    }
    pub fn with_attributes_to_crop(self, attributes_to_crop: &'a str) -> Query<'a> {
        Query {
            attributes_to_crop: Some(attributes_to_crop),
            ..self
        }
    }
    pub fn with_attributes_to_highlight(self, attributes_to_highlight: &'a str) -> Query<'a> {
        Query {
            attributes_to_highlight: Some(attributes_to_highlight),
            ..self
        }
    }
    pub fn with_crop_lenght(self, crop_lenght: usize) -> Query<'a> {
        Query {
            crop_lenght: Some(crop_lenght),
            ..self
        }
    }
    pub fn with_filters(self, filters: &'a str) -> Query<'a> {
        Query {
            filters: Some(filters),
            ..self
        }
    }
}

impl<'a> Query<'a> {
    pub(crate) fn to_url(&self) -> String {
        use urlencoding::encode;
        let mut url = format!("?q={}&", encode(self.query));

        if let Some(offset) = self.offset {
            url.push_str("offset=");
            url.push_str(offset.to_string().as_str());
            url.push('&');
        }
        if let Some(limit) = self.limit {
            url.push_str("limit=");
            url.push_str(limit.to_string().as_str());
            url.push('&');
        }
        if let Some(attributes_to_retrieve) = self.attributes_to_retrieve {
            url.push_str("attributesToRetrieve=");
            url.push_str(encode(attributes_to_retrieve).as_str());
            url.push('&');
        }
        if let Some(attributes_to_crop) = self.attributes_to_crop {
            url.push_str("attributesToCrop=");
            url.push_str(encode(attributes_to_crop).as_str());
            url.push('&');
        }
        if let Some(crop_lenght) = self.crop_lenght {
            url.push_str("cropLength=");
            url.push_str(crop_lenght.to_string().as_str());
            url.push('&');
        }
        if let Some(attributes_to_highlight) = self.attributes_to_highlight {
            url.push_str("attributesToHighlight=");
            url.push_str(encode(attributes_to_highlight).as_str());
            url.push('&');
        }
        if let Some(filters) = self.filters {
            url.push_str("filters=");
            url.push_str(encode(filters).as_str());
        }

        url
    }

    /// Alias for [the Index method](../indexes/struct.Index.html#method.search).
    pub fn execute<T: DeserializeOwned>(&self, index: &Index) -> Result<SearchResults<T>, Error> {
        index.search::<T>(&self)
    }
}
