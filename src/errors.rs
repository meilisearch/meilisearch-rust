/// An enum representing the errors that can occur.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    MeiliSearchError {
        /// The human readable error message
        message: String,
        /// The error code of the error.  Officially documented at
        /// https://docs.meilisearch.com/errors.
        error_code: ErrorCode,
        /// The type of error (invalid request, internal error, or authentication
        /// error)
        error_type: ErrorType,
        /// A link to the MeiliSearch documentation for an error.
        error_link: String,
    },

    /// There is no MeiliSearch server listening on the [specified host]
    /// (../client/struct.Client.html#method.new).
    UnreachableServer,
    /// The MeiliSearch server returned invalid JSON for a request.
    ParseError(serde_json::Error),
    /// This Meilisearch sdk generated an invalid request (which was not sent).
    /// It probably comes from an invalid API key resulting in an invalid HTTP header.
    InvalidRequest,

    /// The http client encountered an error.
    #[cfg(not(target_arch = "wasm32"))]
    HttpError(isahc::Error),
    /// The http client encountered an error.
    #[cfg(target_arch = "wasm32")]
    HttpError(String),
}

/// The type of error that was encountered.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ErrorType {
    /// The submitted request was invalid.
    InvalidRequest,
    /// The MeiliSearch instance encountered an internal error.
    Internal,
    /// Authentication was either incorrect or missing.
    Authentication,
}

/// The error code.
///
/// Officially documented at https://docs.meilisearch.com/errors.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ErrorCode {
    /// An error occurred while trying to create an index.
    IndexCreationFailed,
    /// An index with this UID already exists. You may want to use the
    /// [`get_or_create` method](../client/struct.Client.html#method.get_or_create).
    IndexAlreadyExists,
    /// No index was found with that UID. You may want to use the [get_or_create
    /// method](../client/struct.Client.html#method.get_or_create).
    IndexNotFound,
    /// There was an error in the provided index format. Index UIDs can only be
    /// composed of alphanumeric characters, hyphens (-), and underscores (_).
    InvalidIndexUid,
    /// An internal error occurred while trying to access the requested index.
    IndexNotAccessible,
    /// The database is in an invalid state.  Deleting the database and
    /// re-indexing should solve the problem.
    InvalidState,
    /// MeiliSearch couldn't infer the primary key for the given documents.
    /// Consider specifying the key manually.
    MissingPrimaryKey,
    /// The index already has a set primary key which can't be changed.
    PrimaryKeyAlreadyPresent,
    /// A document was added with more than 65,535 fields.
    MaxFieldsLimitExceeded,
    /// A document is missing its primary key.
    MissingDocumentId,

    /// The facet provided with the search was invalid.
    InvalidFacet,
    /// The filter provided with the search was invalid.
    InvalidFilter,

    /// The request contains invalid parameters, check the error message for
    /// more information.
    BadParameter,
    /// The request is invalid, check the error message for more information.
    BadRequest,
    /// The requested document can't be retrieved. Either it doesn't exist, or
    /// the database was left in an inconsistent state.
    DocumentNotFound,
    /// MeiliSearch experienced an internal error. Check the error message and
    /// open an issue if necessary.
    InternalError,
    /// The provided token is invalid.
    InvalidToken,
    /// The MeiliSearch instance is under maintenance.
    Maintenance,
    /// The requested resources are protected with an API key, which was not
    /// provided in the request header.
    MissingAuthorizationHeader,
    /// The requested resources could not be found.
    NotFound,
    /// The payload sent to the server was too large.
    PayloadTooLarge,
    /// The document exists in store, but there was an error retrieving it. This
    /// is likely caused by an inconsistent state in the database.
    UnretrievableDocument,
    /// There was an error in the search.
    SearchError,
    /// The payload content type is not supported by MeiliSearch. Currently,
    /// MeiliSearch only supports JSON payloads.
    UnsupportedMediaType,
    /// A dump creation is already in progress and a new one can't be triggered until the previous dump creation is not finished.
    DumpAlreadyInProgress,
    /// An error occured during dump creation process, task aborted.
    DumpProcessFailed,

    /// That's unexpected. Please open a GitHub issue after ensuring you are
    /// using the supported version of the MeiliSearch server.
    Unknown(UnknownErrorCode),
}


#[derive(Clone)]
pub struct UnknownErrorCode(String);

impl std::fmt::Display for UnknownErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
impl std::fmt::Debug for UnknownErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl ErrorType {
    /// Converts the error type to the string representation returned by
    /// MeiliSearch.
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorType::InvalidRequest => "invalid_request_error",
            ErrorType::Internal => "internal_error",
            ErrorType::Authentication => "authentication_error",
        }
    }
    /// Converts the error type string returned by MeiliSearch into an
    /// `ErrorType` enum.  If the error type input is not recognized, None is
    /// returned.
    pub fn parse(input: &str) -> Option<Self> {
        match input {
            "invalid_request_error" => Some(ErrorType::InvalidRequest),
            "internal_error" => Some(ErrorType::Internal),
            "authentication_error" => Some(ErrorType::Authentication),
            _ => None,
        }
    }
}

impl ErrorCode {
    /// Converts the error code to the string representation returned by
    /// MeiliSearch.
    pub fn as_str(&self) -> &str {
        match self {
            ErrorCode::IndexCreationFailed => "index_creation_failed",
            ErrorCode::IndexAlreadyExists => "index_already_exists",
            // `index_not_found` doesn't appear on the official docs, but is
            // used in the code. (https://docs.meilisearch.com/errors/)
            ErrorCode::IndexNotFound => "index_not_found",
            ErrorCode::InvalidIndexUid => "invalid_index_uid",
            ErrorCode::IndexNotAccessible => "index_not_accessible",
            ErrorCode::InvalidState => "invalid_state",
            ErrorCode::MissingPrimaryKey => "missing_primary_key",
            ErrorCode::PrimaryKeyAlreadyPresent => "primary_key_already_present",
            ErrorCode::MaxFieldsLimitExceeded => "max_field_limit_exceeded",
            ErrorCode::MissingDocumentId => "missing_document_id",
            ErrorCode::InvalidFacet => "invalid_facet",
            ErrorCode::InvalidFilter => "invalid_filter",
            ErrorCode::BadParameter => "bad_parameter",
            ErrorCode::BadRequest => "bad_request",
            ErrorCode::DocumentNotFound => "document_not_found",
            ErrorCode::InternalError => "internal",
            ErrorCode::InvalidToken => "invalid_token",
            ErrorCode::Maintenance => "maintenance",
            ErrorCode::MissingAuthorizationHeader => "missing_authorization_header",
            // The documentation also has a `missing_header` error, but
            // that doesn't currently exist in MeiliSearch.
            ErrorCode::NotFound => "not_found",
            ErrorCode::PayloadTooLarge => "payload_too_large",
            ErrorCode::UnretrievableDocument => "unretrievable_document",
            ErrorCode::SearchError => "search_error",
            ErrorCode::UnsupportedMediaType => "unsupported_media_type",
            ErrorCode::DumpAlreadyInProgress => "dump_already_in_progress",
            ErrorCode::DumpProcessFailed => "dump_process_failed",
            // Other than this variant, all the other `&str`s are 'static
            ErrorCode::Unknown(inner) => &inner.0,
        }
    }
    /// Converts the error code string returned by MeiliSearch into an `ErrorCode`
    /// enum.  If the error type input is not recognized, `ErrorCode::Unknown`
    /// is returned.
    pub fn parse(input: &str) -> Self {
        match input {
            "index_creation_failed" => ErrorCode::IndexCreationFailed,
            "index_already_exists" => ErrorCode::IndexAlreadyExists,
            "index_not_found" => ErrorCode::IndexNotFound,
            "invalid_index_uid" => ErrorCode::InvalidIndexUid,
            "index_not_accessible" => ErrorCode::IndexNotAccessible,
            "invalid_state" => ErrorCode::InvalidState,
            "missing_primary_key" => ErrorCode::MissingPrimaryKey,
            "primary_key_already_present" => ErrorCode::PrimaryKeyAlreadyPresent,
            "max_field_limit_exceeded" => ErrorCode::MaxFieldsLimitExceeded,
            "missing_document_id" => ErrorCode::MissingDocumentId,
            "invalid_facet" => ErrorCode::InvalidFacet,
            "invalid_filter" => ErrorCode::InvalidFilter,
            "bad_parameter" => ErrorCode::BadParameter,
            "bad_request" => ErrorCode::BadRequest,
            "document_not_found" => ErrorCode::DocumentNotFound,
            "internal" => ErrorCode::InternalError,
            "invalid_token" => ErrorCode::InvalidToken,
            "maintenance" => ErrorCode::Maintenance,
            "missing_authorization_header" => ErrorCode::MissingAuthorizationHeader,
            "not_found" => ErrorCode::NotFound,
            "payload_too_large" => ErrorCode::PayloadTooLarge,
            "unretrievable_document" => ErrorCode::UnretrievableDocument,
            "search_error" => ErrorCode::SearchError,
            "unsupported_media_type" => ErrorCode::UnsupportedMediaType,
            "dump_already_in_progress" => ErrorCode::DumpAlreadyInProgress,
            "dump_process_failed" => ErrorCode::DumpProcessFailed,
            inner => ErrorCode::Unknown(UnknownErrorCode(inner.to_string())),
        }
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            ErrorCode::Unknown(inner) => write!(fmt, "unknown ({})", inner),
            _ => write!(fmt, "{}", self.as_str()),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Error::MeiliSearchError {
                message,
                error_code,
                error_type,
                error_link,
            } => write!(
                fmt,
                "Meilisearch {}: {}: {}. {}",
                error_type.as_str(),
                error_code,
                message,
                error_link,
            ),
            Error::UnreachableServer => write!(fmt, "The MeiliSearch server can't be reached."),
            Error::InvalidRequest => write!(fmt, "Unable to generate a valid HTTP request. It probably comes from an invalid API key."),
            Error::ParseError(e) => write!(fmt, "Error parsing response JSON: {}", e),
            Error::HttpError(e) => write!(fmt, "HTTP request failed: {}", e)
        }
    }
}

impl std::error::Error for Error {}

impl From<&serde_json::Value> for Error {
    fn from(json: &serde_json::Value) -> Error {

        let message = json
            .get("message")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| json.to_string());

        let error_link = json
            .get("errorLink")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(String::new);

        let error_type = json
            .get("errorType")
            .and_then(|v| v.as_str())
            .and_then(|s| ErrorType::parse(s))
            .unwrap_or(ErrorType::Internal);

        // If the response doesn't contain an errorType field, the error type
        // is assumed to be an internal error.

        let error_code = json
            .get("errorCode")
            .and_then(|v| v.as_str())
            .map(|s| ErrorCode::parse(s))
            .unwrap_or_else(|| {
                ErrorCode::Unknown(UnknownErrorCode(String::from("missing errorCode")))
            });

        Error::MeiliSearchError {
            message,
            error_code,
            error_type,
            error_link,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<isahc::Error> for Error {
    fn from(error: isahc::Error) -> Error {
        if error.kind() == isahc::error::ErrorKind::ConnectionFailed {
            Error::UnreachableServer
        } else {
            Error::HttpError(error)
        }
    }
}
