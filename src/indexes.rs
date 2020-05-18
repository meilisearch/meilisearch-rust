use crate::{
    client::Client, document::*, errors::Error, progress::*, request::*, search::*, settings::*,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;
use std::fmt::Display;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub(crate) struct JsonIndex {
    uid: String,
    primaryKey: Option<String>,
    createdAt: String,
    updatedAt: String,
}

impl JsonIndex {
    pub(crate) fn into_index<'a>(self, client: &'a Client) -> Index<'a> {
        Index {
            uid: self.uid,
            client,
        }
    }
}

/// An index containing [Documents](../document/trait.Document.html).
///
/// # Example
///
/// ```
/// # use meilisearch_sdk::{client::*, indexes::*};
/// let client = Client::new("http://localhost:7700", "");
///
/// // get the index called movies or create it if it does not exist
/// let movies = client.get_or_create("movies").unwrap();
///
/// // do something with the index
/// ```
#[derive(Debug)]
pub struct Index<'a> {
    pub(crate) uid: String,
    pub(crate) client: &'a Client<'a>,
}

impl<'a> Index<'a> {
    /// Set the primary key of the index.  
    ///   
    /// If you prefer, you can use the method [set_primary_key](#method.set_primary_key), which is an alias.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn update(&mut self, primary_key: &str) -> Result<(), Error> {
        request::<serde_json::Value, JsonIndex>(
            &format!("{}/indexes/{}", self.client.host, self.uid),
            self.client.apikey,
            Method::Put(json!({ "primaryKey": primary_key })),
            200,
        )?;
        Ok(())
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn update(&mut self, primary_key: &str) -> Result<(), Error> {
        request::<serde_json::Value, JsonIndex>(
            &format!("{}/indexes/{}", self.client.host, self.uid),
            self.client.apikey,
            Method::Put(json!({ "primaryKey": primary_key })),
            200,
        ).await?;
        Ok(())
    }

    /// Delete the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// let client = Client::new("http://localhost:7700", "");
    /// # client.create_index("movies", None);
    ///
    /// // get the index named "movies" and delete it
    /// let movies = client.get_index("movies").unwrap();
    /// movies.delete().unwrap();
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub fn delete(self) -> Result<(), Error> {
        Ok(request::<(), ()>(
            &format!("{}/indexes/{}", self.client.host, self.uid),
            self.client.apikey,
            Method::Delete,
            204,
        )?)
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn delete(self) -> Result<(), Error> {
        Ok(request::<(), ()>(
            &format!("{}/indexes/{}", self.client.host, self.uid),
            self.client.apikey,
            Method::Delete,
            204,
        ).await?)
    }

    /// Search for documents matching a specific query in the index.
    ///
    /// # Example
    ///
    /// ```
    /// use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, search::*};
    ///
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct Movie {
    ///     name: String,
    ///     description: String,
    /// }
    /// // that trait is used by the sdk when the primary key is needed
    /// impl Document for Movie {
    ///     type UIDType = String;
    ///     fn get_uid(&self) -> &Self::UIDType {
    ///         &self.name
    ///     }
    /// }
    ///
    /// let client = Client::new("http://localhost:7700", "");
    /// let mut movies = client.get_or_create("movies").unwrap();
    ///
    /// // add some documents
    /// # movies.add_or_replace(vec![Movie{name:String::from("Interstellar"), description:String::from("Interstellar chronicles the adventures of a group of explorers who make use of a newly discovered wormhole to surpass the limitations on human space travel and conquer the vast distances involved in an interstellar voyage.")},Movie{name:String::from("Unknown"), description:String::from("Unknown")}], Some("name")).unwrap();
    /// # std::thread::sleep(std::time::Duration::from_secs(1));
    ///
    /// let query = Query::new("Interstellar").with_limit(5);
    /// let results = movies.search::<Movie>(&query).unwrap();
    /// # assert!(results.hits.len()>0);
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub fn search<T: 'static + DeserializeOwned>(
        &self,
        query: &Query,
    ) -> Result<SearchResults<T>, Error> {
        Ok(request::<(), SearchResults<T>>(
            &format!(
                "{}/indexes/{}/search{}",
                self.client.host,
                self.uid,
                query.to_url()
            ),
            self.client.apikey,
            Method::Get,
            200,
        )?)
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn search<T: 'static + DeserializeOwned>(
        &self,
        query: &Query<'_>,
    ) -> Result<SearchResults<T>, Error> {
        Ok(request::<(), SearchResults<T>>(
            &format!(
                "{}/indexes/{}/search{}",
                self.client.host,
                self.uid,
                query.to_url()
            ),
            self.client.apikey,
            Method::Get,
            200,
        ).await?)
    }

    /// Get one [document](../document/trait.Document.html) using its unique id.  
    /// Serde is needed. Add `serde = {version="1.0", features=["derive"]}` in the dependencies section of your Cargo.toml.
    ///
    /// # Example
    ///
    /// ```
    /// use serde::{Serialize, Deserialize};
    ///
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*};
    /// let client = Client::new("http://localhost:7700", "");
    /// # client.create_index("movies", None);
    /// let movies = client.get_index("movies").unwrap();
    /// # let mut movies = client.get_index("movies").unwrap();
    ///
    /// #[derive(Serialize, Deserialize, Debug)]
    /// # #[derive(PartialEq)]
    /// struct Movie {
    ///    name: String,
    ///    description: String,
    /// }
    ///
    /// // that trait is used by the sdk when the primary key is needed
    /// impl Document for Movie {
    ///    type UIDType = String;
    ///    fn get_uid(&self) -> &Self::UIDType {
    ///        &self.name
    ///    }
    /// }
    ///
    /// # movies.add_or_replace(vec![Movie{name:String::from("Interstellar"), description:String::from("Interstellar chronicles the adventures of a group of explorers who make use of a newly discovered wormhole to surpass the limitations on human space travel and conquer the vast distances involved in an interstellar voyage.")}], Some("name")).unwrap();
    /// # std::thread::sleep(std::time::Duration::from_secs(1));
    /// #
    /// // retrieve a document (you have to put the document in the index before)
    /// let interstellar = movies.get_document::<Movie>(String::from("Interstellar")).unwrap();
    ///
    /// assert_eq!(interstellar, Movie{
    ///     name: String::from("Interstellar"),
    ///     description: String::from("Interstellar chronicles the adventures of a group of explorers who make use of a newly discovered wormhole to surpass the limitations on human space travel and conquer the vast distances involved in an interstellar voyage.")
    /// });
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_document<T: 'static + Document>(&self, uid: T::UIDType) -> Result<T, Error> {
        Ok(request::<(), T>(
            &format!(
                "{}/indexes/{}/documents/{}",
                self.client.host, self.uid, uid
            ),
            self.client.apikey,
            Method::Get,
            200,
        )?)
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get_document<T: 'static + Document>(&self, uid: T::UIDType) -> Result<T, Error> {
        Ok(request::<(), T>(
            &format!(
                "{}/indexes/{}/documents/{}",
                self.client.host, self.uid, uid
            ),
            self.client.apikey,
            Method::Get,
            200,
        ).await?)
    }

    /// Get [documents](../document/trait.Document.html) by batch.  
    ///   
    /// Using the optional parameters offset and limit, you can browse through all your documents.
    /// If None, offset will be set to 0, limit to 20, and all attributes will be retrieved.  
    ///   
    /// *Note: Documents are ordered by MeiliSearch depending on the hash of their id.*
    ///
    /// # Example
    ///
    /// ```
    /// use serde::{Serialize, Deserialize};
    ///
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*};
    /// let client = Client::new("http://localhost:7700", "");
    /// # client.create_index("movies", None);
    /// let movie_index = client.get_index("movies").unwrap();
    /// # let mut movie_index = client.get_index("movies").unwrap();
    ///
    /// #[derive(Serialize, Deserialize, Debug)]
    /// # #[derive(PartialEq)]
    /// struct Movie {
    ///    name: String,
    ///    description: String,
    /// }
    ///
    /// // that trait is used by the sdk when the primary key is needed
    /// impl Document for Movie {
    ///    type UIDType = String;
    ///    fn get_uid(&self) -> &Self::UIDType {
    ///        &self.name
    ///    }
    /// }
    ///
    /// # movie_index.add_or_replace(vec![Movie{name:String::from("Interstellar"), description:String::from("Interstellar chronicles the adventures of a group of explorers who make use of a newly discovered wormhole to surpass the limitations on human space travel and conquer the vast distances involved in an interstellar voyage.")}], Some("name")).unwrap();
    /// # std::thread::sleep(std::time::Duration::from_secs(1));
    /// #
    /// // retrieve movies (you have to put some movies in the index before)
    /// let movies = movie_index.get_documents::<Movie>(None, None, None).unwrap();
    ///
    /// assert!(movies.len() > 0);
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_documents<T: 'static + Document>(
        &self,
        offset: Option<usize>,
        limit: Option<usize>,
        attributes_to_retrieve: Option<&str>,
    ) -> Result<Vec<T>, Error> {
        let mut url = format!("{}/indexes/{}/documents?", self.client.host, self.uid);
        if let Some(offset) = offset {
            url.push_str("offset=");
            url.push_str(offset.to_string().as_str());
            url.push_str("&");
        }
        if let Some(limit) = limit {
            url.push_str("limit=");
            url.push_str(limit.to_string().as_str());
            url.push_str("&");
        }
        if let Some(attributes_to_retrieve) = attributes_to_retrieve {
            url.push_str("attributesToRetrieve=");
            url.push_str(attributes_to_retrieve.to_string().as_str());
        }
        Ok(request::<(), Vec<T>>(
            &url,
            self.client.apikey,
            Method::Get,
            200,
        )?)
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get_documents<T: 'static + Document>(
        &self,
        offset: Option<usize>,
        limit: Option<usize>,
        attributes_to_retrieve: Option<&str>,
    ) -> Result<Vec<T>, Error> {
        let mut url = format!("{}/indexes/{}/documents?", self.client.host, self.uid);
        if let Some(offset) = offset {
            url.push_str("offset=");
            url.push_str(offset.to_string().as_str());
            url.push_str("&");
        }
        if let Some(limit) = limit {
            url.push_str("limit=");
            url.push_str(limit.to_string().as_str());
            url.push_str("&");
        }
        if let Some(attributes_to_retrieve) = attributes_to_retrieve {
            url.push_str("attributesToRetrieve=");
            url.push_str(attributes_to_retrieve.to_string().as_str());
        }
        Ok(request::<(), Vec<T>>(
            &url,
            self.client.apikey,
            Method::Get,
            200,
        ).await?)
    }

    /// Add a list of [documents](../document/trait.Document.html) or replace them if they already exist.  
    ///   
    /// If you send an already existing document (same id) the **whole existing document** will be overwritten by the new document.
    /// Fields previously in the document not present in the new document are removed.  
    ///   
    /// For a partial update of the document see [add_or_update](#method.add_or_update).
    ///
    /// You can use the alias [add_documents](#method.add_documents) if you prefer.
    ///
    /// # Example
    ///
    /// ```
    /// use serde::{Serialize, Deserialize};
    ///
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*};
    /// # use std::thread::sleep;
    /// # use std::time::Duration;
    /// let client = Client::new("http://localhost:7700", "");
    /// let mut movie_index = client.get_or_create("movies").unwrap();
    ///
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct Movie {
    ///    name: String,
    ///    description: String,
    /// }
    /// // that trait is used by the sdk when the primary key is needed
    /// impl Document for Movie {
    ///    type UIDType = String;
    ///    fn get_uid(&self) -> &Self::UIDType {
    ///        &self.name
    ///    }
    /// }
    ///
    /// movie_index.add_or_replace(vec![
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
    /// ], Some("name")).unwrap();
    /// sleep(Duration::from_secs(1)); // MeiliSearch may take some time to execute the request
    ///
    /// // retrieve movies (you have to put some movies in the index before)
    /// let movies = movie_index.get_documents::<Movie>(None, None, None).unwrap();
    /// assert!(movies.len() >= 3);
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub fn add_or_replace<T: Document>(
        &mut self,
        documents: Vec<T>,
        primary_key: Option<&str>,
    ) -> Result<Progress, Error> {
        let url = if let Some(primary_key) = primary_key {
            format!(
                "{}/indexes/{}/documents?primaryKey={}",
                self.client.host, self.uid, primary_key
            )
        } else {
            format!("{}/indexes/{}/documents", self.client.host, self.uid)
        };
        Ok(
            request::<Vec<T>, ProgressJson>(
                &url,
                self.client.apikey,
                Method::Post(documents),
                202,
            )?
            .into_progress(self),
        )
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn add_or_replace<T: Document>(
        &'a mut self,
        documents: Vec<T>,
        primary_key: Option<&str>,
    ) -> Result<Progress<'a>, Error> {
        let url = if let Some(primary_key) = primary_key {
            format!(
                "{}/indexes/{}/documents?primaryKey={}",
                self.client.host, self.uid, primary_key
            )
        } else {
            format!("{}/indexes/{}/documents", self.client.host, self.uid)
        };
        Ok(
            request::<Vec<T>, ProgressJson>(
                &url,
                self.client.apikey,
                Method::Post(documents),
                202,
            ).await?
            .into_progress(self),
        )
    }

    /// Alias for [add_or_replace](#method.add_or_replace).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn add_documents<T: Document>(
        &mut self,
        documents: Vec<T>,
        primary_key: Option<&str>,
    ) -> Result<Progress, Error> {
        self.add_or_replace(documents, primary_key)
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn add_documents<T: Document>(
        &'a mut self,
        documents: Vec<T>,
        primary_key: Option<&str>,
    ) -> Result<Progress<'a>, Error> {
        self.add_or_replace(documents, primary_key).await
    }

    /// Add a list of documents and update them if they already.  
    ///   
    /// If you send an already existing document (same id) the old document will be only partially updated according to the fields of the new document.
    /// Thus, any fields not present in the new document are kept and remained unchanged.  
    ///   
    /// To completely overwrite a document, check out the [add_and_replace documents](#method.add_or_replace) method.
    ///
    /// # Example
    ///
    /// ```
    /// use serde::{Serialize, Deserialize};
    ///
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*};
    /// # use std::thread::sleep;
    /// # use std::time::Duration;
    /// let client = Client::new("http://localhost:7700", "");
    /// let mut movie_index = client.get_or_create("movies").unwrap();
    ///
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct Movie {
    ///    name: String,
    ///    description: String,
    /// }
    /// // that trait is used by the sdk when the primary key is needed
    /// impl Document for Movie {
    ///    type UIDType = String;
    ///    fn get_uid(&self) -> &Self::UIDType {
    ///        &self.name
    ///    }
    /// }
    ///
    /// movie_index.add_or_update(vec![
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
    /// ], Some("name")).unwrap();
    /// sleep(Duration::from_secs(1)); // MeiliSearch may take some time to execute the request
    ///
    /// // retrieve movies (you have to put some movies in the index before)
    /// let movies = movie_index.get_documents::<Movie>(None, None, None).unwrap();
    /// assert!(movies.len() >= 3);
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub fn add_or_update<T: Document>(
        &mut self,
        documents: Vec<T>,
        primary_key: Option<&str>,
    ) -> Result<Progress, Error> {
        let url = if let Some(primary_key) = primary_key {
            format!(
                "{}/indexes/{}/documents?primaryKey={}",
                self.client.host, self.uid, primary_key
            )
        } else {
            format!("{}/indexes/{}/documents", self.client.host, self.uid)
        };
        Ok(
            request::<Vec<T>, ProgressJson>(&url, self.client.apikey, Method::Put(documents), 202)?
                .into_progress(self),
        )
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn add_or_update<T: Document>(
        &'a mut self,
        documents: Vec<T>,
        primary_key: Option<&str>,
    ) -> Result<Progress<'a>, Error> {
        let url = if let Some(primary_key) = primary_key {
            format!(
                "{}/indexes/{}/documents?primaryKey={}",
                self.client.host, self.uid, primary_key
            )
        } else {
            format!("{}/indexes/{}/documents", self.client.host, self.uid)
        };
        Ok(
            request::<Vec<T>, ProgressJson>(&url, self.client.apikey, Method::Put(documents), 202).await?
                .into_progress(self),
        )
    }

    /// Delete all documents in the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*};
    /// #
    /// # #[derive(Serialize, Deserialize, Debug)]
    /// # struct Movie {
    /// #    name: String,
    /// #    description: String,
    /// # }
    /// #
    /// # // that trait is used by the sdk when the primary key is needed
    /// # impl Document for Movie {
    /// #    type UIDType = String;
    /// #    fn get_uid(&self) -> &Self::UIDType {
    /// #        &self.name
    /// #    }
    /// # }
    /// #
    /// let client = Client::new("http://localhost:7700", "");
    /// let mut movie_index = client.get_or_create("movies").unwrap();
    ///
    /// // add some documents
    ///
    /// movie_index.delete_all_documents().unwrap();
    /// # let movies = movie_index.get_documents::<Movie>(None, None, None).unwrap();
    /// # assert_eq!(movies.len(), 0);
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub fn delete_all_documents(&mut self) -> Result<Progress, Error> {
        Ok(request::<(), ProgressJson>(
            &format!("{}/indexes/{}/documents", self.client.host, self.uid),
            self.client.apikey,
            Method::Delete,
            202,
        )?
        .into_progress(self))
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn delete_all_documents(&'a mut self) -> Result<Progress<'a>, Error> {
        Ok(request::<(), ProgressJson>(
            &format!("{}/indexes/{}/documents", self.client.host, self.uid),
            self.client.apikey,
            Method::Delete,
            202,
        ).await?
        .into_progress(self))
    }

    /// Delete one document based on its unique id.  
    ///
    /// # Example
    ///
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*};
    /// #
    /// # #[derive(Serialize, Deserialize, Debug)]
    /// # struct Movie {
    /// #    name: String,
    /// #    description: String,
    /// # }
    /// #
    /// # // that trait is used by the sdk when the primary key is needed
    /// # impl Document for Movie {
    /// #    type UIDType = String;
    /// #    fn get_uid(&self) -> &Self::UIDType {
    /// #        &self.name
    /// #    }
    /// # }
    /// #
    /// #
    /// let client = Client::new("http://localhost:7700", "");
    /// let mut movies = client.get_or_create("movies").unwrap();
    ///
    /// # movies.add_or_replace(vec![Movie{name:String::from("Interstellar"), description:String::from("Interstellar chronicles the adventures of a group of explorers who make use of a newly discovered wormhole to surpass the limitations on human space travel and conquer the vast distances involved in an interstellar voyage.")}], Some("name")).unwrap();
    /// # std::thread::sleep(std::time::Duration::from_secs(1));
    /// // add a document with id = Interstellar
    ///
    /// movies.delete_document("Interstellar").unwrap();
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub fn delete_document<T: Display>(&mut self, uid: T) -> Result<Progress, Error> {
        Ok(request::<(), ProgressJson>(
            &format!(
                "{}/indexes/{}/documents/{}",
                self.client.host, self.uid, uid
            ),
            self.client.apikey,
            Method::Delete,
            202,
        )?
        .into_progress(self))
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn delete_document<T: Display>(&'a mut self, uid: T) -> Result<Progress<'a>, Error> {
        Ok(request::<(), ProgressJson>(
            &format!(
                "{}/indexes/{}/documents/{}",
                self.client.host, self.uid, uid
            ),
            self.client.apikey,
            Method::Delete,
            202,
        ).await?
        .into_progress(self))
    }

    /// Delete a selection of documents based on array of document id's.  
    ///   
    /// # Example
    ///
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*};
    /// #
    /// # #[derive(Serialize, Deserialize, Debug)]
    /// # struct Movie {
    /// #    name: String,
    /// #    description: String,
    /// # }
    /// #
    /// # // that trait is used by the sdk when the primary key is needed
    /// # impl Document for Movie {
    /// #    type UIDType = String;
    /// #    fn get_uid(&self) -> &Self::UIDType {
    /// #        &self.name
    /// #    }
    /// # }
    /// #
    /// #
    /// let client = Client::new("http://localhost:7700", "");
    /// let mut movies = client.get_or_create("movies").unwrap();
    ///
    /// // add some documents
    /// # movies.add_or_replace(vec![Movie{name:String::from("Interstellar"), description:String::from("Interstellar chronicles the adventures of a group of explorers who make use of a newly discovered wormhole to surpass the limitations on human space travel and conquer the vast distances involved in an interstellar voyage.")},Movie{name:String::from("Unknown"), description:String::from("Unknown")}], Some("name")).unwrap();
    /// # std::thread::sleep(std::time::Duration::from_secs(1));
    ///
    /// // delete some documents
    /// movies.delete_documents(vec!["Interstellar", "Unknown"]).unwrap();
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub fn delete_documents<T: Display + Serialize + std::fmt::Debug>(
        &mut self,
        uids: Vec<T>,
    ) -> Result<Progress, Error> {
        Ok(request::<Vec<T>, ProgressJson>(
            &format!(
                "{}/indexes/{}/documents/delete-batch",
                self.client.host, self.uid
            ),
            self.client.apikey,
            Method::Post(uids),
            202,
        )?
        .into_progress(self))
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn delete_documents<T: Display + Serialize + std::fmt::Debug>(
        &'a mut self,
        uids: Vec<T>,
    ) -> Result<Progress<'a>, Error> {
        Ok(request::<Vec<T>, ProgressJson>(
            &format!(
                "{}/indexes/{}/documents/delete-batch",
                self.client.host, self.uid
            ),
            self.client.apikey,
            Method::Post(uids),
            202,
        ).await?
        .into_progress(self))
    }

    /// Get the [settings](../settings/struct.Settings.html) of the Index.
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*};
    /// let client = Client::new("http://localhost:7700", "");
    /// let movie_index = client.get_or_create("movies").unwrap();
    /// let settings = movie_index.get_settings().unwrap();
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_settings(&self) -> Result<Settings, Error> {
        Ok(request::<(), Settings>(
            &format!("{}/indexes/{}/settings", self.client.host, self.uid),
            self.client.apikey,
            Method::Get,
            200,
        )?)
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn get_settings(&self) -> Result<Settings, Error> {
        Ok(request::<(), Settings>(
            &format!("{}/indexes/{}/settings", self.client.host, self.uid),
            self.client.apikey,
            Method::Get,
            200,
        ).await?)
    }

    /// Update the settings of the index.  
    /// Updates in the settings are partial. This means that any parameters corresponding to a None value will be left unchanged.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// let client = Client::new("http://localhost:7700", "");
    /// let mut movie_index = client.get_or_create("movies").unwrap();
    ///
    /// let stop_words = vec![String::from("a"), String::from("the"), String::from("of")];
    /// let settings = Settings::new()
    ///     .with_stop_words(stop_words.clone())
    ///     .with_accept_new_fields(false);
    ///
    /// let progress = movie_index.set_settings(&settings).unwrap();
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_settings(&mut self, settings: &Settings) -> Result<Progress, Error> {
        Ok(request::<&Settings, ProgressJson>(
            &format!("{}/indexes/{}/settings", self.client.host, self.uid),
            self.client.apikey,
            Method::Post(settings),
            202,
        )?
        .into_progress(self))
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn set_settings(&'a mut self, settings: &Settings) -> Result<Progress<'a>, Error> {
        Ok(request::<&Settings, ProgressJson>(
            &format!("{}/indexes/{}/settings", self.client.host, self.uid),
            self.client.apikey,
            Method::Post(settings),
            202,
        ).await?
        .into_progress(self))
    }

    /// Reset the settings of the index.  
    /// All settings will be reset to their [default value](https://docs.meilisearch.com/references/settings.html#reset-settings).
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// let client = Client::new("http://localhost:7700", "");
    /// let mut movie_index = client.get_or_create("movies").unwrap();
    ///
    /// let progress = movie_index.reset_settings().unwrap();
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub fn reset_settings(&mut self) -> Result<Progress, Error> {
        Ok(request::<(), ProgressJson>(
            &format!("{}/indexes/{}/settings", self.client.host, self.uid),
            self.client.apikey,
            Method::Delete,
            202,
        )?
        .into_progress(self))
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn reset_settings(&'a mut self) -> Result<Progress<'a>, Error> {
        Ok(request::<(), ProgressJson>(
            &format!("{}/indexes/{}/settings", self.client.host, self.uid),
            self.client.apikey,
            Method::Delete,
            202,
        ).await?
        .into_progress(self))
    }

    /// Alias for the [update method](#method.update).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_primary_key(&mut self, primary_key: &str) -> Result<(), Error> {
        self.update(primary_key)
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn set_primary_key(&mut self, primary_key: &str) -> Result<(), Error> {
        self.update(primary_key).await
    }
}
