use crate::{client::Client, documents::*, errors::Error, request::*, progress::Progress};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
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

#[derive(Debug)]
pub struct Index<'a> {
    pub(crate) uid: String,
    pub(crate) client: &'a Client<'a>,
}

impl<'a> Index<'a> {
    pub fn update(&mut self, primary_key: Option<&str>) {
        unimplemented!();
    }

    pub fn delete(self) -> Result<(), Error> {
        Ok(request::<(), ()>(
            &format!("{}/indexes/{}", self.client.host, self.uid),
            self.client.apikey,
            Method::Delete,
            204,
        )?)
    }

    pub fn get_document<T: DeserializeOwned, U: Display>(&self, uid: U) -> Result<Document<T, U>, Error> {
        let value: T = request::<(), T>(
            &format!("{}/indexes/{}/documents/{}", self.client.host, self.uid, uid),
            self.client.apikey,
            Method::Get,
            200,
        )?;
        Ok(Document {
            index: &self,
            value,
            uid
        })
    }

    pub fn get_documents<T: DeserializeOwned>(
        &self,
        offset: Option<usize>,
        limit: Option<usize>,
        attributes_to_retrive: Option<Vec<&str>>,
    ) -> Result<Vec<Document<T, String>>, Error> {
        let values: Vec<T> = request::<(), Vec<T>>(
            &format!("{}/indexes/{}/documents", self.client.host, self.uid),
            self.client.apikey,
            Method::Get,
            200,
        )?;
        let mut documents = Vec::new();
        for value in values {
            documents.push(Document {
                index: &self,
                value,
                uid: String::new(),
            })
        }
        Ok(documents)
    }

    pub fn add_or_replace<T: Serialize + std::fmt::Debug>(
        &mut self,
        documents: Vec<T>,
        primary_key: Option<&str>,
    ) -> Result<Progress, Error> {
        Ok(request::<Vec<T>, Progress>(
            &format!("{}/indexes/{}/documents", self.client.host, self.uid),
            self.client.apikey,
            Method::Post(documents),
            202,
        )?)
    }

    pub fn add_or_update<T: Serialize + std::fmt::Debug>(
        &mut self,
        documents: Vec<T>,
        primary_key: Option<&str>,
    ) -> Result<Progress, Error> {
        Ok(request::<Vec<T>, Progress>(
            &format!("{}/indexes/{}/documents", self.client.host, self.uid),
            self.client.apikey,
            Method::Put(documents),
            202,
        )?)
    }

    pub fn delete_all_documents(&mut self) -> Result<(), Error> {
        Ok(request::<(), ()>(
            &format!("{}/indexes/{}/documents", self.client.host, self.uid),
            self.client.apikey,
            Method::Delete,
            202,
        )?)
    }

    /*pub fn delete_document<U: Display, T: Documentable<U>>(&mut self, document: Document<U, T>) -> Result<(), Error> {
        Ok(request::<(), ()>(
            &format!(
                "{}/indexes/{}/documents/{}",
                self.client.host, self.uid, document.get_uid()
            ),
            self.client.apikey,
            Method::Delete,
            202,
        )?)
    }*/

    
}
