#[derive(Debug)]
/// Struct representing errors.
/// Unknow Errors are unexpected. You should consider panicking and open a GitHub issue (after ensuring you are using the supported version of the MeiliSearch server).
pub enum Error {
    /// There is no MeiliSearch server listening on the [specified host](../client/struct.Client.html#method.new).
    UnreachableServer,
    /// You tried to create an Index that already exists. You may want to use the [get_or_create method](../client/struct.Client.html#method.get_or_create).
    IndexAlreadyExist,
    /// You tried to get an Index that does not exist. You may want to use the [get_or_create method](../client/struct.Client.html#method.get_or_create).
    IndexNotFound,
    /// You tried to use an invalid UID for an Index. Index UID can only be composed of alphanumeric characters, hyphens (-), and underscores (_).
    InvalidIndexUid,
    /// You tried to add documents on an Index but MeiliSearch can't infer the primary key. Consider specifying the key.
    CantInferPrimaryKey,
    /// Server is in maintenance. You can set the maintenance state by using the `set_healthy` method of a Client.
    ServerInMaintenance,
    /// That's unexpected. Please open a GitHub issue after ensuring you are using the supported version of the MeiliSearch server.
    Unknown(String),
    /// The http client encountered an error.
    #[cfg(not(target_arch = "wasm32"))]
    Http(reqwest::Error),
    #[cfg(target_arch = "wasm32")]
    /// Never happens on wasm target.
    Http(())
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Error::UnreachableServer => write!(formatter, "Error::UnreachableServer: The MeiliSearch server can't be reached."),
            Error::IndexAlreadyExist => write!(formatter, "Error::IndexAlreadyExist: The creation of an index failed because it already exists."),
            Error::IndexNotFound => write!(formatter, "Error::IndexNotFound: The requested index does not exist."),
            Error::InvalidIndexUid => write!(formatter, "Error::InvalidIndexUid: The requested UID is invalid. Index UID can only be composed of alphanumeric characters, hyphens (-), and underscores (_)."),
            Error::CantInferPrimaryKey => write!(formatter, "Error::CantInferPrimaryKey: MeiliSearch was unable to infer the primary key of added documents."),
            Error::Http(error) => write!(formatter, "Error::Http: The http request failed: {:?}.", error),
            Error::ServerInMaintenance => write!(formatter, "Error::ServerInMaintenance: Server is in maintenance, please try again later."),
            Error::Unknown(message) => write!(formatter, "Error::Unknown: An unknown error occured. Please open an issue (https://github.com/Mubelotix/meilisearch-sdk/issues). Message: {:?}", message),
        }
    }
}

impl std::error::Error for Error {}

impl From<&serde_json::Value> for Error {
    fn from(message: &serde_json::Value) -> Error {
        // Error codes from https://github.com/meilisearch/MeiliSearch/blob/v0.12.0/meilisearch-error/src/lib.rs
        match message.get("errorCode").and_then(|v| v.as_str()) {
            Some("index_not_found") => Error::IndexNotFound,
            Some("index_already_exists") => Error::IndexAlreadyExist,
            Some("invalid_index_uid") => Error::InvalidIndexUid,
            Some("missing_primary_key") => Error::CantInferPrimaryKey,
            Some("maintenance") => Error::ServerInMaintenance,
            Some(_) => Error::Unknown(message.to_string()),
            None => {
                // Meilisearch 0.11 and below 
                match message.get("message").and_then(|v| v.as_str()) {
                    Some("Impossible to create index; index already exists") => Error::IndexAlreadyExist,
                    Some("Could not infer a primary key") => Error::CantInferPrimaryKey,
                    Some(m) if m.starts_with("Server is in maintenance, please try again later") => Error::ServerInMaintenance,
                    Some(m) if m.starts_with("Index ") && m.ends_with(" not found") => Error::IndexNotFound,
                    Some(m) if m.starts_with("Index must have a valid uid;") => Error::InvalidIndexUid,
                    _ => {
                        Error::Unknown(message.to_string())
                    },
                }
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        match error.status() {
            None => {
                Error::UnreachableServer
            }
            Some(_e) => Error::Http(error),
        }
    }
}

// TODO from http code https://docs.meilisearch.com/references/#error
