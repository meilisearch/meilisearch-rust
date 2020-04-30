use crate::{errors::*, indexes::*, request::*};
use serde_json::{json, Value};

#[derive(Debug)]
pub struct Client<'a> {
    pub(crate) host: &'a str,
    pub(crate) apikey: &'a str,
}

impl<'a> Client<'a> {
    pub fn new(host: &'a str, apikey: &'a str) -> Client<'a> {
        Client { host, apikey }
    }

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

    pub fn get_index(&'a self, uid: &'a str) -> Result<Index<'a>, Error> {
        Ok(request::<(), JsonIndex>(
            &format!("{}/indexes/{}", self.host, uid),
            self.apikey,
            Method::Get,
            200,
        )?
        .into_index(self))
    }

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
}
