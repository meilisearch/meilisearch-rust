use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Display;

pub trait Document: DeserializeOwned + std::fmt::Debug + Serialize {
    type UIDType: Display;

    fn get_uid(&self) -> &Self::UIDType;
}