use crate::{errors::Error, indexes::Index, request::*};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Display;

pub trait Documentable: DeserializeOwned + std::fmt::Debug + Serialize {
    type UIDType: Display;

    fn get_uid(&self) -> &Self::UIDType;
}

#[derive(Debug)]
pub struct Document<'a, T: Documentable> {
    pub(crate) index: &'a Index<'a>,
    pub value: T,
}

impl<'a, T: Documentable> Document<'a, T> {
    pub fn delete(self) -> Result<(), Error> {
        Ok(request::<(), ()>(
            &format!(
                "{}/indexes/{}/documents/{}",
                self.index.client.host,
                self.index.uid,
                self.value.get_uid()
            ),
            self.index.client.apikey,
            Method::Delete,
            202,
        )?)
    }
}
