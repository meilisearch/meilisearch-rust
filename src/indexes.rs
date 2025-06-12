use crate::{
    client::Client,
    documents::{DocumentDeletionQuery, DocumentQuery, DocumentsQuery, DocumentsResults},
    errors::{Error, MeilisearchCommunicationError, MeilisearchError, MEILISEARCH_VERSION_HINT},
    request::*,
    search::*,
    similar::*,
    task_info::TaskInfo,
    tasks::*,
    DefaultHttpClient,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display, time::Duration};
use time::OffsetDateTime;

/// A Meilisearch [index](https://www.meilisearch.com/docs/learn/core_concepts/indexes).
///
/// # Example
///
/// You can create an index remotely and, if that succeed, make an `Index` out of it.
/// See the [`Client::create_index`] method.
/// ```
/// # use meilisearch_sdk::{client::*, indexes::*};
/// #
/// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
/// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
/// #
/// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
/// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
///
/// // get the index called movies or create it if it does not exist
/// let movies = client
///     .create_index("index", None)
///     .await
///     .unwrap()
///     // We wait for the task to execute until completion
///     .wait_for_completion(&client, None, None)
///     .await
///     .unwrap()
///     // Once the task finished, we try to create an `Index` out of it
///     .try_make_index(&client)
///     .unwrap();
///
/// assert_eq!(movies.as_ref(), "index");
/// # movies.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
/// # });
/// ```
///
/// Or, if you know the index already exist remotely you can create an [Index] with its builder.
/// ```
/// # use meilisearch_sdk::{client::*, indexes::*};
/// #
/// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
/// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
/// #
/// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
/// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
///
/// // Meilisearch would be able to create the index if it does not exist during:
/// // - the documents addition (add and update routes)
/// // - the settings update
/// let movies = Index::new("movies", client);
///
/// assert_eq!(movies.uid, "movies");
/// # });
/// ```
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Index<Http: HttpClient = DefaultHttpClient> {
    #[serde(skip_serializing)]
    pub client: Client<Http>,
    pub uid: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub created_at: Option<OffsetDateTime>,
    pub primary_key: Option<String>,
}

impl<Http: HttpClient> Index<Http> {
    pub fn new(uid: impl Into<String>, client: Client<Http>) -> Index<Http> {
        Index {
            uid: uid.into(),
            client,
            primary_key: None,
            created_at: None,
            updated_at: None,
        }
    }
    /// Internal Function to create an [Index] from `serde_json::Value` and [Client].
    pub(crate) fn from_value(
        raw_index: serde_json::Value,
        client: Client<Http>,
    ) -> Result<Index<Http>, Error> {
        #[derive(Deserialize, Debug)]
        #[allow(non_snake_case)]
        struct IndexFromSerde {
            uid: String,
            #[serde(with = "time::serde::rfc3339::option")]
            updatedAt: Option<OffsetDateTime>,
            #[serde(with = "time::serde::rfc3339::option")]
            createdAt: Option<OffsetDateTime>,
            primaryKey: Option<String>,
        }

        let i: IndexFromSerde = serde_json::from_value(raw_index).map_err(Error::ParseError)?;

        Ok(Index {
            uid: i.uid,
            client,
            created_at: i.createdAt,
            updated_at: i.updatedAt,
            primary_key: i.primaryKey,
        })
    }

    /// Update an [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, task_info::*, tasks::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # let mut index = client
    /// #   .create_index("index_update", None)
    /// #   .await
    /// #   .unwrap()
    /// #   .wait_for_completion(&client, None, None)
    /// #   .await
    /// #   .unwrap()
    /// # // Once the task finished, we try to create an `Index` out of it
    /// #   .try_make_index(&client)
    /// #   .unwrap();
    /// #
    /// index.primary_key = Some("special_id".to_string());
    /// let task = index.update()
    ///     .await
    ///     .unwrap()
    ///     .wait_for_completion(&client, None, None)
    ///     .await
    ///     .unwrap();
    ///
    /// let index = client.get_index("index_update").await.unwrap();
    ///
    /// assert_eq!(index.primary_key, Some("special_id".to_string()));
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn update(&self) -> Result<TaskInfo, Error> {
        let mut index_update = IndexUpdater::new(self, &self.client);

        if let Some(ref primary_key) = self.primary_key {
            index_update.with_primary_key(primary_key);
        }

        index_update.execute().await
    }

    /// Delete the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # let index = client.create_index("delete", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap().try_make_index(&client).unwrap();
    ///
    /// // get the index named "movies" and delete it
    /// let index = client.index("delete");
    /// let task = index.delete().await.unwrap();
    ///
    /// client.wait_for_task(task, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn delete(self) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), (), TaskInfo>(
                &format!("{}/indexes/{}", self.client.host, self.uid),
                Method::Delete { query: () },
                202,
            )
            .await
    }

    /// Search for documents matching a specific query in the index.
    ///
    /// See also [`Index::search`].
    ///
    /// # Example
    ///
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::{client::*, indexes::*, search::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct Movie {
    ///     name: String,
    ///     description: String,
    /// }
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// let movies = client.index("execute_query");
    ///
    /// // add some documents
    /// # movies.add_or_replace(&[Movie{name:String::from("Interstellar"), description:String::from("Interstellar chronicles the adventures of a group of explorers who make use of a newly discovered wormhole to surpass the limitations on human space travel and conquer the vast distances involved in an interstellar voyage.")},Movie{name:String::from("Unknown"), description:String::from("Unknown")}], Some("name")).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    ///
    /// let query = SearchQuery::new(&movies).with_query("Interstellar").with_limit(5).build();
    /// let results = movies.execute_query::<Movie>(&query).await.unwrap();
    ///
    /// assert!(results.hits.len() > 0);
    /// # movies.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn execute_query<T: 'static + DeserializeOwned + Send + Sync>(
        &self,
        body: &SearchQuery<'_, Http>,
    ) -> Result<SearchResults<T>, Error> {
        self.client
            .http_client
            .request::<(), &SearchQuery<Http>, SearchResults<T>>(
                &format!("{}/indexes/{}/search", self.client.host, self.uid),
                Method::Post { body, query: () },
                200,
            )
            .await
    }

    /// Search for documents matching a specific query in the index.
    ///
    /// See also [`Index::execute_query`].
    ///
    /// # Example
    ///
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::{client::*, indexes::*, search::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct Movie {
    ///     name: String,
    ///     description: String,
    /// }
    ///
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// let mut movies = client.index("search");
    /// # // add some documents
    /// # movies.add_or_replace(&[Movie{name:String::from("Interstellar"), description:String::from("Interstellar chronicles the adventures of a group of explorers who make use of a newly discovered wormhole to surpass the limitations on human space travel and conquer the vast distances involved in an interstellar voyage.")},Movie{name:String::from("Unknown"), description:String::from("Unknown")}], Some("name")).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    ///
    /// let results = movies.search()
    ///     .with_query("Interstellar")
    ///     .with_limit(5)
    ///     .execute::<Movie>()
    ///     .await
    ///     .unwrap();
    ///
    /// assert!(results.hits.len() > 0);
    /// # movies.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    #[must_use]
    pub fn search(&self) -> SearchQuery<Http> {
        SearchQuery::new(self)
    }

    /// Get one document using its unique id.
    ///
    /// Serde is needed. Add `serde = {version="1.0", features=["derive"]}` in the dependencies section of your Cargo.toml.
    ///
    /// # Example
    ///
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// #[derive(Serialize, Deserialize, Debug, PartialEq)]
    /// struct Movie {
    ///     name: String,
    ///     description: String
    /// }
    ///
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// let movies = client.index("get_document");
    /// # movies.add_or_replace(&[Movie{name:String::from("Interstellar"), description:String::from("Interstellar chronicles the adventures of a group of explorers who make use of a newly discovered wormhole to surpass the limitations on human space travel and conquer the vast distances involved in an interstellar voyage.")}], Some("name")).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    ///
    /// // retrieve a document (you have to put the document in the index before)
    /// let interstellar = movies.get_document::<Movie>("Interstellar").await.unwrap();
    ///
    /// assert_eq!(interstellar, Movie {
    ///     name: String::from("Interstellar"),
    ///     description: String::from("Interstellar chronicles the adventures of a group of explorers who make use of a newly discovered wormhole to surpass the limitations on human space travel and conquer the vast distances involved in an interstellar voyage."),
    /// });
    /// # movies.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_document<T: 'static + DeserializeOwned + Send + Sync>(
        &self,
        document_id: &str,
    ) -> Result<T, Error> {
        let url = format!(
            "{}/indexes/{}/documents/{}",
            self.client.host, self.uid, document_id
        );
        self.client
            .http_client
            .request::<(), (), T>(&url, Method::Get { query: () }, 200)
            .await
    }

    /// Get one document with parameters.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, documents::*};
    /// # use serde::{Deserialize, Serialize};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// #[derive(Debug, Serialize, Deserialize, PartialEq)]
    /// struct MyObject {
    ///     id: String,
    ///     kind: String,
    /// }
    ///
    /// #[derive(Debug, Serialize, Deserialize, PartialEq)]
    /// struct MyObjectReduced {
    ///     id: String,
    /// }
    /// # let index = client.index("document_query_execute");
    /// # index.add_or_replace(&[MyObject{id:"1".to_string(), kind:String::from("a kind")},MyObject{id:"2".to_string(), kind:String::from("some kind")}], None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    ///
    /// let mut document_query = DocumentQuery::new(&index);
    /// document_query.with_fields(["id"]);
    ///
    /// let document = index.get_document_with::<MyObjectReduced>("1", &document_query).await.unwrap();
    ///
    /// assert_eq!(
    ///     document,
    ///     MyObjectReduced { id: "1".to_string() }
    /// );
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    pub async fn get_document_with<T: 'static + DeserializeOwned + Send + Sync>(
        &self,
        document_id: &str,
        document_query: &DocumentQuery<'_, Http>,
    ) -> Result<T, Error> {
        let url = format!(
            "{}/indexes/{}/documents/{}",
            self.client.host, self.uid, document_id
        );
        self.client
            .http_client
            .request::<&DocumentQuery<Http>, (), T>(
                &url,
                Method::Get {
                    query: document_query,
                },
                200,
            )
            .await
    }

    /// Get documents by batch.
    ///
    /// # Example
    ///
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// #[derive(Serialize, Deserialize, PartialEq, Debug)]
    /// struct Movie {
    ///     name: String,
    ///     description: String,
    /// }
    ///
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// let movie_index = client.index("get_documents");
    /// # movie_index.add_or_replace(&[Movie{name:String::from("Interstellar"), description:String::from("Interstellar chronicles the adventures of a group of explorers who make use of a newly discovered wormhole to surpass the limitations on human space travel and conquer the vast distances involved in an interstellar voyage.")}], Some("name")).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    ///
    /// // retrieve movies (you have to put some movies in the index before)
    /// let movies = movie_index.get_documents::<Movie>().await.unwrap();
    ///
    /// assert!(movies.results.len() > 0);
    /// # movie_index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_documents<T: DeserializeOwned + 'static + Send + Sync>(
        &self,
    ) -> Result<DocumentsResults<T>, Error> {
        let url = format!("{}/indexes/{}/documents", self.client.host, self.uid);
        self.client
            .http_client
            .request::<(), (), DocumentsResults<T>>(&url, Method::Get { query: () }, 200)
            .await
    }

    /// Get documents by batch with parameters.
    ///
    /// # Example
    ///
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::{client::*, indexes::*, documents::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// #[derive(Serialize, Deserialize, PartialEq, Debug)]
    /// struct Movie {
    ///     name: String,
    ///     description: String,
    /// }
    ///
    /// #[derive(Deserialize, Debug, PartialEq)]
    /// struct ReturnedMovie {
    ///     name: String,
    /// }
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    ///
    /// let movie_index = client.index("get_documents_with");
    /// # movie_index.add_or_replace(&[Movie{name:String::from("Interstellar"), description:String::from("Interstellar chronicles the adventures of a group of explorers who make use of a newly discovered wormhole to surpass the limitations on human space travel and conquer the vast distances involved in an interstellar voyage.")}], Some("name")).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    ///
    /// let mut query = DocumentsQuery::new(&movie_index);
    /// query.with_limit(1);
    /// query.with_fields(["name"]);
    /// // retrieve movies (you have to put some movies in the index before)
    /// let movies = movie_index.get_documents_with::<ReturnedMovie>(&query).await.unwrap();
    ///
    /// assert_eq!(movies.results.len(), 1);
    /// # movie_index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_documents_with<T: DeserializeOwned + 'static + Send + Sync>(
        &self,
        documents_query: &DocumentsQuery<'_, Http>,
    ) -> Result<DocumentsResults<T>, Error> {
        if documents_query.filter.is_some() {
            let url = format!("{}/indexes/{}/documents/fetch", self.client.host, self.uid);
            return self
                .client
                .http_client
                .request::<(), &DocumentsQuery<Http>, DocumentsResults<T>>(
                    &url,
                    Method::Post {
                        body: documents_query,
                        query: (),
                    },
                    200,
                )
                .await
                .map_err(|err| match err {
                    Error::MeilisearchCommunication(error) => {
                        Error::MeilisearchCommunication(MeilisearchCommunicationError {
                            status_code: error.status_code,
                            url: error.url,
                            message: Some(format!("{}.", MEILISEARCH_VERSION_HINT)),
                        })
                    }
                    Error::Meilisearch(error) => Error::Meilisearch(MeilisearchError {
                        error_code: error.error_code,
                        error_link: error.error_link,
                        error_type: error.error_type,
                        error_message: format!(
                            "{}\n{}.",
                            error.error_message, MEILISEARCH_VERSION_HINT
                        ),
                    }),
                    _ => err,
                });
        }

        let url = format!("{}/indexes/{}/documents", self.client.host, self.uid);
        self.client
            .http_client
            .request::<&DocumentsQuery<Http>, (), DocumentsResults<T>>(
                &url,
                Method::Get {
                    query: documents_query,
                },
                200,
            )
            .await
    }

    /// Add a list of documents or replace them if they already exist.
    ///
    /// If you send an already existing document (same id) the **whole existing document** will be overwritten by the new document.
    /// Fields previously in the document not present in the new document are removed.
    ///
    /// For a partial update of the document see [`Index::add_or_update`].
    ///
    /// You can use the alias [`Index::add_documents`] if you prefer.
    ///
    /// # Example
    ///
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// # use std::thread::sleep;
    /// # use std::time::Duration;
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct Movie {
    ///     name: String,
    ///     description: String,
    /// }
    ///
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// let movie_index = client.index("add_or_replace");
    ///
    /// let task = movie_index.add_or_replace(&[
    ///     Movie{
    ///         name: String::from("Interstellar"),
    ///         description: String::from("Interstellar chronicles the adventures of a group of explorers who make use of a newly discovered wormhole to surpass the limitations on human space travel and conquer the vast distances involved in an interstellar voyage.")
    ///     },
    ///     Movie{
    ///         // note that the id field can only take alphanumerics characters (and '-' and '/')
    ///         name: String::from("MrsDoubtfire"),
    ///         description: String::from("Loving but irresponsible dad Daniel Hillard, estranged from his exasperated spouse, is crushed by a court order allowing only weekly visits with his kids. When Daniel learns his ex needs a housekeeper, he gets the job -- disguised as an English nanny. Soon he becomes not only his children's best pal but the kind of parent he should have been from the start.")
    ///     },
    ///     Movie{
    ///         name: String::from("Apollo13"),
    ///         description: String::from("The true story of technical troubles that scuttle the Apollo 13 lunar mission in 1971, risking the lives of astronaut Jim Lovell and his crew, with the failed journey turning into a thrilling saga of heroism. Drifting more than 200,000 miles from Earth, the astronauts work furiously with the ground crew to avert tragedy.")
    ///     },
    /// ], Some("name")).await.unwrap();
    /// // Meilisearch may take some time to execute the request so we are going to wait till it's completed
    /// client.wait_for_task(task, None, None).await.unwrap();
    ///
    /// let movies = movie_index.get_documents::<Movie>().await.unwrap();
    /// assert!(movies.results.len() >= 3);
    /// # movie_index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn add_or_replace<T: Serialize + Send + Sync>(
        &self,
        documents: &[T],
        primary_key: Option<&str>,
    ) -> Result<TaskInfo, Error> {
        let url = if let Some(primary_key) = primary_key {
            format!(
                "{}/indexes/{}/documents?primaryKey={}",
                self.client.host, self.uid, primary_key
            )
        } else {
            format!("{}/indexes/{}/documents", self.client.host, self.uid)
        };
        self.client
            .http_client
            .request::<(), &[T], TaskInfo>(
                &url,
                Method::Post {
                    query: (),
                    body: documents,
                },
                202,
            )
            .await
    }

    /// Add a raw and unchecked payload to meilisearch.
    ///
    /// This can be useful if your application is only forwarding data from other sources.
    ///
    /// If you send an already existing document (same id) the **whole existing document** will be overwritten by the new document.
    /// Fields previously in the document not present in the new document are removed.
    ///
    /// For a partial update of the document see [`Index::add_or_update_unchecked_payload`].
    ///
    /// # Example
    ///
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// # use std::thread::sleep;
    /// # use std::time::Duration;
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// let movie_index = client.index("add_or_replace_unchecked_payload");
    ///
    /// let task = movie_index.add_or_replace_unchecked_payload(
    ///     r#"{ "id": 1, "body": "doggo" }
    ///     { "id": 2, "body": "catto" }"#.as_bytes(),
    ///     "application/x-ndjson",
    ///     Some("id"),
    ///   ).await.unwrap();
    /// // Meilisearch may take some time to execute the request so we are going to wait till it's completed
    /// client.wait_for_task(task, None, None).await.unwrap();
    ///
    /// let movies = movie_index.get_documents::<serde_json::Value>().await.unwrap();
    /// assert_eq!(movies.results.len(), 2);
    /// # movie_index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn add_or_replace_unchecked_payload<
        T: futures_io::AsyncRead + Send + Sync + 'static,
    >(
        &self,
        payload: T,
        content_type: &str,
        primary_key: Option<&str>,
    ) -> Result<TaskInfo, Error> {
        let url = if let Some(primary_key) = primary_key {
            format!(
                "{}/indexes/{}/documents?primaryKey={}",
                self.client.host, self.uid, primary_key
            )
        } else {
            format!("{}/indexes/{}/documents", self.client.host, self.uid)
        };
        self.client
            .http_client
            .stream_request::<(), T, TaskInfo>(
                &url,
                Method::Post {
                    query: (),
                    body: payload,
                },
                content_type,
                202,
            )
            .await
    }

    /// Alias for [`Index::add_or_replace`].
    pub async fn add_documents<T: Serialize + Send + Sync>(
        &self,
        documents: &[T],
        primary_key: Option<&str>,
    ) -> Result<TaskInfo, Error> {
        self.add_or_replace(documents, primary_key).await
    }

    /// Add a raw ndjson payload and update them if they already.
    ///
    /// It configures the correct content type for ndjson data.
    ///
    /// If you send an already existing document (same id) the old document will be only partially updated according to the fields of the new document.
    /// Thus, any fields not present in the new document are kept and remained unchanged.
    ///
    /// To completely overwrite a document, check out the [`Index::add_documents_ndjson`] documents method.
    ///
    /// # Example
    ///
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// # use std::thread::sleep;
    /// # use std::time::Duration;
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// let movie_index = client.index("update_documents_ndjson");
    ///
    /// let task = movie_index.update_documents_ndjson(
    ///     r#"{ "id": 1, "body": "doggo" }
    ///     { "id": 2, "body": "catto" }"#.as_bytes(),
    ///     Some("id"),
    ///   ).await.unwrap();
    /// // Meilisearch may take some time to execute the request so we are going to wait till it's completed
    /// client.wait_for_task(task, None, None).await.unwrap();
    ///
    /// let movies = movie_index.get_documents::<serde_json::Value>().await.unwrap();
    /// assert_eq!(movies.results.len(), 2);
    /// # movie_index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn update_documents_ndjson<T: futures_io::AsyncRead + Send + Sync + 'static>(
        &self,
        payload: T,
        primary_key: Option<&str>,
    ) -> Result<TaskInfo, Error> {
        self.add_or_update_unchecked_payload(payload, "application/x-ndjson", primary_key)
            .await
    }

    /// Add a raw ndjson payload to meilisearch.
    ///
    /// It configures the correct content type for ndjson data.
    ///
    /// If you send an already existing document (same id) the **whole existing document** will be overwritten by the new document.
    /// Fields previously in the document not present in the new document are removed.
    ///
    /// For a partial update of the document see [`Index::update_documents_ndjson`].
    ///
    /// # Example
    ///
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// # use std::thread::sleep;
    /// # use std::time::Duration;
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// let movie_index = client.index("add_documents_ndjson");
    ///
    /// let task = movie_index.add_documents_ndjson(
    ///     r#"{ "id": 1, "body": "doggo" }
    ///     { "id": 2, "body": "catto" }"#.as_bytes(),
    ///     Some("id"),
    ///   ).await.unwrap();
    /// // Meilisearch may take some time to execute the request so we are going to wait till it's completed
    /// client.wait_for_task(task, None, None).await.unwrap();
    ///
    /// let movies = movie_index.get_documents::<serde_json::Value>().await.unwrap();
    /// assert_eq!(movies.results.len(), 2);
    /// # movie_index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn add_documents_ndjson<T: futures_io::AsyncRead + Send + Sync + 'static>(
        &self,
        payload: T,
        primary_key: Option<&str>,
    ) -> Result<TaskInfo, Error> {
        self.add_or_replace_unchecked_payload(payload, "application/x-ndjson", primary_key)
            .await
    }

    /// Add a raw csv payload and update them if they already.
    ///
    /// It configures the correct content type for csv data.
    ///
    /// If you send an already existing document (same id) the old document will be only partially updated according to the fields of the new document.
    /// Thus, any fields not present in the new document are kept and remained unchanged.
    ///
    /// To completely overwrite a document, check out the [`Index::add_documents_csv`] documents method.
    ///
    /// # Example
    ///
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// # use std::thread::sleep;
    /// # use std::time::Duration;
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// let movie_index = client.index("update_documents_csv");
    ///
    /// let task = movie_index.update_documents_csv(
    ///     "id,body\n1,\"doggo\"\n2,\"catto\"".as_bytes(),
    ///     Some("id"),
    ///   ).await.unwrap();
    /// // Meilisearch may take some time to execute the request so we are going to wait till it's completed
    /// client.wait_for_task(task, None, None).await.unwrap();
    ///
    /// let movies = movie_index.get_documents::<serde_json::Value>().await.unwrap();
    /// assert_eq!(movies.results.len(), 2);
    /// # movie_index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn update_documents_csv<T: futures_io::AsyncRead + Send + Sync + 'static>(
        &self,
        payload: T,
        primary_key: Option<&str>,
    ) -> Result<TaskInfo, Error> {
        self.add_or_update_unchecked_payload(payload, "text/csv", primary_key)
            .await
    }

    /// Add a raw csv payload to meilisearch.
    ///
    /// It configures the correct content type for csv data.
    ///
    /// If you send an already existing document (same id) the **whole existing document** will be overwritten by the new document.
    /// Fields previously in the document not present in the new document are removed.
    ///
    /// For a partial update of the document see [`Index::update_documents_csv`].
    ///
    /// # Example
    ///
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// # use std::thread::sleep;
    /// # use std::time::Duration;
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// let movie_index = client.index("add_documents_csv");
    ///
    /// let task = movie_index.add_documents_csv(
    ///     "id,body\n1,\"doggo\"\n2,\"catto\"".as_bytes(),
    ///     Some("id"),
    ///   ).await.unwrap();
    /// // Meilisearch may take some time to execute the request so we are going to wait till it's completed
    /// client.wait_for_task(task, None, None).await.unwrap();
    ///
    /// let movies = movie_index.get_documents::<serde_json::Value>().await.unwrap();
    /// assert_eq!(movies.results.len(), 2);
    /// # movie_index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn add_documents_csv<T: futures_io::AsyncRead + Send + Sync + 'static>(
        &self,
        payload: T,
        primary_key: Option<&str>,
    ) -> Result<TaskInfo, Error> {
        self.add_or_replace_unchecked_payload(payload, "text/csv", primary_key)
            .await
    }

    /// Add a list of documents and update them if they already.
    ///
    /// If you send an already existing document (same id) the old document will be only partially updated according to the fields of the new document.
    /// Thus, any fields not present in the new document are kept and remained unchanged.
    ///
    /// To completely overwrite a document, check out the [`Index::add_or_replace`] documents method.
    ///
    /// # Example
    ///
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::client::*;
    /// # use std::thread::sleep;
    /// # use std::time::Duration;
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct Movie {
    ///     name: String,
    ///     description: String,
    /// }
    ///
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// let movie_index = client.index("add_or_update");
    ///
    /// let task = movie_index.add_or_update(&[
    ///     Movie {
    ///         name: String::from("Interstellar"),
    ///         description: String::from("Interstellar chronicles the adventures of a group of explorers who make use of a newly discovered wormhole to surpass the limitations on human space travel and conquer the vast distances involved in an interstellar voyage.")
    ///     },
    ///     Movie {
    ///         // note that the id field can only take alphanumerics characters (and '-' and '/')
    ///         name: String::from("MrsDoubtfire"),
    ///         description: String::from("Loving but irresponsible dad Daniel Hillard, estranged from his exasperated spouse, is crushed by a court order allowing only weekly visits with his kids. When Daniel learns his ex needs a housekeeper, he gets the job -- disguised as an English nanny. Soon he becomes not only his children's best pal but the kind of parent he should have been from the start.")
    ///     },
    ///     Movie {
    ///         name: String::from("Apollo13"),
    ///         description: String::from("The true story of technical troubles that scuttle the Apollo 13 lunar mission in 1971, risking the lives of astronaut Jim Lovell and his crew, with the failed journey turning into a thrilling saga of heroism. Drifting more than 200,000 miles from Earth, the astronauts work furiously with the ground crew to avert tragedy.")
    ///     },
    /// ], Some("name")).await.unwrap();
    ///
    /// // Meilisearch may take some time to execute the request so we are going to wait till it's completed
    /// client.wait_for_task(task, None, None).await.unwrap();
    ///
    /// let movies = movie_index.get_documents::<Movie>().await.unwrap();
    /// assert!(movies.results.len() >= 3);
    /// # movie_index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn add_or_update<T: Serialize + Send + Sync>(
        &self,
        documents: &[T],
        primary_key: Option<&str>,
    ) -> Result<TaskInfo, Error> {
        let url = if let Some(primary_key) = primary_key {
            format!(
                "{}/indexes/{}/documents?primaryKey={}",
                self.client.host, self.uid, primary_key
            )
        } else {
            format!("{}/indexes/{}/documents", self.client.host, self.uid)
        };
        self.client
            .http_client
            .request::<(), &[T], TaskInfo>(
                &url,
                Method::Put {
                    query: (),
                    body: documents,
                },
                202,
            )
            .await
    }

    /// Add a raw and unchecked payload to meilisearch.
    ///
    /// This can be useful if your application is only forwarding data from other sources.
    ///
    /// If you send an already existing document (same id) the old document will be only partially updated according to the fields of the new document.
    /// Thus, any fields not present in the new document are kept and remained unchanged.
    ///
    /// To completely overwrite a document, check out the [`Index::add_or_replace_unchecked_payload`] documents method.
    ///
    /// # Example
    ///
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// # use std::thread::sleep;
    /// # use std::time::Duration;
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// let movie_index = client.index("add_or_replace_unchecked_payload");
    ///
    /// let task = movie_index.add_or_update_unchecked_payload(
    ///     r#"{ "id": 1, "body": "doggo" }
    ///     { "id": 2, "body": "catto" }"#.as_bytes(),
    ///     "application/x-ndjson",
    ///     Some("id"),
    /// ).await.unwrap();
    /// // Meilisearch may take some time to execute the request so we are going to wait till it's completed
    /// client.wait_for_task(task, None, None).await.unwrap();
    ///
    /// let movies = movie_index.get_documents::<serde_json::Value>().await.unwrap();
    ///
    /// assert_eq!(movies.results.len(), 2);
    /// # movie_index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn add_or_update_unchecked_payload<
        T: futures_io::AsyncRead + Send + Sync + 'static,
    >(
        &self,
        payload: T,
        content_type: &str,
        primary_key: Option<&str>,
    ) -> Result<TaskInfo, Error> {
        let url = if let Some(primary_key) = primary_key {
            format!(
                "{}/indexes/{}/documents?primaryKey={}",
                self.client.host, self.uid, primary_key
            )
        } else {
            format!("{}/indexes/{}/documents", self.client.host, self.uid)
        };
        self.client
            .http_client
            .stream_request::<(), T, TaskInfo>(
                &url,
                Method::Put {
                    query: (),
                    body: payload,
                },
                content_type,
                202,
            )
            .await
    }

    /// Delete all documents in the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # #[derive(Serialize, Deserialize, Debug)]
    /// # struct Movie {
    /// #    name: String,
    /// #    description: String,
    /// # }
    /// #
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// #
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// let movie_index = client.index("delete_all_documents");
    /// #
    /// # movie_index.add_or_replace(&[Movie{name:String::from("Interstellar"), description:String::from("Interstellar chronicles the adventures of a group of explorers who make use of a newly discovered wormhole to surpass the limitations on human space travel and conquer the vast distances involved in an interstellar voyage.")}], Some("name")).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// #
    /// movie_index.delete_all_documents()
    ///     .await
    ///     .unwrap()
    ///     .wait_for_completion(&client, None, None)
    ///     .await
    ///     .unwrap();
    /// let movies = movie_index.get_documents::<Movie>().await.unwrap();
    /// assert_eq!(movies.results.len(), 0);
    /// # movie_index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn delete_all_documents(&self) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), (), TaskInfo>(
                &format!("{}/indexes/{}/documents", self.client.host, self.uid),
                Method::Delete { query: () },
                202,
            )
            .await
    }

    /// Delete one document based on its unique id.
    ///
    /// # Example
    ///
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::client::*;
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # #[derive(Serialize, Deserialize, Debug)]
    /// # struct Movie {
    /// #    name: String,
    /// #    description: String,
    /// # }
    /// #
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// #
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// let mut movies = client.index("delete_document");
    /// # movies.add_or_replace(&[Movie{name:String::from("Interstellar"), description:String::from("Interstellar chronicles the adventures of a group of explorers who make use of a newly discovered wormhole to surpass the limitations on human space travel and conquer the vast distances involved in an interstellar voyage.")}], Some("name")).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// // add a document with id = Interstellar
    /// movies.delete_document("Interstellar")
    ///     .await
    ///     .unwrap()
    ///     .wait_for_completion(&client, None, None)
    ///     .await
    ///     .unwrap();
    /// # movies.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn delete_document<T: Display>(&self, uid: T) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), (), TaskInfo>(
                &format!(
                    "{}/indexes/{}/documents/{}",
                    self.client.host, self.uid, uid
                ),
                Method::Delete { query: () },
                202,
            )
            .await
    }

    /// Delete a selection of documents based on array of document id's.
    ///
    /// # Example
    ///
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::client::*;
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # #[derive(Serialize, Deserialize, Debug)]
    /// # struct Movie {
    /// #    name: String,
    /// #    description: String,
    /// # }
    /// #
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// #
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// let movies = client.index("delete_documents");
    /// #
    /// # // add some documents
    /// # movies.add_or_replace(&[Movie{name:String::from("Interstellar"), description:String::from("Interstellar chronicles the adventures of a group of explorers who make use of a newly discovered wormhole to surpass the limitations on human space travel and conquer the vast distances involved in an interstellar voyage.")},Movie{name:String::from("Unknown"), description:String::from("Unknown")}], Some("name")).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// #
    /// // delete some documents
    /// movies.delete_documents(&["Interstellar", "Unknown"])
    ///     .await
    ///     .unwrap()
    ///     .wait_for_completion(&client, None, None)
    ///     .await
    ///     .unwrap();
    /// # movies.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn delete_documents<T: Display + Serialize + std::fmt::Debug + Send + Sync>(
        &self,
        uids: &[T],
    ) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), &[T], TaskInfo>(
                &format!(
                    "{}/indexes/{}/documents/delete-batch",
                    self.client.host, self.uid
                ),
                Method::Post {
                    query: (),
                    body: uids,
                },
                202,
            )
            .await
    }

    /// Delete a selection of documents with filters.
    ///
    /// # Example
    ///
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::{client::*, documents::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # #[derive(Serialize, Deserialize, Debug)]
    /// # struct Movie {
    /// #    name: String,
    /// #    id: String,
    /// # }
    /// #
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// #
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// let index = client.index("delete_documents_with");
    /// #
    /// # index.set_filterable_attributes(["id"]);
    /// # // add some documents
    /// # index.add_or_replace(&[Movie{id:String::from("1"), name: String::from("First movie") }, Movie{id:String::from("1"), name: String::from("First movie") }], Some("id")).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    ///
    /// let mut query = DocumentDeletionQuery::new(&index);
    /// query.with_filter("id = 1");
    /// // delete some documents
    /// index.delete_documents_with(&query)
    ///     .await
    ///     .unwrap()
    ///     .wait_for_completion(&client, None, None)
    ///     .await
    ///     .unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn delete_documents_with(
        &self,
        query: &DocumentDeletionQuery<'_, Http>,
    ) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), &DocumentDeletionQuery<Http>, TaskInfo>(
                &format!("{}/indexes/{}/documents/delete", self.client.host, self.uid),
                Method::Post {
                    query: (),
                    body: query,
                },
                202,
            )
            .await
    }

    /// Alias for the [`Index::update`] method.
    pub async fn set_primary_key(
        &mut self,
        primary_key: impl AsRef<str>,
    ) -> Result<TaskInfo, Error> {
        self.primary_key = Some(primary_key.as_ref().to_string());

        self.update().await
    }

    /// Fetch the information of the index as a raw JSON [Index], this index should already exist.
    ///
    /// If you use it directly from the [Client], you can use the method [`Client::get_raw_index`], which is the equivalent method from the client.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # let index = client.create_index("fetch_info", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap().try_make_index(&client).unwrap();
    /// let mut idx = client.index("fetch_info");
    /// idx.fetch_info().await.unwrap();
    ///
    /// println!("{idx:?}");
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn fetch_info(&mut self) -> Result<(), Error> {
        let v = self.client.get_raw_index(&self.uid).await?;
        *self = Index::from_value(v, self.client.clone())?;
        Ok(())
    }

    /// Fetch the primary key of the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # // create the client
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// let mut index = client.create_index("get_primary_key", Some("id"))
    ///     .await
    ///     .unwrap()
    ///     .wait_for_completion(&client, None, None)
    ///     .await.unwrap()
    ///     .try_make_index(&client)
    ///     .unwrap();
    ///
    /// let primary_key = index.get_primary_key().await.unwrap();
    ///
    /// assert_eq!(primary_key, Some("id"));
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_primary_key(&mut self) -> Result<Option<&str>, Error> {
        self.fetch_info().await?;
        Ok(self.primary_key.as_deref())
    }

    /// Get a [Task] from a specific [Index] to keep track of [asynchronous operations](https://www.meilisearch.com/docs/learn/advanced/asynchronous_operations).
    ///
    /// # Example
    ///
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use std::thread::sleep;
    /// # use std::time::Duration;
    /// # use meilisearch_sdk::{client::*, indexes::*, tasks::Task};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # #[derive(Debug, Serialize, Deserialize, PartialEq)]
    /// # struct Document {
    /// #    id: usize,
    /// #    value: String,
    /// #    kind: String,
    /// # }
    /// #
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// let movies = client.index("get_task");
    ///
    /// let task = movies.add_documents(&[
    ///     Document { id: 0, kind: "title".into(), value: "The Social Network".to_string() }
    /// ], None).await.unwrap();
    /// # task.clone().wait_for_completion(&client, None, None).await.unwrap();
    ///
    /// // Get task status from the index, using `uid`
    /// let status = movies.get_task(&task).await.unwrap();
    ///
    /// let from_index = match status {
    ///     Task::Enqueued { content } => content.uid,
    ///     Task::Processing { content } => content.uid,
    ///     Task::Failed { content } => content.task.uid,
    ///     Task::Succeeded { content } => content.uid,
    /// };
    ///
    /// assert_eq!(task.get_task_uid(), from_index);
    /// # movies.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_task(&self, uid: impl AsRef<u32>) -> Result<Task, Error> {
        self.client
            .http_client
            .request::<(), (), Task>(
                &format!("{}/tasks/{}", self.client.host, uid.as_ref()),
                Method::Get { query: () },
                200,
            )
            .await
    }

    /// Get the status of all tasks in a given index.
    ///
    /// # Example
    ///
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # let index = client.create_index("get_tasks", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap().try_make_index(&client).unwrap();
    /// let tasks = index.get_tasks().await.unwrap();
    ///
    /// assert!(tasks.results.len() > 0);
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_tasks(&self) -> Result<TasksResults, Error> {
        let mut query = TasksSearchQuery::new(&self.client);
        query.with_index_uids([self.uid.as_str()]);

        self.client.get_tasks_with(&query).await
    }

    /// Get the status of all tasks in a given index.
    ///
    /// # Example
    ///
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::{client::*, indexes::*, tasks::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # let index = client.create_index("get_tasks_with", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap().try_make_index(&client).unwrap();
    /// let mut query = TasksSearchQuery::new(&client);
    /// query.with_index_uids(["none_existant"]);
    ///
    /// let tasks = index.get_tasks_with(&query).await.unwrap();
    ///
    /// assert!(tasks.results.len() > 0);
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_tasks_with(
        &self,
        tasks_query: &TasksQuery<'_, TasksPaginationFilters, Http>,
    ) -> Result<TasksResults, Error> {
        let mut query = tasks_query.clone();
        query.with_index_uids([self.uid.as_str()]);

        self.client.get_tasks_with(&query).await
    }

    /// Get stats of an index.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # let index = client.create_index("get_stats", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap().try_make_index(&client).unwrap();
    /// let stats = index.get_stats().await.unwrap();
    ///
    /// assert_eq!(stats.is_indexing, false);
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_stats(&self) -> Result<IndexStats, Error> {
        self.client
            .http_client
            .request::<(), (), IndexStats>(
                &format!("{}/indexes/{}/stats", self.client.host, self.uid),
                Method::Get { query: () },
                200,
            )
            .await
    }

    /// Wait until Meilisearch processes a [Task], and get its status.
    ///
    /// `interval` = The frequency at which the server should be polled. **Default = 50ms**
    ///
    /// `timeout` = The maximum time to wait for processing to complete. **Default = 5000ms**
    ///
    /// If the waited time exceeds `timeout` then an [`Error::Timeout`] will be returned.
    ///
    /// See also [`Client::wait_for_task`, `Task::wait_for_completion`].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, tasks::Task};
    /// # use serde::{Serialize, Deserialize};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # #[derive(Debug, Serialize, Deserialize, PartialEq)]
    /// # struct Document {
    /// #    id: usize,
    /// #    value: String,
    /// #    kind: String,
    /// # }
    /// #
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// let movies = client.index("movies_index_wait_for_task");
    ///
    /// let task = movies.add_documents(&[
    ///     Document { id: 0, kind: "title".into(), value: "The Social Network".to_string() },
    ///     Document { id: 1, kind: "title".into(), value: "Harry Potter and the Sorcerer's Stone".to_string() },
    /// ], None).await.unwrap();
    ///
    /// let status = movies.wait_for_task(task, None, None).await.unwrap();
    ///
    /// assert!(matches!(status, Task::Succeeded { .. }));
    /// # movies.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn wait_for_task(
        &self,
        task_id: impl AsRef<u32>,
        interval: Option<Duration>,
        timeout: Option<Duration>,
    ) -> Result<Task, Error> {
        self.client.wait_for_task(task_id, interval, timeout).await
    }

    /// Add documents to the index in batches.
    ///
    /// `documents` = A slice of documents
    ///
    /// `batch_size` = Optional parameter that allows you to specify the size of the batch
    ///
    /// **`batch_size` is 1000 by default**
    ///
    /// # Example
    ///
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::client::*;
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct Movie {
    ///     name: String,
    ///     description: String,
    /// }
    ///
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// let movie_index = client.index("add_documents_in_batches");
    ///
    /// let tasks = movie_index.add_documents_in_batches(&[
    ///  Movie {
    ///         name: String::from("Interstellar"),
    ///         description: String::from("Interstellar chronicles the adventures of a group of explorers who make use of a newly discovered wormhole to surpass the limitations on human space travel and conquer the vast distances involved in an interstellar voyage.")
    ///  },
    ///  Movie {
    ///         // note that the id field can only take alphanumerics characters (and '-' and '/')
    ///         name: String::from("MrsDoubtfire"),
    ///         description: String::from("Loving but irresponsible dad Daniel Hillard, estranged from his exasperated spouse, is crushed by a court order allowing only weekly visits with his kids. When Daniel learns his ex needs a housekeeper, he gets the job -- disguised as an English nanny. Soon he becomes not only his children's best pal but the kind of parent he should have been from the start.")
    ///  },
    ///  Movie {
    ///         name: String::from("Apollo13"),
    ///         description: String::from("The true story of technical troubles that scuttle the Apollo 13 lunar mission in 1971, risking the lives of astronaut Jim Lovell and his crew, with the failed journey turning into a thrilling saga of heroism. Drifting more than 200,000 miles from Earth, the astronauts work furiously with the ground crew to avert tragedy.")
    ///     }],
    ///     Some(1),
    ///     Some("name")
    /// ).await.unwrap();
    ///
    /// client.wait_for_task(tasks.last().unwrap(), None, None).await.unwrap();
    ///
    /// let movies = movie_index.get_documents::<Movie>().await.unwrap();
    ///
    /// assert!(movies.results.len() >= 3);
    /// # movie_index.delete().await.unwrap().wait_for_completion(&client, None,
    /// # None).await.unwrap();
    /// # });
    /// ```
    pub async fn add_documents_in_batches<T: Serialize + Send + Sync>(
        &self,
        documents: &[T],
        batch_size: Option<usize>,
        primary_key: Option<&str>,
    ) -> Result<Vec<TaskInfo>, Error> {
        let mut task = Vec::with_capacity(documents.len());
        for document_batch in documents.chunks(batch_size.unwrap_or(1000)) {
            task.push(self.add_documents(document_batch, primary_key).await?);
        }
        Ok(task)
    }

    /// Update documents to the index in batches.
    ///
    /// `documents` = A slice of documents
    ///
    /// `batch_size` = Optional parameter that allows you to specify the size of the batch
    ///
    /// **`batch_size` is 1000 by default**
    ///
    /// # Example
    ///
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::client::*;
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
    /// struct Movie {
    ///     name: String,
    ///     description: String,
    /// }
    ///
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// let movie_index = client.index("update_documents_in_batches");
    ///
    /// let tasks = movie_index.add_documents_in_batches(&[
    ///  Movie {
    ///         name: String::from("Interstellar"),
    ///         description: String::from("Interstellar chronicles the adventures of a group of explorers who make use of a newly discovered wormhole to surpass the limitations on human space travel and conquer the vast distances involved in an interstellar voyage.")
    ///  },
    ///  Movie {
    ///         // note that the id field can only take alphanumerics characters (and '-' and '/')
    ///         name: String::from("MrsDoubtfire"),
    ///         description: String::from("Loving but irresponsible dad Daniel Hillard, estranged from his exasperated spouse, is crushed by a court order allowing only weekly visits with his kids. When Daniel learns his ex needs a housekeeper, he gets the job -- disguised as an English nanny. Soon he becomes not only his children's best pal but the kind of parent he should have been from the start.")
    ///  },
    ///  Movie {
    ///         name: String::from("Apollo13"),
    ///         description: String::from("The true story of technical troubles that scuttle the Apollo 13 lunar mission in 1971, risking the lives of astronaut Jim Lovell and his crew, with the failed journey turning into a thrilling saga of heroism. Drifting more than 200,000 miles from Earth, the astronauts work furiously with the ground crew to avert tragedy.")
    ///     }],
    ///     Some(1),
    ///     Some("name")
    /// ).await.unwrap();
    ///
    /// client.wait_for_task(tasks.last().unwrap(), None, None).await.unwrap();
    ///
    /// let movies = movie_index.get_documents::<Movie>().await.unwrap();
    /// assert!(movies.results.len() >= 3);
    ///
    /// let updated_movies = [
    ///  Movie {
    ///         name: String::from("Interstellar"),
    ///         description: String::from("Updated!")
    ///  },
    ///  Movie {
    ///         // note that the id field can only take alphanumerics characters (and '-' and '/')
    ///         name: String::from("MrsDoubtfire"),
    ///         description: String::from("Updated!")
    ///  },
    ///  Movie {
    ///         name: String::from("Apollo13"),
    ///         description: String::from("Updated!")
    /// }];
    ///
    /// let tasks = movie_index.update_documents_in_batches(&updated_movies, Some(1), None).await.unwrap();
    ///
    /// client.wait_for_task(tasks.last().unwrap(), None, None).await.unwrap();
    ///
    /// let movies_updated = movie_index.get_documents::<Movie>().await.unwrap();
    ///
    /// assert!(movies_updated.results.len() >= 3);
    /// # movie_index.delete().await.unwrap().wait_for_completion(&client, None,
    /// # None).await.unwrap();
    /// # });
    /// ```
    pub async fn update_documents_in_batches<T: Serialize + Send + Sync>(
        &self,
        documents: &[T],
        batch_size: Option<usize>,
        primary_key: Option<&str>,
    ) -> Result<Vec<TaskInfo>, Error> {
        let mut task = Vec::with_capacity(documents.len());
        for document_batch in documents.chunks(batch_size.unwrap_or(1000)) {
            task.push(self.add_or_update(document_batch, primary_key).await?);
        }
        Ok(task)
    }

    /// Get similar documents in the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::{client::*, indexes::*, similar::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct Movie {
    ///     name: String,
    ///     description: String,
    /// }
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// let movies = client.index("execute_query");
    ///
    /// // add some documents
    /// # movies.add_or_replace(&[Movie{name:String::from("Interstellar"), description:String::from("Interstellar chronicles the adventures of a group of explorers who make use of a newly discovered wormhole to surpass the limitations on human space travel and conquer the vast distances involved in an interstellar voyage.")},Movie{name:String::from("Unknown"), description:String::from("Unknown")}], Some("name")).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    ///
    /// let query = SimilarQuery::new(&movies, "1", "default").build();
    /// let results = query.similar_query::<Movie>(&query).await.unwrap();
    ///
    /// assert!(results.hits.len() > 0);
    /// # movies.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn similar_query<T: 'static + DeserializeOwned + Send + Sync>(
        &self,
        body: &SimilarQuery<'_, Http>,
    ) -> Result<SimilarResults<T>, Error> {
        self.client
            .http_client
            .request::<(), &SimilarQuery<Http>, SimilarResults<T>>(
                &format!("{}/indexes/{}/similar", self.client.host, self.uid),
                Method::Post { body, query: () },
                200,
            )
            .await
    }
}

impl<Http: HttpClient> AsRef<str> for Index<Http> {
    fn as_ref(&self) -> &str {
        &self.uid
    }
}

/// An [`IndexUpdater`] used to update the specifics of an index.
///
/// # Example
///
/// ```
/// # use meilisearch_sdk::{client::*, indexes::*, task_info::*, tasks::{Task, SucceededTask}};
/// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
/// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
/// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
/// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
/// # let index = client
/// #   .create_index("index_updater", None)
/// #   .await
/// #   .unwrap()
/// #   .wait_for_completion(&client, None, None)
/// #   .await
/// #   .unwrap()
/// # // Once the task finished, we try to create an `Index` out of it
/// #   .try_make_index(&client)
/// #   .unwrap();
/// let task = IndexUpdater::new("index_updater", &client)
///     .with_primary_key("special_id")
///     .execute()
///     .await
///     .unwrap()
///     .wait_for_completion(&client, None, None)
///     .await
///     .unwrap();
///
/// let index = client.get_index("index_updater").await.unwrap();
///
/// assert_eq!(index.primary_key, Some("special_id".to_string()));
/// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
/// # });
/// ```
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndexUpdater<'a, Http: HttpClient> {
    #[serde(skip)]
    pub client: &'a Client<Http>,
    #[serde(skip_serializing)]
    pub uid: String,
    pub primary_key: Option<String>,
}

impl<'a, Http: HttpClient> IndexUpdater<'a, Http> {
    pub fn new(uid: impl AsRef<str>, client: &Client<Http>) -> IndexUpdater<Http> {
        IndexUpdater {
            client,
            primary_key: None,
            uid: uid.as_ref().to_string(),
        }
    }
    /// Define the new `primary_key` to set on the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, task_info::*, tasks::{Task, SucceededTask}};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # let index = client
    /// #   .create_index("index_updater_with_primary_key", None)
    /// #   .await
    /// #   .unwrap()
    /// #   .wait_for_completion(&client, None, None)
    /// #   .await
    /// #   .unwrap()
    /// # // Once the task finished, we try to create an `Index` out of it
    /// #   .try_make_index(&client)
    /// #   .unwrap();
    /// let task = IndexUpdater::new("index_updater_with_primary_key", &client)
    ///     .with_primary_key("special_id")
    ///     .execute()
    ///     .await
    ///     .unwrap()
    ///     .wait_for_completion(&client, None, None)
    ///     .await
    ///     .unwrap();
    ///
    /// let index = client.get_index("index_updater_with_primary_key").await.unwrap();
    ///
    /// assert_eq!(index.primary_key, Some("special_id".to_string()));
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub fn with_primary_key(
        &mut self,
        primary_key: impl AsRef<str>,
    ) -> &mut IndexUpdater<'a, Http> {
        self.primary_key = Some(primary_key.as_ref().to_string());
        self
    }

    /// Execute the update of an [Index] using the [`IndexUpdater`].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, task_info::*, tasks::{Task, SucceededTask}};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # let index = client
    /// #   .create_index("index_updater_execute", None)
    /// #   .await
    /// #   .unwrap()
    /// #   .wait_for_completion(&client, None, None)
    /// #   .await
    /// #   .unwrap()
    /// # // Once the task finished, we try to create an `Index` out of it
    /// #   .try_make_index(&client)
    /// #   .unwrap();
    /// let task = IndexUpdater::new("index_updater_execute", &client)
    ///     .with_primary_key("special_id")
    ///     .execute()
    ///     .await
    ///     .unwrap()
    ///     .wait_for_completion(&client, None, None)
    ///     .await
    ///     .unwrap();
    ///
    /// let index = client.get_index("index_updater_execute").await.unwrap();
    ///
    /// assert_eq!(index.primary_key, Some("special_id".to_string()));
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn execute(&'a self) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), &IndexUpdater<Http>, TaskInfo>(
                &format!("{}/indexes/{}", self.client.host, self.uid),
                Method::Patch {
                    query: (),
                    body: self,
                },
                202,
            )
            .await
    }
}

impl<Http: HttpClient> AsRef<str> for IndexUpdater<'_, Http> {
    fn as_ref(&self) -> &str {
        &self.uid
    }
}

impl<'a, Http: HttpClient> AsRef<IndexUpdater<'a, Http>> for IndexUpdater<'a, Http> {
    fn as_ref(&self) -> &IndexUpdater<'a, Http> {
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexStats {
    pub number_of_documents: usize,
    pub is_indexing: bool,
    pub field_distribution: HashMap<String, usize>,
}

/// An [`IndexesQuery`] containing filter and pagination parameters when searching for [Indexes](Index).
///
/// # Example
///
/// ```
/// # use meilisearch_sdk::{client::*, indexes::*};
/// #
/// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
/// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
/// #
/// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
/// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
/// # let index = client
/// #   .create_index("index_query_builder", None)
/// #   .await
/// #   .unwrap()
/// #   .wait_for_completion(&client, None, None)
/// #   .await
/// #   .unwrap()
/// #   // Once the task finished, we try to create an `Index` out of it.
/// #   .try_make_index(&client)
/// #   .unwrap();
/// let mut indexes = IndexesQuery::new(&client)
///     .with_limit(1)
///     .execute().await.unwrap();
///
/// assert_eq!(indexes.results.len(), 1);
/// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
/// # });
/// ```
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndexesQuery<'a, Http: HttpClient> {
    #[serde(skip_serializing)]
    pub client: &'a Client<Http>,
    /// The number of [Indexes](Index) to skip.
    ///
    /// If the value of the parameter `offset` is `n`, the `n` first indexes will not be returned.
    /// This is helpful for pagination.
    ///
    /// Example: If you want to skip the first index, set offset to `1`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,

    /// The maximum number of [Indexes](Index) returned.
    ///
    /// If the value of the parameter `limit` is `n`, there will never be more than `n` indexes in the response.
    /// This is helpful for pagination.
    ///
    /// Example: If you don't want to get more than two indexes, set limit to `2`.
    ///
    /// **Default: `20`**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

impl<'a, Http: HttpClient> IndexesQuery<'a, Http> {
    #[must_use]
    pub fn new(client: &Client<Http>) -> IndexesQuery<Http> {
        IndexesQuery {
            client,
            offset: None,
            limit: None,
        }
    }

    /// Specify the offset.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # let index = client
    /// #   .create_index("index_query_with_offset", None)
    /// #   .await
    /// #   .unwrap()
    /// #   .wait_for_completion(&client, None, None)
    /// #   .await
    /// #   .unwrap()
    /// #   // Once the task finished, we try to create an `Index` out of it
    /// #   .try_make_index(&client)
    /// #   .unwrap();
    /// let mut indexes = IndexesQuery::new(&client)
    ///     .with_offset(1)
    ///     .execute().await.unwrap();
    ///
    /// assert_eq!(indexes.offset, 1);
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub fn with_offset(&mut self, offset: usize) -> &mut IndexesQuery<'a, Http> {
        self.offset = Some(offset);
        self
    }

    /// Specify the maximum number of [Indexes](Index) to return.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # let index = client
    /// #   .create_index("index_query_with_limit", None)
    /// #   .await
    /// #   .unwrap()
    /// #   .wait_for_completion(&client, None, None)
    /// #   .await
    /// #   .unwrap()
    /// #   // Once the task finished, we try to create an `Index` out of it
    /// #   .try_make_index(&client)
    /// #   .unwrap();
    /// let mut indexes = IndexesQuery::new(&client)
    ///     .with_limit(1)
    ///     .execute().await.unwrap();
    ///
    /// assert_eq!(indexes.results.len(), 1);
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub fn with_limit(&mut self, limit: usize) -> &mut IndexesQuery<'a, Http> {
        self.limit = Some(limit);
        self
    }
    /// Get [Indexes](Index).
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{indexes::IndexesQuery, client::Client};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # let index = client
    /// #   .create_index("index_query_with_execute", None)
    /// #   .await
    /// #   .unwrap()
    /// #   .wait_for_completion(&client, None, None)
    /// #   .await
    /// #   .unwrap()
    /// #   // Once the task finished, we try to create an `Index` out of it
    /// #   .try_make_index(&client)
    /// #   .unwrap();
    /// let mut indexes = IndexesQuery::new(&client)
    ///     .with_limit(1)
    ///     .execute().await.unwrap();
    ///
    /// assert_eq!(indexes.results.len(), 1);
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn execute(&self) -> Result<IndexesResults<Http>, Error> {
        self.client.list_all_indexes_with(self).await
    }
}

#[derive(Debug, Clone)]
pub struct IndexesResults<Http: HttpClient = DefaultHttpClient> {
    pub results: Vec<Index<Http>>,
    pub limit: u32,
    pub offset: u32,
    pub total: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    use big_s::S;
    use meilisearch_test_macro::meilisearch_test;
    use serde_json::json;

    #[meilisearch_test]
    async fn test_from_value(client: Client) {
        let t = OffsetDateTime::now_utc();
        let trfc3339 = t
            .format(&time::format_description::well_known::Rfc3339)
            .unwrap();

        let value = json!({
            "createdAt": &trfc3339,
            "primaryKey": null,
            "uid": "test_from_value",
            "updatedAt": &trfc3339,
        });

        let idx = Index {
            uid: S("test_from_value"),
            primary_key: None,
            created_at: Some(t),
            updated_at: Some(t),
            client: client.clone(),
        };

        let res = Index::from_value(value, client).unwrap();

        assert_eq!(res.updated_at, idx.updated_at);
        assert_eq!(res.created_at, idx.created_at);
        assert_eq!(res.uid, idx.uid);
        assert_eq!(res.primary_key, idx.primary_key);
        assert_eq!(res.client.host, idx.client.host);
        assert_eq!(res.client.api_key, idx.client.api_key);
    }

    #[meilisearch_test]
    async fn test_fetch_info(mut index: Index) {
        let res = index.fetch_info().await;
        assert!(res.is_ok());
        assert!(index.updated_at.is_some());
        assert!(index.created_at.is_some());
        assert!(index.primary_key.is_none());
    }

    #[meilisearch_test]
    async fn test_get_documents(index: Index) {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Object {
            id: usize,
            value: String,
            kind: String,
        }
        let res = index.get_documents::<Object>().await.unwrap();

        assert_eq!(res.limit, 20);
    }

    #[meilisearch_test]
    async fn test_get_documents_with(index: Index) {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Object {
            id: usize,
            value: String,
            kind: String,
        }

        let mut documents_query = DocumentsQuery::new(&index);
        documents_query.with_limit(1).with_offset(2);

        let res = index
            .get_documents_with::<Object>(&documents_query)
            .await
            .unwrap();

        assert_eq!(res.limit, 1);
        assert_eq!(res.offset, 2);
    }

    #[meilisearch_test]
    async fn test_update_document_json(client: Client, index: Index) -> Result<(), Error> {
        let old_json = [
            json!({ "id": 1, "body": "doggo" }),
            json!({ "id": 2, "body": "catto" }),
        ];
        let updated_json = [
            json!({ "id": 1, "second_body": "second_doggo" }),
            json!({ "id": 2, "second_body": "second_catto" }),
        ];

        let task = index
            .add_documents(&old_json, Some("id"))
            .await
            .unwrap()
            .wait_for_completion(&client, None, None)
            .await
            .unwrap();
        let _ = index.get_task(task).await?;

        let task = index
            .add_or_update(&updated_json, None)
            .await
            .unwrap()
            .wait_for_completion(&client, None, None)
            .await
            .unwrap();

        let status = index.get_task(task).await?;
        let elements = index.get_documents::<serde_json::Value>().await.unwrap();

        assert!(matches!(status, Task::Succeeded { .. }));
        assert_eq!(elements.results.len(), 2);

        let expected_result = vec![
            json!( {"body": "doggo", "id": 1, "second_body": "second_doggo"}),
            json!( {"body": "catto", "id": 2, "second_body": "second_catto"}),
        ];

        assert_eq!(elements.results, expected_result);

        Ok(())
    }

    #[meilisearch_test]
    async fn test_add_documents_ndjson(client: Client, index: Index) -> Result<(), Error> {
        let ndjson = r#"{ "id": 1, "body": "doggo" }{ "id": 2, "body": "catto" }"#.as_bytes();

        let task = index
            .add_documents_ndjson(ndjson, Some("id"))
            .await?
            .wait_for_completion(&client, None, None)
            .await?;

        let status = index.get_task(task).await?;
        let elements = index.get_documents::<serde_json::Value>().await.unwrap();
        assert!(matches!(status, Task::Succeeded { .. }));
        assert_eq!(elements.results.len(), 2);

        Ok(())
    }

    #[meilisearch_test]
    async fn test_update_documents_ndjson(client: Client, index: Index) -> Result<(), Error> {
        let old_ndjson = r#"{ "id": 1, "body": "doggo" }{ "id": 2, "body": "catto" }"#.as_bytes();
        let updated_ndjson =
            r#"{ "id": 1, "second_body": "second_doggo" }{ "id": 2, "second_body": "second_catto" }"#.as_bytes();
        // Add first njdson document
        let task = index
            .add_documents_ndjson(old_ndjson, Some("id"))
            .await?
            .wait_for_completion(&client, None, None)
            .await?;
        let _ = index.get_task(task).await?;

        // Update via njdson document
        let task = index
            .update_documents_ndjson(updated_ndjson, Some("id"))
            .await?
            .wait_for_completion(&client, None, None)
            .await?;

        let status = index.get_task(task).await?;
        let elements = index.get_documents::<serde_json::Value>().await.unwrap();

        assert!(matches!(status, Task::Succeeded { .. }));
        assert_eq!(elements.results.len(), 2);

        let expected_result = vec![
            json!( {"body": "doggo", "id": 1, "second_body": "second_doggo"}),
            json!( {"body": "catto", "id": 2, "second_body": "second_catto"}),
        ];

        assert_eq!(elements.results, expected_result);

        Ok(())
    }

    #[meilisearch_test]
    async fn test_add_documents_csv(client: Client, index: Index) -> Result<(), Error> {
        let csv_input = "id,body\n1,\"doggo\"\n2,\"catto\"".as_bytes();

        let task = index
            .add_documents_csv(csv_input, Some("id"))
            .await?
            .wait_for_completion(&client, None, None)
            .await?;

        let status = index.get_task(task).await?;
        let elements = index.get_documents::<serde_json::Value>().await.unwrap();
        assert!(matches!(status, Task::Succeeded { .. }));
        assert_eq!(elements.results.len(), 2);

        Ok(())
    }

    #[meilisearch_test]
    async fn test_update_documents_csv(client: Client, index: Index) -> Result<(), Error> {
        let old_csv = "id,body\n1,\"doggo\"\n2,\"catto\"".as_bytes();
        let updated_csv = "id,body\n1,\"new_doggo\"\n2,\"new_catto\"".as_bytes();
        // Add first njdson document
        let task = index
            .add_documents_csv(old_csv, Some("id"))
            .await?
            .wait_for_completion(&client, None, None)
            .await?;
        let _ = index.get_task(task).await?;

        // Update via njdson document
        let task = index
            .update_documents_csv(updated_csv, Some("id"))
            .await?
            .wait_for_completion(&client, None, None)
            .await?;

        let status = index.get_task(task).await?;
        let elements = index.get_documents::<serde_json::Value>().await.unwrap();

        assert!(matches!(status, Task::Succeeded { .. }));
        assert_eq!(elements.results.len(), 2);

        let expected_result = vec![
            json!( {"body": "new_doggo", "id": "1"}),
            json!( {"body": "new_catto", "id": "2"}),
        ];

        assert_eq!(elements.results, expected_result);

        Ok(())
    }
    #[meilisearch_test]

    async fn test_get_one_task(client: Client, index: Index) -> Result<(), Error> {
        let task = index
            .delete_all_documents()
            .await?
            .wait_for_completion(&client, None, None)
            .await?;

        let status = index.get_task(task).await?;

        match status {
            Task::Enqueued {
                content:
                    EnqueuedTask {
                        index_uid: Some(index_uid),
                        ..
                    },
            }
            | Task::Processing {
                content:
                    ProcessingTask {
                        index_uid: Some(index_uid),
                        ..
                    },
            }
            | Task::Failed {
                content:
                    FailedTask {
                        task:
                            SucceededTask {
                                index_uid: Some(index_uid),
                                ..
                            },
                        ..
                    },
            }
            | Task::Succeeded {
                content:
                    SucceededTask {
                        index_uid: Some(index_uid),
                        ..
                    },
            } => assert_eq!(index_uid, *index.uid),
            task => panic!(
                "The task should have an index_uid that is not null {:?}",
                task
            ),
        }
        Ok(())
    }
}
