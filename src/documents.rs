// TODO: remove unused
use crate::{
    client::Client, errors::Error, indexes::Index, request::*, search::*, task_info::TaskInfo,
    tasks::*,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display, time::Duration};
use time::OffsetDateTime;

#[derive(Debug, Clone, Deserialize)]
pub struct DocumentsResults<T> {
    pub results: Vec<T>,
    pub limit: u32,
    pub offset: u32,
    pub total: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct DocumentsQuery<'a> {
    #[serde(skip_serializing)]
    pub index: &'a Index,

    /// The number of documents to skip.
    /// If the value of the parameter `offset` is `n`, the `n` first documents will not be returned.
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

    pub fields: Option<Vec<&'a str>>,
}

impl<'a> DocumentsQuery<'a> {
    pub fn new(index: &Index) -> DocumentsQuery {
        DocumentsQuery {
            index,
            offset: None,
            limit: None,
            fields: None,
        }
    }

    /// Specify the offset.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, documents::*};
    /// #
    /// # let MEILISEARCH_HOST = option_env!("MEILISEARCH_HOST").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # let client = Client::new(MEILISEARCH_HOST, MEILISEARCH_API_KEY);
    ///
    /// let index = client.index("documents_query_offset");
    ///
    /// let mut documents_query = DocumentsQuery::new(&index);
    ///
    /// documents_query.with_offset(1);
    /// ```
    pub fn with_offset(&mut self, offset: usize) -> &mut DocumentsQuery<'a> {
        self.offset = Some(offset);
        self
    }

    /// Specify the limit.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, documents::*};
    /// #
    /// # let MEILISEARCH_HOST = option_env!("MEILISEARCH_HOST").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # let client = Client::new(MEILISEARCH_HOST, MEILISEARCH_API_KEY);
    ///
    /// let index = client.index("documents_query_offset");
    ///
    /// let mut documents_query = DocumentsQuery::new(&index);
    ///
    /// documents_query.with_offset(1);
    /// ```
    pub fn with_limit(&mut self, limit: usize) -> &mut DocumentsQuery<'a> {
        self.limit = Some(limit);
        self
    }

    // TODO: add doc
    pub fn with_fields(
        &mut self,
        fields: impl IntoIterator<Item = &'a str>,
    ) -> &mut DocumentsQuery<'a> {
        self.fields = Some(fields.into_iter().collect());
        self
    }

    // TODO: add doc
    pub async fn execute<T: DeserializeOwned + 'static>(
        &self,
    ) -> Result<DocumentsResults<T>, Error> {
        self.index.get_documents_with::<T>(self).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{client::*, indexes::*};
    use meilisearch_test_macro::meilisearch_test;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct MyObject {
        id: usize,
        kind: String,
    }

    async fn setup_test_index(client: &Client, index: &Index) -> Result<(), Error> {
        let t0 = index
            .add_documents(
                &[
                    MyObject {
                        id: 0,
                        kind: "text".into(),
                    },
                    MyObject {
                        id: 1,
                        kind: "text".into(),
                    },
                    MyObject {
                        id: 2,
                        kind: "title".into(),
                    },
                    MyObject {
                        id: 3,
                        kind: "title".into(),
                    },
                ],
                None,
            )
            .await?;

        t0.wait_for_completion(client, None, None).await?;

        Ok(())
    }

    #[meilisearch_test]
    async fn test_get_documents_with_execute(_client: Client, index: Index) -> Result<(), Error> {
        // let documents = index.get_documents(None, None, None).await.unwrap();
        let documents = DocumentsQuery::new(&index)
            .with_limit(1)
            .with_offset(1)
            .with_fields(["kind"])
            .execute::<MyObject>()
            .await
            .unwrap();

        dbg!(&documents);
        assert_eq!(documents.limit, 1);
        assert_eq!(documents.offset, 1);

        Ok(())
    }
}