/// An enum representing the errors that can occur.

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// The exhaustive list of MeiliSearch errors: https://github.com/meilisearch/specifications/blob/main/text/0061-error-format-and-definitions.md
    /// Also check out: https://github.com/meilisearch/MeiliSearch/blob/main/meilisearch-error/src/lib.rs
    MeiliSearchError {
        /// The human readable error message
        error_message: String,
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
    /// The MeiliSearch server returned an invalid JSON for a request.
    ParseError(serde_json::Error),
    /// This MeiliSearch SDK generated an invalid request (which was not sent).
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
    IndexCreationFailed,
    IndexAlreadyExists,
    IndexNotFound,
    InvalidIndexUid,
    InvalidState,
    PrimaryKeyInferenceFailed,
    IndexPrimaryKeyAlreadyPresent,
    InvalidRankingRule,
    InvalidStoreFile,
    MaxFieldsLimitExceeded,
    MissingDocumentId,
    InvalidDocumentId,
    InvalidFilter,
    InvalidSort,
    BadParameter,
    BadRequest,
    DatabaseSizeLimitReached,
    DocumentNotFound,
    InternalError,
    InvalidGeoField,
    InvalidApiKey,
    MissingAuthorizationHeader,
    TaskNotFound,
    DumpNotFound,
    NoSpaceLeftOnDevice,
    PayloadTooLarge,
    UnretrievableDocument,
    SearchError,
    UnsupportedMediaType,
    DumpAlreadyProcessing,
    DumpProcessFailed,
    MissingContentType,
    MalformedPayload,
    InvalidContentType,
    MissingPayload,

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
            ErrorType::InvalidRequest => "invalid_request",
            ErrorType::Internal => "internal",
            ErrorType::Authentication => "authentication",
        }
    }
    /// Converts the error type string returned by MeiliSearch into an
    /// `ErrorType` enum. If the error type input is not recognized, None is
    /// returned.
    pub fn parse(input: &str) -> Option<Self> {
        match input {
            "invalid_request" => Some(ErrorType::InvalidRequest),
            "internal" => Some(ErrorType::Internal),
            "authentication" => Some(ErrorType::Authentication),
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
            ErrorCode::IndexNotFound => "index_not_found",
            ErrorCode::InvalidIndexUid => "invalid_index_uid",
            ErrorCode::InvalidState => "invalid_state",
            ErrorCode::PrimaryKeyInferenceFailed => "primary_key_inference_failed",
            ErrorCode::IndexPrimaryKeyAlreadyPresent => "index_primary_key_already_exists",
            ErrorCode::InvalidRankingRule => "invalid_ranking_rule",
            ErrorCode::InvalidStoreFile => "invalid_store_file",
            ErrorCode::MaxFieldsLimitExceeded => "max_field_limit_exceeded",
            ErrorCode::MissingDocumentId => "missing_document_id",
            ErrorCode::InvalidDocumentId => "invalid_document_id",
            ErrorCode::InvalidFilter => "invalid_filter",
            ErrorCode::InvalidSort => "invalid_sort",
            ErrorCode::BadParameter => "bad_parameter",
            ErrorCode::BadRequest => "bad_request",
            ErrorCode::DatabaseSizeLimitReached => "database_size_limit_reached",
            ErrorCode::DocumentNotFound => "document_not_found",
            ErrorCode::InternalError => "internal",
            ErrorCode::InvalidGeoField => "invalid_geo_field",
            ErrorCode::InvalidApiKey => "invalid_api_key",
            ErrorCode::MissingAuthorizationHeader => "missing_authorization_header",
            ErrorCode::TaskNotFound => "task_not_found",
            ErrorCode::DumpNotFound => "dump_not_found",
            ErrorCode::NoSpaceLeftOnDevice => "no_space_left_on_device",
            ErrorCode::PayloadTooLarge => "payload_too_large",
            ErrorCode::UnretrievableDocument => "unretrievable_document",
            ErrorCode::SearchError => "search_error",
            ErrorCode::UnsupportedMediaType => "unsupported_media_type",
            ErrorCode::DumpAlreadyProcessing => "dump_already_processing",
            ErrorCode::DumpProcessFailed => "dump_process_failed",
            ErrorCode::MissingContentType => "missing_content_type",
            ErrorCode::MalformedPayload => "malformed_payload",
            ErrorCode::InvalidContentType => "invalid_content_type",
            ErrorCode::MissingPayload => "missing_payload",
            // Other than this variant, all the other `&str`s are 'static
            ErrorCode::Unknown(inner) => &inner.0,
        }
    }
    /// Converts the error code string returned by MeiliSearch into an `ErrorCode`
    /// enum. If the error type input is not recognized, `ErrorCode::Unknown`
    /// is returned.
    pub fn parse(input: &str) -> Self {
        match input {
            "index_creation_failed" => ErrorCode::IndexCreationFailed,
            "index_already_exists" => ErrorCode::IndexAlreadyExists,
            "index_not_found" => ErrorCode::IndexNotFound,
            "invalid_index_uid" => ErrorCode::InvalidIndexUid,
            "invalid_state" => ErrorCode::InvalidState,
            "primary_key_inference_failed" => ErrorCode::PrimaryKeyInferenceFailed,
            "index_primary_key_already_exists" => ErrorCode::IndexPrimaryKeyAlreadyPresent,
            "invalid_ranking_rule" => ErrorCode::InvalidRankingRule,
            "invalid_store_file" => ErrorCode::InvalidStoreFile,
            "max_field_limit_exceeded" => ErrorCode::MaxFieldsLimitExceeded,
            "missing_document_id" => ErrorCode::MissingDocumentId,
            "invalid_document_id" => ErrorCode::InvalidDocumentId,
            "invalid_filter" => ErrorCode::InvalidFilter,
            "invalid_sort" => ErrorCode::InvalidSort,
            "bad_parameter" => ErrorCode::BadParameter,
            "bad_request" => ErrorCode::BadRequest,
            "database_size_limit_reached" => ErrorCode::DatabaseSizeLimitReached,
            "document_not_found" => ErrorCode::DocumentNotFound,
            "internal" => ErrorCode::InternalError,
            "invalid_geo_field" => ErrorCode::InvalidGeoField,
            "invalid_api_key" => ErrorCode::InvalidApiKey,
            "missing_authorization_header" => ErrorCode::MissingAuthorizationHeader,
            "task_not_found" => ErrorCode::TaskNotFound,
            "dump_not_found" => ErrorCode::DumpNotFound,
            "no_space_left_on_device" => ErrorCode::NoSpaceLeftOnDevice,
            "payload_too_large" => ErrorCode::PayloadTooLarge,
            "unretrievable_document" => ErrorCode::UnretrievableDocument,
            "search_error" => ErrorCode::SearchError,
            "unsupported_media_type" => ErrorCode::UnsupportedMediaType,
            "dump_already_processing" => ErrorCode::DumpAlreadyProcessing,
            "dump_process_failed" => ErrorCode::DumpProcessFailed,
            "missing_content_type" => ErrorCode::MissingContentType,
            "malformed_payload" => ErrorCode::MalformedPayload,
            "invalid_content_type" => ErrorCode::InvalidContentType,
            "missing_payload" => ErrorCode::MissingPayload,
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
                error_message,
                error_code,
                error_type,
                error_link,
            } => write!(
                fmt,
                "Meilisearch {}: {}: {}. {}",
                error_type.as_str(),
                error_code,
                error_message,
                error_link,
            ),
            Error::UnreachableServer => write!(fmt, "The MeiliSearch server can't be reached."),
            Error::InvalidRequest => write!(fmt, "Unable to generate a valid HTTP request. It probably comes from an invalid API key."),
            Error::ParseError(e) => write!(fmt, "Error parsing response JSON: {}", e),
            Error::HttpError(e) => write!(fmt, "HTTP request failed: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<&serde_json::Value> for Error {
    fn from(json: &serde_json::Value) -> Error {

        let error_message = json
            .get("message")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| json.to_string());

        let error_link = json
            .get("link")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(String::new);

        let error_type = json
            .get("type")
            .and_then(|v| v.as_str())
            .and_then(|s| ErrorType::parse(s))
            .unwrap_or(ErrorType::Internal);

        // If the response doesn't contain a type field, the error type
        // is assumed to be an internal error.

        let error_code = json
            .get("code")
            .and_then(|v| v.as_str())
            .map(|s| ErrorCode::parse(s))
            .unwrap_or_else(|| {
                ErrorCode::Unknown(UnknownErrorCode(String::from("missing error code")))
            });

        Error::MeiliSearchError {
            error_message,
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
