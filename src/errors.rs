#[derive(Debug, PartialEq)]
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
    /// That's unexpected. Please open a GitHub issue after ensuring you are using the supported version of the MeiliSearch server.
    Unknown(String),
}

impl From<&str> for Error {
    fn from(message: &str) -> Error {
        match message {
            "{\"message\":\"Impossible to create index; index already exists\"}" => Error::IndexAlreadyExist,
            "{\"message\":\"Index must have a valid uid; Index uid can be of type integer or string only composed of alphanumeric characters, hyphens (-) and underscores (_).\"}" => Error::InvalidIndexUid,
            "{\"message\":\"Could not infer a primary key\"}" => Error::CantInferPrimaryKey,
            m if m.starts_with("{\"message\":\"Index ") && m.ends_with(" not found\"}") => Error::IndexNotFound,
            e => {
                Error::Unknown(e.to_string())
            },
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<minreq::Error> for Error {
    fn from(error: minreq::Error) -> Error {
        match error {
            minreq::Error::IoError(e) if e.kind() == std::io::ErrorKind::ConnectionRefused => {
                Error::UnreachableServer
            }
            e => {
                Error::Unknown(format!("{:?}", e))
            },
        }
    }
}

// TODO from http code https://docs.meilisearch.com/references/#error