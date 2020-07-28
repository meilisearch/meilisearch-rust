use crate::{errors::*, indexes::*, request::*};
use serde_json::{json, Value};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// The top-level struct of the SDK, representing a client containing [indexes](../indexes/struct.Index.html).
#[derive(Debug)]
pub struct Client<'a> {
    pub(crate) host: &'a str,
    pub(crate) apikey: &'a str,
}

impl<'a> Client<'a> {
    /// Create a client using the specified server.
    /// Don't put a '/' at the end of the host.
    /// If you are not in production mode, the second field is useless.
    /// In production mode, see [the documentation](https://docs.meilisearch.com/references/keys.html) to get the needed key.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// // create the client
    /// let client = Client::new("http://localhost:7700", "");
    /// ```
    pub const fn new(host: &'a str, apikey: &'a str) -> Client<'a> {
        Client { host, apikey }
    }

    /// List all [indexes](../indexes/struct.Index.html).
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// # #[tokio::main]
    /// # async fn main() {
    /// // create the client
    /// let client = Client::new("http://localhost:7700", "");
    ///
    /// let indexes: Vec<Index> = client.list_all_indexes().await.unwrap();
    /// println!("{:?}", indexes);
    /// # }
    /// ```
    pub async fn list_all_indexes(&'a self) -> Result<Vec<Index<'a>>, Error> {
        let json_indexes = request::<(), Vec<JsonIndex>>(
            &format!("{}/indexes", self.host),
            self.apikey,
            Method::Get,
            200,
        ).await?;

        let mut indexes = Vec::new();
        for json_index in json_indexes {
            indexes.push(json_index.into_index(self))
        }

        Ok(indexes)
    }

    /// Get an [index](../indexes/struct.Index.html).
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// // create the client
    /// let client = Client::new("http://localhost:7700", "");
    /// # client.create_index("movies", None).await;
    ///
    /// // get the index named "movies"
    /// let movies = client.get_index("movies").await.unwrap();
    /// # }
    /// ```
    pub async fn get_index(&'a self, uid: &'a str) -> Result<Index<'a>, Error> {
        Ok(request::<(), JsonIndex>(
            &format!("{}/indexes/{}", self.host, uid),
            self.apikey,
            Method::Get,
            200,
        ).await?
        .into_index(self))
    }

    /// Assume that an [index](../indexes/struct.Index.html) exist and create a corresponding object without any check.
    pub fn assume_index(&'a self, uid: &'a str) -> Index<'a> {
        Index {
            client: &self,
            uid: uid.to_string()
        }
    }

    /// Create an [index](../indexes/struct.Index.html).
    /// The second parameter will be used as the primary key of the new index. If it is not specified, MeiliSearch will **try** to infer the primary key.
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # #[tokio::main]
    /// # async fn main() {
    /// // create the client
    /// let client = Client::new("http://localhost:7700", "");
    ///
    /// # if let Ok(mut movies) = client.get_index("movies").await {
    /// #   movies.delete().await.unwrap();
    /// # }
    /// // create a new index called movies and access it
    /// let movies = client.create_index("movies", None).await;
    /// # }
    /// ```
    pub async fn create_index(
        &'a self,
        uid: &'a str,
        primary_key: Option<&str>,
    ) -> Result<Index<'a>, Error> {
        Ok(request::<Value, JsonIndex>(
            &format!("{}/indexes", self.host),
            self.apikey,
            Method::Post(json!({
                "uid": uid,
                "primaryKey": primary_key,
            })),
            201,
        ).await?
        .into_index(self))
    }

    /// Delete an index from its UID.
    /// To delete an index from the [index object](../indexes/struct.Index.html), use [the delete method](../indexes/struct.Index.html#method.delete).
    pub async fn delete_index(&self, uid: &str) -> Result<(), Error> {
        Ok(request::<(), ()>(
            &format!("{}/indexes/{}", self.host, uid),
            self.apikey,
            Method::Delete,
            204,
        ).await?)
    }

    /// This will try to get an index and create the index if it does not exist.
    pub async fn get_or_create(&'a self, uid: &'a str) -> Result<Index<'a>, Error> {
        if let Ok(index) = self.get_index(uid).await {
            Ok(index)
        } else {
            self.create_index(uid, None).await
        }
    }

    /// Alias for [list_all_indexes](#method.list_all_indexes).
    pub async fn get_indexes(&'a self) -> Result<Vec<Index<'a>>, Error> {
        self.list_all_indexes().await
    }

    /// Get stats of all indexes.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "");
    /// let stats = client.get_stats().await.unwrap();
    /// # }
    /// ```
    pub async fn get_stats(&self) -> Result<ClientStats, Error> {
        request::<serde_json::Value, ClientStats>(
            &format!("{}/stats", self.host),
            self.apikey,
            Method::Get,
            200,
        ).await
    }

    /// Get health of MeiliSearch server.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, errors::Error};
    /// #
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "");
    ///
    /// match client.get_health().await {
    ///     Ok(()) => println!("server is operationnal"),
    ///     Err(Error::ServerInMaintenance) => eprintln!("server is in maintenance"),
    ///     _ => panic!("should never happen"),
    /// }
    /// # }
    /// ```
    pub async fn get_health(&self) -> Result<(), Error> {
        let r = request::<(), ()>(
            &format!("{}/health", self.host),
            self.apikey,
            Method::Get,
            204,
        ).await;
        match r {
            Err(Error::Unknown(m)) if &m == "null" => Ok(()),
            e => e
        }
    }

    /// Update health of MeiliSearch server.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "");
    ///
    /// client.set_health(false).await.unwrap();
    /// # client.set_health(true).await.unwrap();
    /// # }
    /// ```
    pub async fn set_health(&self, health: bool) -> Result<(), Error> {
        #[derive(Debug, Serialize)]
        struct HealthBody {
            health: bool
        }

        let r = request::<HealthBody, ()>(
            &format!("{}/health", self.host),
            self.apikey,
            Method::Put(HealthBody { health }),
            204,
        ).await;
        match r {
            Err(Error::Unknown(m)) if &m == "null" => Ok(()),
            e => e
        }
    }

    /// Get version of the MeiliSearch server.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, errors::Error};
    /// #
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "");
    /// let version = client.get_version().await.unwrap();
    /// # }
    /// ```
    pub async fn get_version(&self) -> Result<Version, Error> {
        request::<(), Version>(
            &format!("{}/version", self.host),
            self.apikey,
            Method::Get,
            200,
        ).await
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientStats {
    pub database_size: usize,
    pub last_update: Option<String>,
    pub indexes: HashMap<String, IndexStats>,
}

/// Version of a MeiliSearch server.
/// Example:
/// ```text
/// Version {
///    commit_sha: "b46889b5f0f2f8b91438a08a358ba8f05fc09fc1".to_string(),
///    build_date: "2019-11-15T09:51:54.278247+00:00".to_string(),
///    pkg_version: "0.1.1".to_string(),
/// }
/// ```
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub commit_sha: String,
    pub build_date: String,
    pub pkg_version: String,
}
