use crate::{indexes::Index, errors::Error, request::*};
use serde::{Serialize, de::DeserializeOwned};
use std::fmt::Display;

pub struct Document<'a, T, U: Display> {
    pub(crate) index: &'a Index<'a>,
    pub value: T,
    pub uid: U,
}

impl<'a, T, U: Display> Document<'a, T, U> {
    pub fn delete(self) -> Result<(), Error> {
        Ok(request::<(), ()>(
            &format!("{}/indexes/{}/documents/{}", self.index.client.host, self.index.uid, self.uid),
            self.index.client.apikey,
            Method::Delete,
            202,
        )?)
    }
}
