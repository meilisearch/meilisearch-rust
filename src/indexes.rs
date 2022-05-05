use crate::{client::Client, errors::Error, request::*, search::*, tasks::*, Rc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, fmt::Display, time::Duration};
use time::OffsetDateTime;

/// An index containing [Document]s.
///
/// # Example
///
/// You can create an index remotly and, if that succeed, make an `Index` out of it.
/// See the [Client::create_index] method.
/// ```
/// # use meilisearch_sdk::{client::*, indexes::*};
/// # futures::executor::block_on(async move {
/// let client = Client::new("http://localhost:7700", "masterKey");
///
/// // get the index called movies or create it if it does not exist
/// let movies = client
///   .create_index("index", None)
///   .await
///   .unwrap()
///   // We wait for the task to execute until completion
///   .wait_for_completion(&client, None, None)
///   .await
///   .unwrap()
///   // Once the task finished, we try to create an `Index` out of it
///   .try_make_index(&client)
///   .unwrap();
///
/// assert_eq!(movies.as_ref(), "index");
/// # movies.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
/// # });
/// ```
///
/// Or, if you know the index already exist remotely you can create an `Index` with the [Client::index] function.
/// ```
/// # use meilisearch_sdk::{client::*, indexes::*};
/// # futures::executor::block_on(async move {
/// let client = Client::new("http://localhost:7700", "masterKey");
///
/// // use the implicit index creation if the index already exist or
/// // Meilisearch would be able to create the index if it does not exist during:
/// // - the documents addition (add and update routes)
/// // - the settings update
/// let movies = client.index("index");
///
/// // do something with the index
/// # });
/// ```
#[derive(Debug, Clone)]
pub struct Index {
    pub(crate) uid: Rc<String>,
    pub(crate) client: Client,
    pub(crate) primary_key: Option<String>,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}

impl Index {
    /// Internal Function to create an [Index] from `serde_json::Value` and [Client]
    pub(crate) fn from_value(v: serde_json::Value, client: Client) -> Result<Index, Error> {
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

        let i: IndexFromSerde = serde_json::from_value(v).map_err(Error::ParseError)?;

        Ok(Index {
            uid: Rc::new(i.uid),
            client,
            created_at: i.createdAt,
            updated_at: i.updatedAt,
            primary_key: i.primaryKey,
        })
    }

    /// Set the primary key of the index.
    ///
    /// If you prefer, you can use the method [Index::set_primary_key], which is an alias.
    pub async fn update(&self, primary_key: impl AsRef<str>) -> Result<(), Error> {
        request::<serde_json::Value, serde_json::Value>(
            &format!("{}/indexes/{}", self.client.host, self.uid),
            &self.client.api_key,
            Method::Put(json!({ "primaryKey": primary_key.as_ref() })),
            200,
        )
        .await?;
        Ok(())
    }

    /// Delete the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// # let index = client.create_index("delete", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap().try_make_index(&client).unwrap();
    ///
    /// // get the index named "movies" and delete it
    /// let index = client.index("delete");
    /// let task = index.delete().await.unwrap();
    /// client.wait_for_task(task, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn delete(self) -> Result<Task, Error> {
        request::<(), Task>(
            &format!("{}/indexes/{}", self.client.host, self.uid),
            &self.client.api_key,
            Method::Delete,
            202,
        )
        .await
    }

    /// Search for documents matching a specific query in the index.\
    /// See also [Index::search].
    ///
    /// # Example
    ///
    /// ```
    /// use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::{client::*, indexes::*, search::*};
    ///
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct Movie {
    ///     name: String,
    ///     description: String,
    /// }
    ///
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let movies = client.index("execute_query");
    ///
    /// // add some documents
    /// # movies.add_or_replace(&[Movie{name:String::from("Interstellar"), description:String::from("Interstellar chronicles the adventures of a group of explorers who make use of a newly discovered wormhole to surpass the limitations on human space travel and conquer the vast distances involved in an interstellar voyage.")},Movie{name:String::from("Unknown"), description:String::from("Unknown")}], Some("name")).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    ///
    /// let query = Query::new(&movies).with_query("Interstellar").with_limit(5).build();
    /// let results = movies.execute_query::<Movie>(&query).await.unwrap();
    /// assert!(results.hits.len()>0);
    /// # movies.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn execute_query<T: 'static + DeserializeOwned>(
        &self,
        query: &Query<'_>,
    ) -> Result<SearchResults<T>, Error> {
        request::<&Query, SearchResults<T>>(
            &format!("{}/indexes/{}/search", self.client.host, self.uid),
            &self.client.api_key,
            Method::Post(query),
            200,
        )
        .await
    }

    /// Search for documents matching a specific query in the index.\
    /// See also [Index::execute_query].
    ///
    /// # Example
    ///
    /// ```
    /// use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::{client::*, indexes::*, search::*};
    ///
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct Movie {
    ///     name: String,
    ///     description: String,
    /// }
    ///
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movies = client.index("search");
    ///
    /// // add some documents
    /// # movies.add_or_replace(&[Movie{name:String::from("Interstellar"), description:String::from("Interstellar chronicles the adventures of a group of explorers who make use of a newly discovered wormhole to surpass the limitations on human space travel and conquer the vast distances involved in an interstellar voyage.")},Movie{name:String::from("Unknown"), description:String::from("Unknown")}], Some("name")).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    ///
    /// let results = movies.search()
    ///     .with_query("Interstellar")
    ///     .with_limit(5)
    ///     .execute::<Movie>()
    ///     .await
    ///     .unwrap();
    ///
    /// assert!(results.hits.len()>0);
    /// # movies.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub fn search(&self) -> Query {
        Query::new(self)
    }

    /// Get one [Document] using its unique id.
    /// Serde is needed. Add `serde = {version="1.0", features=["derive"]}` in the dependencies section of your Cargo.toml.
    ///
    /// # Example
    ///
    /// ```
    /// use serde::{Serialize, Deserialize};
    ///
    /// # use meilisearch_sdk::{client::*, indexes::*};
    ///
    /// #[derive(Serialize, Deserialize, Debug)]
    /// # #[derive(PartialEq)]
    /// struct Movie {
    ///    name: String,
    ///    description: String,
    /// }
    ///
    ///
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let movies = client.index("get_document");
    /// # movies.add_or_replace(&[Movie{name:String::from("Interstellar"), description:String::from("Interstellar chronicles the adventures of a group of explorers who make use of a newly discovered wormhole to surpass the limitations on human space travel and conquer the vast distances involved in an interstellar voyage.")}], Some("name")).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    ///
    /// // retrieve a document (you have to put the document in the index before)
    /// let interstellar = movies.get_document::<Movie>("Interstellar").await.unwrap();
    ///
    /// assert_eq!(interstellar, Movie {
    ///     name: String::from("Interstellar"),
    ///     description: String::from("Interstellar chronicles the adventures of a group of explorers who make use of a newly discovered wormhole to surpass the limitations on human space travel and conquer the vast distances involved in an interstellar voyage.")
    /// });
    /// # movies.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_document<T: 'static + DeserializeOwned>(&self, uid: &str) -> Result<T, Error> {
        request::<(), T>(
            &format!(
                "{}/indexes/{}/documents/{}",
                self.client.host, self.uid, uid
            ),
            &self.client.api_key,
            Method::Get,
            200,
        )
        .await
    }

    /// Get [Document]s by batch.
    ///
    /// Using the optional parameters offset and limit, you can browse through all your documents.
    /// If None, offset will be set to 0, limit to 20, and all attributes will be retrieved.
    ///
    /// *Note: Documents are ordered by Meilisearch depending on the hash of their id.*
    ///
    /// # Example
    ///
    /// ```
    /// use serde::{Serialize, Deserialize};
    ///
    /// # use meilisearch_sdk::{client::*, indexes::*};
    ///
    /// #[derive(Serialize, Deserialize, Debug)]
    /// # #[derive(PartialEq)]
    /// struct Movie {
    ///    name: String,
    ///    description: String,
    /// }
    ///
    ///
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let movie_index = client.index("get_documents");
    ///
    /// # movie_index.add_or_replace(&[Movie{name:String::from("Interstellar"), description:String::from("Interstellar chronicles the adventures of a group of explorers who make use of a newly discovered wormhole to surpass the limitations on human space travel and conquer the vast distances involved in an interstellar voyage.")}], Some("name")).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    ///
    /// // retrieve movies (you have to put some movies in the index before)
    /// let movies = movie_index.get_documents::<Movie>(None, None, None).await.unwrap();
    ///
    /// assert!(movies.len() > 0);
    /// # movie_index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_documents<T: DeserializeOwned + 'static>(
        &self,
        offset: Option<usize>,
        limit: Option<usize>,
        attributes_to_retrieve: Option<&str>,
    ) -> Result<Vec<T>, Error> {
        let mut url = format!("{}/indexes/{}/documents?", self.client.host, self.uid);
        if let Some(offset) = offset {
            url.push_str("offset=");
            url.push_str(offset.to_string().as_str());
            url.push('&');
        }
        if let Some(limit) = limit {
            url.push_str("limit=");
            url.push_str(limit.to_string().as_str());
            url.push('&');
        }
        if let Some(attributes_to_retrieve) = attributes_to_retrieve {
            url.push_str("attributesToRetrieve=");
            url.push_str(attributes_to_retrieve);
        }
        request::<(), Vec<T>>(&url, &self.client.api_key, Method::Get, 200).await
    }

    /// Add a list of [Document]s or replace them if they already exist.
    ///
    /// If you send an already existing document (same id) the **whole existing document** will be overwritten by the new document.
    /// Fields previously in the document not present in the new document are removed.
    ///
    /// For a partial update of the document see [Index::add_or_update].
    ///
    /// You can use the alias [Index::add_documents] if you prefer.
    ///
    /// # Example
    ///
    /// ```
    /// use serde::{Serialize, Deserialize};
    ///
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// # use std::thread::sleep;
    /// # use std::time::Duration;
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct Movie {
    ///    name: String,
    ///    description: String,
    /// }
    ///
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
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
    /// let movies = movie_index.get_documents::<Movie>(None, None, None).await.unwrap();
    /// assert!(movies.len() >= 3);
    /// # movie_index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn add_or_replace<T: Serialize>(
        &self,
        documents: &[T],
        primary_key: Option<&str>,
    ) -> Result<Task, Error> {
        let url = if let Some(primary_key) = primary_key {
            format!(
                "{}/indexes/{}/documents?primaryKey={}",
                self.client.host, self.uid, primary_key
            )
        } else {
            format!("{}/indexes/{}/documents", self.client.host, self.uid)
        };
        request::<&[T], Task>(&url, &self.client.api_key, Method::Post(documents), 202).await
    }

    /// Alias for [Index::add_or_replace].
    pub async fn add_documents<T: Serialize>(
        &self,
        documents: &[T],
        primary_key: Option<&str>,
    ) -> Result<Task, Error> {
        self.add_or_replace(documents, primary_key).await
    }

    /// Add a list of documents and update them if they already.
    ///
    /// If you send an already existing document (same id) the old document will be only partially updated according to the fields of the new document.
    /// Thus, any fields not present in the new document are kept and remained unchanged.
    ///
    /// To completely overwrite a document, check out the [Index::add_or_replace] documents method.
    ///
    /// # Example
    ///
    /// ```
    /// use serde::{Serialize, Deserialize};
    ///
    /// # use meilisearch_sdk::client::*;
    /// # use std::thread::sleep;
    /// # use std::time::Duration;
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct Movie {
    ///    name: String,
    ///    description: String,
    /// }
    ///
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
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
    /// let movies = movie_index.get_documents::<Movie>(None, None, None).await.unwrap();
    /// assert!(movies.len() >= 3);
    /// # movie_index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn add_or_update<T: Serialize>(
        &self,
        documents: &[T],
        primary_key: Option<impl AsRef<str>>,
    ) -> Result<Task, Error> {
        let url = if let Some(primary_key) = primary_key {
            format!(
                "{}/indexes/{}/documents?primaryKey={}",
                self.client.host,
                self.uid,
                primary_key.as_ref()
            )
        } else {
            format!("{}/indexes/{}/documents", self.client.host, self.uid)
        };
        request::<&[T], Task>(&url, &self.client.api_key, Method::Put(documents), 202).await
    }

    /// Delete all documents in the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # #[derive(Serialize, Deserialize, Debug)]
    /// # struct Movie {
    /// #    name: String,
    /// #    description: String,
    /// # }
    /// #
    /// #
    /// # futures::executor::block_on(async move {
    /// #
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let movie_index = client.index("delete_all_documents");
    ///
    /// # movie_index.add_or_replace(&[Movie{name:String::from("Interstellar"), description:String::from("Interstellar chronicles the adventures of a group of explorers who make use of a newly discovered wormhole to surpass the limitations on human space travel and conquer the vast distances involved in an interstellar voyage.")}], Some("name")).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// // add some documents
    ///
    /// movie_index.delete_all_documents()
    ///   .await
    ///   .unwrap()
    ///   .wait_for_completion(&client, None, None)
    ///   .await
    ///   .unwrap();
    /// let movies = movie_index.get_documents::<Movie>(None, None, None).await.unwrap();
    /// assert_eq!(movies.len(), 0);
    /// # movie_index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn delete_all_documents(&self) -> Result<Task, Error> {
        request::<(), Task>(
            &format!("{}/indexes/{}/documents", self.client.host, self.uid),
            &self.client.api_key,
            Method::Delete,
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
    /// # #[derive(Serialize, Deserialize, Debug)]
    /// # struct Movie {
    /// #    name: String,
    /// #    description: String,
    /// # }
    /// #
    /// #
    /// # futures::executor::block_on(async move {
    /// #
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movies = client.index("delete_document");
    ///
    /// # movies.add_or_replace(&[Movie{name:String::from("Interstellar"), description:String::from("Interstellar chronicles the adventures of a group of explorers who make use of a newly discovered wormhole to surpass the limitations on human space travel and conquer the vast distances involved in an interstellar voyage.")}], Some("name")).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// // add a document with id = Interstellar
    ///
    /// movies.delete_document("Interstellar")
    ///   .await
    ///   .unwrap()
    ///   .wait_for_completion(&client, None, None)
    ///   .await
    ///   .unwrap();
    /// # movies.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn delete_document<T: Display>(&self, uid: T) -> Result<Task, Error> {
        request::<(), Task>(
            &format!(
                "{}/indexes/{}/documents/{}",
                self.client.host, self.uid, uid
            ),
            &self.client.api_key,
            Method::Delete,
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
    /// # #[derive(Serialize, Deserialize, Debug)]
    /// # struct Movie {
    /// #    name: String,
    /// #    description: String,
    /// # }
    /// #
    /// #
    /// # futures::executor::block_on(async move {
    /// #
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let movies = client.index("delete_documents");
    ///
    /// // add some documents
    /// # movies.add_or_replace(&[Movie{name:String::from("Interstellar"), description:String::from("Interstellar chronicles the adventures of a group of explorers who make use of a newly discovered wormhole to surpass the limitations on human space travel and conquer the vast distances involved in an interstellar voyage.")},Movie{name:String::from("Unknown"), description:String::from("Unknown")}], Some("name")).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    ///
    /// // delete some documents
    /// movies.delete_documents(&["Interstellar", "Unknown"])
    ///   .await
    ///   .unwrap()
    ///   .wait_for_completion(&client, None, None)
    ///   .await
    ///   .unwrap();
    /// # movies.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn delete_documents<T: Display + Serialize + std::fmt::Debug>(
        &self,
        uids: &[T],
    ) -> Result<Task, Error> {
        request::<&[T], Task>(
            &format!(
                "{}/indexes/{}/documents/delete-batch",
                self.client.host, self.uid
            ),
            &self.client.api_key,
            Method::Post(uids),
            202,
        )
        .await
    }

    /// Alias for the [Index::update] method.
    pub async fn set_primary_key(&self, primary_key: impl AsRef<str>) -> Result<(), Error> {
        self.update(primary_key).await
    }

    /// Fetch the information of the index as a raw JSON [Index], this index should already exist.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    ///
    /// # futures::executor::block_on(async move {
    /// // create the client
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// # let index = client.create_index("fetch_info", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap().try_make_index(&client).unwrap();
    ///
    /// // get the information of the index named "fetch_info"
    /// let mut idx = client.index("fetch_info");
    /// idx.fetch_info().await.unwrap();
    /// println!("{idx:?}");
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    /// If you use it directly from the [Client], you can use the method [Client::get_raw_index], which is the equivalent method from the client.
    pub async fn fetch_info(&mut self) -> Result<(), Error> {
        let v = self.client.get_raw_index(self.uid.as_ref()).await?;
        *self = Index::from_value(v, self.client.clone())?;
        Ok(())
    }

    /// Fetch the primary key of the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    ///
    /// # futures::executor::block_on(async move {
    /// // create the client
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// # let index = client.create_index("get_primary_key", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap().try_make_index(&client).unwrap();
    ///
    /// // get the primary key of the index named "movies"
    /// let movies = client.index("movies").get_primary_key().await;
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_primary_key(&mut self) -> Result<Option<&str>, Error> {
        self.fetch_info().await?;
        Ok(self.primary_key.as_deref())
    }

    /// Get a [Task] from a specific [Index] to keep track of [asynchronous operations](https://docs.meilisearch.com/learn/advanced/asynchronous_operations.html).
    ///
    /// # Example
    ///
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use std::thread::sleep;
    /// # use std::time::Duration;
    /// # use meilisearch_sdk::{client::*, indexes::*, tasks::Task};
    /// #
    /// # #[derive(Debug, Serialize, Deserialize, PartialEq)]
    /// # struct Document {
    /// #    id: usize,
    /// #    value: String,
    /// #    kind: String,
    /// # }
    /// #
    /// #
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
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
    ///    Task::Enqueued { content } => content.uid,
    ///    Task::Processing { content } => content.uid,
    ///    Task::Failed { content } => content.task.uid,
    ///    Task::Succeeded { content } => content.uid,
    /// };
    ///
    /// assert_eq!(task.get_uid(), from_index);
    /// # movies.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_task(&self, uid: impl AsRef<u64>) -> Result<Task, Error> {
        request::<(), Task>(
            &format!(
                "{}/indexes/{}/tasks/{}",
                self.client.host,
                self.uid,
                uid.as_ref()
            ),
            &self.client.api_key,
            Method::Get,
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
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new("http://localhost:7700", "masterKey");
    /// # let index = client.create_index("get_tasks", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap().try_make_index(&client).unwrap();
    ///
    /// let status = index.get_tasks().await.unwrap();
    /// assert!(status.len() == 1); // the index was created
    ///
    /// index.set_ranking_rules(["wrong_ranking_rule"]).await.unwrap();
    ///
    /// let status = index.get_tasks().await.unwrap();
    /// assert!(status.len() == 2);
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_tasks(&self) -> Result<Vec<Task>, Error> {
        #[derive(Deserialize)]
        struct AllTasks {
            results: Vec<Task>,
        }

        Ok(request::<(), AllTasks>(
            &format!("{}/indexes/{}/tasks", self.client.host, self.uid),
            &self.client.api_key,
            Method::Get,
            200,
        )
        .await?
        .results)
    }

    /// Get stats of an index.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new("http://localhost:7700", "masterKey");
    /// # let index = client.create_index("get_stats", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap().try_make_index(&client).unwrap();
    ///
    /// let stats = index.get_stats().await.unwrap();
    /// assert_eq!(stats.is_indexing, false);
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_stats(&self) -> Result<IndexStats, Error> {
        request::<serde_json::Value, IndexStats>(
            &format!("{}/indexes/{}/stats", self.client.host, self.uid),
            &self.client.api_key,
            Method::Get,
            200,
        )
        .await
    }

    /// Wait until Meilisearch processes a [Task], and get its status.
    ///
    /// `interval` = The frequency at which the server should be polled. Default = 50ms
    /// `timeout` = The maximum time to wait for processing to complete. Default = 5000ms
    ///
    /// If the waited time exceeds `timeout` then an [Error::Timeout] will be returned.
    ///
    /// See also [Client::wait_for_task, Task::wait_for_completion].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, tasks::Task};
    /// # use serde::{Serialize, Deserialize};
    /// #
    /// # #[derive(Debug, Serialize, Deserialize, PartialEq)]
    /// # struct Document {
    /// #    id: usize,
    /// #    value: String,
    /// #    kind: String,
    /// # }
    /// #
    /// #
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
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
        task_id: impl AsRef<u64>,
        interval: Option<Duration>,
        timeout: Option<Duration>,
    ) -> Result<Task, Error> {
        self.client.wait_for_task(task_id, interval, timeout).await
    }

    /// Add documents to the index in batches
    ///
    /// `documents` = A slice of documents
    /// `batch_size` = Optional parameter that allows you to specify the size of the batch
    /// `batch_size` is 1000 by default
    ///
    /// # Example
    ///
    /// ```
    /// use serde::{Serialize, Deserialize};
    /// use meilisearch_sdk::client::*;
    ///
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct Movie {
    ///     name: String,
    ///     description: String,
    /// }
    ///
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
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
    /// let movies = movie_index.get_documents::<Movie>(None, None, None).await.unwrap();
    /// assert!(movies.len() >= 3);
    /// # movie_index.delete().await.unwrap().wait_for_completion(&client, None,
    /// None).await.unwrap();
    /// # });
    /// ```
    pub async fn add_documents_in_batches<T: Serialize>(
        &self,
        documents: &[T],
        batch_size: Option<usize>,
        primary_key: Option<&str>,
    ) -> Result<Vec<Task>, Error> {
        let mut task = Vec::with_capacity(documents.len());
        for document_batch in documents.chunks(batch_size.unwrap_or(1000)) {
            task.push(self.add_documents(document_batch, primary_key).await?);
        }
        Ok(task)
    }

    /// Update documents to the index in batches
    ///
    /// `documents` = A slice of documents
    /// `batch_size` = Optional parameter that allows you to specify the size of the batch
    /// `batch_size` is 1000 by default
    ///
    /// # Example
    ///
    /// ```
    /// use serde::{Serialize, Deserialize};
    /// use meilisearch_sdk::client::*;
    ///
    /// #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
    /// struct Movie {
    ///     name: String,
    ///     description: String,
    /// }
    ///
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
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
    /// let movies = movie_index.get_documents::<Movie>(None, None, None).await.unwrap();
    /// assert!(movies.len() >= 3);
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
    /// let movies_updated = movie_index.get_documents::<Movie>(None, None, None).await.unwrap();
    /// assert!(movies_updated.len() >= 3);
    ///
    /// assert!(&movies_updated[..] == &updated_movies[..]);
    ///
    /// # movie_index.delete().await.unwrap().wait_for_completion(&client, None,
    /// None).await.unwrap();
    /// # });
    /// ```
    pub async fn update_documents_in_batches<T: Serialize>(
        &self,
        documents: &[T],
        batch_size: Option<usize>,
        primary_key: Option<&str>,
    ) -> Result<Vec<Task>, Error> {
        let mut task = Vec::with_capacity(documents.len());
        for document_batch in documents.chunks(batch_size.unwrap_or(1000)) {
            task.push(self.add_or_update(document_batch, primary_key).await?);
        }
        Ok(task)
    }
}

impl AsRef<str> for Index {
    fn as_ref(&self) -> &str {
        &self.uid
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexStats {
    pub number_of_documents: usize,
    pub is_indexing: bool,
    pub field_distribution: HashMap<String, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    use meilisearch_test_macro::meilisearch_test;

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
            uid: Rc::new("test_from_value".to_string()),
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
    }

    #[meilisearch_test]
    async fn test_get_tasks_no_docs(index: Index) {
        // The at this point the only task that is supposed to exist is the creation of the index
        let status = index.get_tasks().await.unwrap();
        assert_eq!(status.len(), 1);
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
            Task::Enqueued { content } => assert_eq!(content.index_uid, *index.uid),
            Task::Processing { content } => assert_eq!(content.index_uid, *index.uid),
            Task::Failed { content } => assert_eq!(content.task.index_uid, *index.uid),
            Task::Succeeded { content } => assert_eq!(content.index_uid, *index.uid),
        }
        Ok(())
    }
}
