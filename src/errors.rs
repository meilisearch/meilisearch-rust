#[derive(Debug, PartialEq)]
pub enum Error {
    UnreachableServer,
    IndexAlreadyExist,
    IndexNotFound,
    InvalidUid,
    CantInferPrimaryKey,
    Unknown(String),
}

impl From<&str> for Error {
    fn from(message: &str) -> Error {
        match message {
            "{\"message\":\"Impossible to create index; index already exists\"}" => Error::IndexAlreadyExist,
            "{\"message\":\"Index must have a valid uid; Index uid can be of type integer or string only composed of alphanumeric characters, hyphens (-) and underscores (_).\"}" => Error::InvalidUid,
            "{\"message\":\"Could not infer a primary key\"}" => Error::CantInferPrimaryKey,
            m if m.starts_with("{\"message\":\"Index ") && m.ends_with(" not found\"}") => Error::IndexNotFound,
            e => Error::Unknown(e.to_string()),
        }
    }
}

impl From<minreq::Error> for Error {
    fn from(error: minreq::Error) -> Error {
        match error {
            minreq::Error::IoError(e) if e.kind() == std::io::ErrorKind::ConnectionRefused => {
                Error::UnreachableServer
            }
            e => Error::Unknown(format!("{:?}", e)),
        }
    }
}
