use crate::errors::Error;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;

use crate::{indexes::Index, request::HttpClient};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldResult {
    /// The name of the field
    pub name: String,

    /// contains `enabled` key indicating if field is displayed
    pub displayed: HashMap<String, bool>,

    /// contains `enabled` key indicating if this field is searchable
    pub searchable: HashMap<String, bool>,

    /// contains `enabled` key indicating if this field is sortable
    pub sortable: HashMap<String, bool>,

    /// contains `enabled` key indicating if this field is distinct
    pub distinct: HashMap<String, bool>,

    /// contains `enabled` key indicating if this field is used in ranking rules.
    /// If enabled, also contains 'order' with value 'asc' or 'desc'
    pub ranking_rule: Map<String, Value>,

    /// contains `enabled` key indicating if field is filterable,
    /// and the following filter settings:
    /// - sortBy: Sort order for facet values (e.g., 'alpha')
    /// - facetSearch: Whether facet search is enabled
    /// - equality: Whether equality filtering is enabled
    /// - comparison: Whether comparison filtering is enabled
    pub filterable: Map<String, Value>,

    /// Contains 'locales' key with locales array
    /// e.g. `{"locales": ["en", "fr"]}`
    pub localized: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldsResult {
    pub results: Vec<FieldResult>,
    pub offset: u32,
    pub limit: u32,
    pub total: u32,
}

/// An [`FieldsQuery`] containing filter and pagination parameters when looking up an index's fields.
///
/// # Example
///
/// ```
/// # use serde::{Serialize, Deserialize};
/// # use meilisearch_sdk::{client::*, indexes::*, fields::*};
/// #
/// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
/// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
/// #
/// # #[derive(Serialize, Deserialize, Debug)]
/// # struct Movie {
/// #    name: String,
/// #    description: String,
/// # }
///
/// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
/// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
/// # let index = client
/// #   .create_index("fields_query", None)
/// #   .await
/// #   .unwrap()
/// #   .wait_for_completion(&client, None, None)
/// #   .await
/// #   .unwrap()
/// #   // Once the task finished, we try to create an `Index` out of it.
/// #   .try_make_index(&client)
/// #   .unwrap();
/// # index.add_or_replace(&[Movie{name:String::from("Interstellar"), description:String::from("Interstellar chronicles the adventures of a group of explorers who make use of a newly discovered wormhole to surpass the limitations on human space travel and conquer the vast distances involved in an interstellar voyage.")}], Some("name")).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
/// let fields = FieldsQuery::new(&index)
///     .with_offset(1)
///     .execute()
///     .await
///     .unwrap();
/// assert_eq!(fields.results.len(), 1);
/// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
/// # });
/// ```
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldsQuery<'a, Http: HttpClient> {
    #[serde(skip_serializing)]
    pub index: &'a Index<Http>,
    /// The number of fields to skip.
    ///
    /// If the value of the parameter `offset` is `n`, the `n` first fields will not be returned.
    ///
    /// Example: If you want to skip the first field, set offset to `1`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    /// The maximum number of fields returned.
    ///
    /// If the value of the parameter `limit` is `n`, there will never be more than `n` fields in the response.
    ///
    /// Example: If you don't want to get more than two fields, set limit to `2`.
    ///
    /// **Default: `20`**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    /// [Filter](`FieldsQueryFilter`) for fields returned
    ///
    /// All fields return must match **all** of the filter criteria
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<FieldsQueryFilter>,
}

impl<'a, Http: HttpClient> FieldsQuery<'a, Http> {
    #[must_use]
    pub fn new(index: &Index<Http>) -> FieldsQuery<'_, Http> {
        FieldsQuery {
            index,
            offset: None,
            limit: None,
            filter: None,
        }
    }

    /// Specify the number of fields to skip.
    pub fn with_offset(&mut self, offset: usize) -> &mut FieldsQuery<'a, Http> {
        self.offset = Some(offset);
        self
    }

    /// Specify the maximum number of fields to return.
    pub fn with_limit(&mut self, limit: usize) -> &mut FieldsQuery<'a, Http> {
        self.limit = Some(limit);
        self
    }

    /// Specify the [`FieldsQueryFilter`].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, fields::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # let index = client
    /// #   .create_index("fields_query_with_filter", None)
    /// #   .await
    /// #   .unwrap()
    /// #   .wait_for_completion(&client, None, None)
    /// #   .await
    /// #   .unwrap()
    /// #   // Once the task finished, we try to create an `Index` out of it
    /// #   .try_make_index(&client)
    /// #   .unwrap();
    /// let filter = FieldsQueryFilter::new().with_displayed(true);
    /// let mut fields = FieldsQuery::new(&index)
    ///     .with_filter(filter)
    ///     .execute().await.unwrap();
    ///
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub fn with_filter(&mut self, filter: FieldsQueryFilter) -> &mut FieldsQuery<'a, Http> {
        self.filter = Some(filter);

        self
    }

    /// Get an index's fields.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{fields::FieldsQuery, client::Client};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # let index = client
    /// #   .create_index("fields_query_execute", None)
    /// #   .await
    /// #   .unwrap()
    /// #   .wait_for_completion(&client, None, None)
    /// #   .await
    /// #   .unwrap()
    /// #   // Once the task finished, we try to create an `Index` out of it
    /// #   .try_make_index(&client)
    /// #   .unwrap();
    /// let fields = FieldsQuery::new(&index)
    ///     .with_limit(1)
    ///     .execute()
    ///     .await
    ///     .unwrap();
    ///
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn execute(&self) -> Result<FieldsResult, Error> {
        self.index.get_fields_with(self).await
    }
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FieldsQueryFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_patterns: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub displayed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sortable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distinct: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranking_rule: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filterable: Option<bool>,
}

impl FieldsQueryFilter {
    pub fn new() -> Self {
        FieldsQueryFilter::default()
    }

    /// Match fields using attribute patterns (supports wildcards: * for any characters), e.g.
    /// - `"cuisine.*"` matches `cuisine.type`, `cuisine.region`
    /// - `"user*"` matches `user_id`, username, `user_profile`
    /// - `"*_id"` matches all fields ending with `_id`
    /// # Example
    /// ```
    /// # use meilisearch_sdk::fields::*;
    /// let filter = FieldsQueryFilter::new()
    ///     .with_attribute_patterns(vec!["cuisine.*", "*_id"]);
    /// ```
    pub fn with_attribute_patterns(
        mut self,
        attribute_patterns: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self.attribute_patterns = Some(
            attribute_patterns
                .into_iter()
                .map(|v| v.as_ref().to_string())
                .collect(),
        );

        self
    }

    /// Filter by whether fields are displayed in search results
    ///
    ///  `true` = only displayed fields, `false` = only hidden fields
    pub fn with_displayed(mut self, displayed: bool) -> Self {
        self.displayed = Some(displayed);

        self
    }

    /// Filter by whether fields are searchable (indexed for full-text search)
    ///
    /// `true` = only searchable fields, `false` = only non-searchable fields
    pub fn with_searchable(mut self, searchable: bool) -> Self {
        self.searchable = Some(searchable);

        self
    }

    /// Filter by whether fields can be used for sorting results
    ///
    /// `true` = only sortable fields, `false` = only non-sortable fields
    pub fn with_sortable(mut self, sortable: bool) -> Self {
        self.sortable = Some(sortable);

        self
    }

    /// Filter by whether the field is used as the distinct attribute
    ///
    /// `true` = only the distinct field, `false` = only non-distinct fields
    pub fn with_distinct(mut self, distinct: bool) -> Self {
        self.distinct = Some(distinct);

        self
    }

    /// Filter by whether the field is used in ranking rules
    ///
    /// `true` = only fields used in ranking, `false` = only fields not used in ranking
    pub fn with_ranking_rule(mut self, ranking_rule: bool) -> Self {
        self.ranking_rule = Some(ranking_rule);

        self
    }

    /// Filter by whether the field can be used for filtering/faceting
    ///
    /// `true` = only filterable fields, `false` = only non-filterable fields
    pub fn with_filterable(mut self, filterable: bool) -> Self {
        self.filterable = Some(filterable);

        self
    }
}

#[cfg(test)]
mod tests {
    use crate::client::Client;

    use super::*;

    use meilisearch_test_macro::meilisearch_test;
    use serde_json::json;

    #[meilisearch_test]
    async fn test_fields_query(client: Client, index: Index) -> Result<(), Error> {
        let document_with_5_fields = json!({
            "id": 1,
            "name": "doggo",
            "field3": "value",
            "field4": "value",
            "field5": "value"
        });

        index
            .add_documents(&[document_with_5_fields], None)
            .await
            .unwrap()
            .wait_for_completion(&client, None, None)
            .await
            .unwrap();

        let fields_result = index.get_fields().await;

        assert!(fields_result.is_ok_and(|fields| fields.results.len() == 5));

        Ok(())
    }

    #[meilisearch_test]
    async fn test_get_fields_with_filter(client: Client, index: Index) -> Result<(), Error> {
        let document_with_7_fields = json!({
            "id": 1,
            "field1": "value",
            "field2": "value",
            "field3": "value",
            "field4": "value",
            "field5": "value",
            "field6": "value",
        });

        index
            .add_documents(&[document_with_7_fields], None)
            .await
            .unwrap()
            .wait_for_completion(&client, None, None)
            .await
            .unwrap();

        let fields_result = FieldsQuery::new(&index)
            .with_offset(1)
            .with_limit(6)
            .with_filter(
                FieldsQueryFilter::new()
                    .with_attribute_patterns(["field*"])
                    .with_displayed(true)
                    .with_searchable(true)
                    .with_sortable(false)
                    .with_distinct(false)
                    .with_ranking_rule(false)
                    .with_filterable(false),
            )
            .execute()
            .await;

        //skipped 1/6 fields with offset = 1
        assert!(fields_result.is_ok_and(|fields| fields.results.len() == 5));

        Ok(())
    }
}
