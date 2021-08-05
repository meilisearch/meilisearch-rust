use serde::{de::DeserializeOwned, Serialize, Deserialize};
use std::fmt::Display;

/// Documents are not a predefined structure.
/// You can use your structs as documents by implementing that trait.
///
/// **WARNING**! The get_uid() method **MUST** only return an object that displays himself only using alphanumeric characters, '/' and '-'.
/// Otherwise, the MeiliSearch server will reject your documents.
///
/// *To be able to use derive with serde, put this line on your Cargo.toml: `serde = {version="1.0", features=["derive"]}`.*
///
/// # Example
///
/// ```
/// # use meilisearch_sdk::document::Document;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Serialize, Deserialize, Debug)]
/// struct Movie {
///     id: usize,
///     name: String,
///     description: String,
/// }
///
/// impl Document for Movie {
///     type UIDType = usize;
///
///     fn get_uid(&self) -> &Self::UIDType {
///         &self.id
///     }
/// }
/// ```
pub trait Document: DeserializeOwned + std::fmt::Debug + Serialize {
    /// The type of the primary key
    type UIDType: Display;

    /// The method returning the primary key of the Document.
    ///
    /// **WARNING**! This method **MUST** only return an object that displays himself only using alphanumeric characters, '/' and '-'.
    /// Otherwise, the MeiliSearch server will reject your document.
    fn get_uid(&self) -> &Self::UIDType;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnknownDocument {
    #[serde(flatten)]
    pub value: serde_json::Value,
}

impl Document for UnknownDocument {
    type UIDType = &'static str;

    fn get_uid(&self) -> &Self::UIDType {
        panic!("UID cannot be inferred on unknown documents")
    }
}
