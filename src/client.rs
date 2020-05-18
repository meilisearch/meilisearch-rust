use crate::{errors::*, indexes::*, request::*};
use serde_json::{json, Value};

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
    pub fn new(host: &'a str, apikey: &'a str) -> Client<'a> {
        Client { host, apikey }
    }

    /// List all [indexes](../indexes/struct.Index.html).
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// // create the client
    /// let client = Client::new("http://localhost:7700", "");
    ///
    /// let indexes: Vec<Index> = client.list_all_indexes().unwrap();
    /// println!("{:?}", indexes);
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub fn list_all_indexes(&'a self) -> Result<Vec<Index<'a>>, Error> {
        let json_indexes = request::<(), Vec<JsonIndex>>(
            &format!("{}/indexes", self.host),
            self.apikey,
            Method::Get,
            200,
        )?;

        let mut indexes = Vec::new();
        for json_index in json_indexes {
            indexes.push(json_index.into_index(self))
        }

        Ok(indexes)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn list_all_indexes(&'static self, callback: Box<dyn Fn(Result<Vec<Index>, Error>)>) {
        request::<(), Vec<JsonIndex>>(
            &format!("{}/indexes", self.host),
            self.apikey,
            Method::Get,
            200,
            Box::new(move |value: Result<Vec<JsonIndex>, Error>| match value {
                Ok(json_indexes) => {
                    let mut indexes = Vec::new();
                    for json_index in json_indexes {
                        indexes.push(json_index.into_index(self))
                    }
                    callback(Ok(indexes));
                }
                Err(e) => callback(Err(e)),
            }),
        );
    }

    /// Get an [index](../indexes/struct.Index.html).
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// # let client = Client::new("http://localhost:7700", "");
    /// # client.create_index("movies", None);
    /// // create the client
    /// let client = Client::new("http://localhost:7700", "");
    ///
    /// // get the index named "movies"
    /// let movies = client.get_index("movies").unwrap();
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_index(&'a self, uid: &'a str) -> Result<Index<'a>, Error> {
        Ok(request::<(), JsonIndex>(
            &format!("{}/indexes/{}", self.host, uid),
            self.apikey,
            Method::Get,
            200,
        )?
        .into_index(self))
    }

    #[cfg(target_arch = "wasm32")]
    pub fn get_index(&'static self, uid: &'a str, callback: Box<dyn Fn(Result<Index, Error>)>) {
        request::<(), JsonIndex>(
            &format!("{}/indexes/{}", self.host, uid),
            self.apikey,
            Method::Get,
            200,
            Box::new(move |value: Result<JsonIndex, Error>| match value {
                Ok(value) => callback(Ok(value.into_index(&self))),
                Err(e) => callback(Err(e)),
            }),
        );
    }

    /// Create an [index](../indexes/struct.Index.html).
    /// The second parameter will be used as the primary key of the new index. If it is not specified, MeiliSearch will **try** to infer the primary key.
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// // create the client
    /// let client = Client::new("http://localhost:7700", "");
    /// # if let Ok(mut movies) = client.get_index("movies") {
    /// #   movies.delete();
    /// # }
    ///
    /// // create a new index called movies and access it
    /// let movies = client.create_index("movies", None);
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub fn create_index(
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
        )?
        .into_index(self))
    }

    #[cfg(target_arch = "wasm32")]
    pub fn create_index(
        &'static self,
        uid: &'a str,
        primary_key: Option<&str>,
        callback: Box<dyn Fn(Result<Index, Error>)>,
    ) {
        request::<Value, JsonIndex>(
            &format!("{}/indexes", self.host),
            self.apikey,
            Method::Post(json!({
                "uid": uid,
                "primaryKey": primary_key,
            })),
            201,
            Box::new(move |value: Result<JsonIndex, Error>| match value {
                Ok(value) => callback(Ok(value.into_index(&self))),
                Err(e) => callback(Err(e)),
            }),
        );
    }

    /// Delete an index from its UID.  
    /// To delete an index from the [index object](../indexes/struct.Index.html), use [the delete method](../indexes/struct.Index.html#method.delete).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn delete_index(&self, uid: &str) -> Result<(), Error> {
        Ok(request::<(), ()>(
            &format!("{}/indexes/{}", self.host, uid),
            self.apikey,
            Method::Delete,
            204,
        )?)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn delete_index(&self, uid: &str, callback: Box<dyn Fn(Result<(), Error>)>) {
        request::<(), ()>(
            &format!("{}/indexes/{}", self.host, uid),
            self.apikey,
            Method::Delete,
            204,
            callback,
        );
    }

    /// This will try to get an index and create the index if it does not exist.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_or_create(&'a self, uid: &'a str) -> Result<Index<'a>, Error> {
        if let Ok(index) = self.get_index(uid) {
            Ok(index)
        } else {
            self.create_index(uid, None)
        }
    }

    // TODO implement get_or_create on wasm

    /// Alias for [list_all_indexes](#method.list_all_indexes).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_indexes(&'a self) -> Result<Vec<Index<'a>>, Error> {
        self.list_all_indexes()
    }

    #[cfg(target_arch = "wasm32")]
    pub fn get_indexes(&'static self, callback: Box<dyn Fn(Result<Vec<Index>, Error>)>) {
        self.list_all_indexes(callback)
    }
}
