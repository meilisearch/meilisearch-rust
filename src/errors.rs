/// The enum representing the errors that can occur
#[derive(Debug)]
pub enum Error {
    MeiliSearchError {
        /// The human readable error message
        message: String,
        /// The error code of the error.  For MeiliSearch versions before v12,
        /// this is an approximation of the real error code, as MeiliSearch did
        /// not send a machine readable error code.
        error_code: ErrorCode,
        /// A link to the MeiliSearch documentation for an error.  Only exists
        /// for MeiliSearch versions v12 and above.
        error_link: Option<String>,
    },

    /// There is no MeiliSearch server listening on the [specified host]
    /// (../client/struct.Client.html#method.new).
    UnreachableServer,
    /// The MeiliSearch server returned invalid JSON for a request.
    ParseError(serde_json::Error),
    /// An unknown error
    Unknown(String),
    /// An erroring status code, but no body
    Empty,

    /// The http client encountered an error.
    #[cfg(not(target_arch = "wasm32"))]
    HttpError(reqwest::Error),
    /// Can never occur on the wasm target.
    #[cfg(target_arch = "wasm32")]
    HttpError(()),
}

#[derive(Debug)]
pub enum ErrorCode {
    IndexCreationFailed,
    /// You tried to create an Index that already exists. You may want to use the [get_or_create method](../client/struct.Client.html#method.get_or_create).
    IndexAlreadyExists,
    /// You tried to get an Index that does not exist. You may want to use the [get_or_create method](../client/struct.Client.html#method.get_or_create).
    IndexNotFound,
    /// You tried to use an invalid UID for an Index. Index UID can only be composed of alphanumeric characters, hyphens (-), and underscores (_).
    InvalidIndexUid,
    IndexNotAccessible,

    InvalidState,
    /// You tried to add documents on an Index but MeiliSearch can't infer the primary key. Consider specifying the key.
    MissingPrimaryKey,
    PrimaryKeyAlreadyPresent,

    MaxFieldsLimitExceeded,
    MissingDocumentId,

    InvalidFacet,
    InvalidFilter,

    BadParameter,
    BadRequest,
    DocumentNotFound,
    InternalError,
    InvalidToken,
    /// Server is in maintenance. You can set the maintenance state by using the `set_healthy` method of a Client.
    Maintenance,
    MissingAuthorizationHeader,
    NotFound,
    PayloadTooLarge,
    UnretrievableDocument,
    SearchError,
    UnsupportedMediaType,

    /// That's unexpected. Please open a GitHub issue after ensuring you are using the supported version of the MeiliSearch server.
    Unknown(String),
}

impl std::fmt::Display for ErrorCode {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        match self {
            ErrorCode::IndexCreationFailed => write!(formatter, "index_creation_failed"),
            ErrorCode::IndexAlreadyExists => write!(formatter, "index_already_exists"),
            ErrorCode::IndexNotFound => write!(formatter, "index_not_found"),
            ErrorCode::InvalidIndexUid => write!(formatter, "invalid_index_uid"),
            ErrorCode::IndexNotAccessible => write!(formatter, "index_not_accessible"),
            ErrorCode::InvalidState => write!(formatter, "invalid_state"),
            ErrorCode::MissingPrimaryKey => write!(formatter, "missing_primary_key"),
            ErrorCode::PrimaryKeyAlreadyPresent => write!(formatter, "primary_key_already_present"),
            ErrorCode::MaxFieldsLimitExceeded => write!(formatter, "max_field_limit_exceeded"),
            ErrorCode::MissingDocumentId => write!(formatter, "missing_document_id"),
            ErrorCode::InvalidFacet => write!(formatter, "invalid_facet"),
            ErrorCode::InvalidFilter => write!(formatter, "invalid_filter"),
            ErrorCode::BadParameter => write!(formatter, "bad_parameter"),
            ErrorCode::BadRequest => write!(formatter, "bad_request"),
            ErrorCode::DocumentNotFound => write!(formatter, "document_not_found"),
            ErrorCode::InternalError => write!(formatter, "internal"),
            ErrorCode::InvalidToken => write!(formatter, "invalid_token"),
            ErrorCode::Maintenance => write!(formatter, "maintenance"),
            ErrorCode::MissingAuthorizationHeader => {
                write!(formatter, "missing_authorization_header")
            }
            ErrorCode::NotFound => write!(formatter, "not_found"),
            ErrorCode::PayloadTooLarge => write!(formatter, "payload_too_large"),
            ErrorCode::UnretrievableDocument => write!(formatter, "unretrievable_document"),
            ErrorCode::SearchError => write!(formatter, "search_error"),
            ErrorCode::UnsupportedMediaType => write!(formatter, "unsupported_media_type"),

            ErrorCode::Unknown(inner) => write!(formatter, "unknown ({})", inner),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Error::MeiliSearchError {
                message,
                error_code,
                error_link,
            } => {
                write!(formatter, "Meilisearch error {}: {}", error_code, message)?;
                if let Some(link) = error_link {
                    write!(formatter, ". {}", link)?;
                }
                Ok(())
            }
            Error::UnreachableServer => {
                write!(formatter, "The MeiliSearch server can't be reached.")
            }
            Error::ParseError(e) => write!(formatter, "Error parsing response JSON: {}", e),
            Error::HttpError(e) => write!(formatter, "HTTP request failed: {}", e),
            Error::Unknown(e) => write!(formatter, "An unknown error occurred: {}", e),
            Error::Empty => write!(formatter, "An error occured without a message"),
        }
    }
}

impl std::error::Error for Error {}

impl From<&serde_json::Value> for Error {
    fn from(json: &serde_json::Value) -> Error {
        if json.is_null() {
            return Error::Empty;
        }

        let message = json
            .get("message")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| json.to_string());

        let link = json
            .get("errorLink")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        // Error codes from https://github.com/meilisearch/MeiliSearch/blob/v0.12.0/meilisearch-error/src/lib.rs
        let error_code = match json.get("errorCode").and_then(|v| v.as_str()) {
            Some("index_creation_failed") => ErrorCode::IndexCreationFailed,
            Some("index_already_exists") => ErrorCode::IndexAlreadyExists,
            Some("index_not_found") => ErrorCode::IndexNotFound,
            Some("invalid_index_uid") => ErrorCode::InvalidIndexUid,
            Some("index_not_accessible") => ErrorCode::IndexNotAccessible,
            Some("invalid_state") => ErrorCode::InvalidState,
            Some("missing_primary_key") => ErrorCode::MissingPrimaryKey,
            Some("primary_key_already_present") => ErrorCode::PrimaryKeyAlreadyPresent,
            Some("max_field_limit_exceeded") => ErrorCode::MaxFieldsLimitExceeded,
            Some("missing_document_id") => ErrorCode::MissingDocumentId,
            Some("invalid_facet") => ErrorCode::InvalidFacet,
            Some("invalid_filter") => ErrorCode::InvalidFilter,
            Some("bad_parameter") => ErrorCode::BadParameter,
            Some("bad_request") => ErrorCode::BadRequest,
            Some("document_not_found") => ErrorCode::DocumentNotFound,
            Some("internal") => ErrorCode::InternalError,
            Some("invalid_token") => ErrorCode::InvalidToken,
            Some("maintenance") => ErrorCode::Maintenance,
            Some("missing_authorization_header") => ErrorCode::MissingAuthorizationHeader,
            Some("not_found") => ErrorCode::NotFound,
            Some("payload_too_large") => ErrorCode::PayloadTooLarge,
            Some("unretrievable_document") => ErrorCode::UnretrievableDocument,
            Some("search_error") => ErrorCode::SearchError,
            Some("unsupported_media_type") => ErrorCode::UnsupportedMediaType,
            Some(code) => ErrorCode::Unknown(code.to_string()),

            None => {
                // Meilisearch 0.11 and below
                match json.get("message").and_then(|v| v.as_str()) {
                    Some("Payload to large") => ErrorCode::PayloadTooLarge,
                    Some("Unsupported media type") => ErrorCode::UnsupportedMediaType,
                    Some(m) if m.starts_with("impossible to search documents") => {
                        ErrorCode::SearchError
                    }
                    Some(m) if m.starts_with("Url parameter") => ErrorCode::BadParameter,
                    Some("Impossible to create index; index already exists") => {
                        ErrorCode::IndexAlreadyExists
                    }
                    Some("Could not infer a primary key") => ErrorCode::MissingPrimaryKey,
                    Some("Server is in maintenance, please try again later") => {
                        ErrorCode::Maintenance
                    }
                    Some(m) if m.starts_with("Index ") && m.ends_with(" not found") => {
                        ErrorCode::IndexNotFound
                    }
                    Some(m) if m.starts_with("Index must have a valid uid;") => {
                        ErrorCode::InvalidIndexUid
                    }
                    _ => ErrorCode::Unknown(String::from("unknown")),
                }
            }
        };

        Error::MeiliSearchError {
            message,
            error_code,
            error_link: link,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        match error.status() {
            None => Error::UnreachableServer,
            Some(_e) => Error::HttpError(error),
        }
    }
}

// TODO from http code https://docs.meilisearch.com/references/#error
