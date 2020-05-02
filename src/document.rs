use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Display;

/// Documents are not predefined structure.
/// You can use your own structs as documents by implementing that trait.  
///   
/// **WARNING**! The get_uid() method can only return a type that display himself only using alphanumeric caracters, '/' and '-'. 
/// Otherwise, the MeiliSearch server will reject your documents.  
///   
/// *To be able to use derive with serde, put this line on your Cargo.toml: `serde = {version="1.0", features=["derive"]}`.*
/// 
/// # Example
/// 
/// ```
/// use meilisearch_sdk::document::Document;
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
    type UIDType: Display;

    fn get_uid(&self) -> &Self::UIDType;
}