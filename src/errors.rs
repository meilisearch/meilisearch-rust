use serde::{Deserialize, Serialize};

/// An enum representing the errors that can occur.

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// The exhaustive list of Meilisearch errors: <https://github.com/meilisearch/specifications/blob/main/text/0061-error-format-and-definitions.md>
    /// Also check out: <https://github.com/meilisearch/Meilisearch/blob/main/meilisearch-error/src/lib.rs>
    Meilisearch(MeilisearchError),
    /// There is no Meilisearch server listening on the [specified host]
    /// (../client/struct.Client.html#method.new).
    UnreachableServer,
    /// The Meilisearch server returned an invalid JSON for a request.
    ParseError(serde_json::Error),
    /// A timeout happened while waiting for an update to complete.
    Timeout,
    /// This Meilisearch SDK generated an invalid request (which was not sent).
    /// It probably comes from an invalid API key resulting in an invalid HTTP header.
    InvalidRequest,

    /// It is not possible to generate a tenant token with a invalid api key.
    /// Empty strings or with less than 8 characters are considered invalid.
    TenantTokensInvalidApiKey,
    /// It is not possible to generate an already expired tenant token.
    TenantTokensExpiredSignature,

    /// When jsonwebtoken cannot generate the token successfully.
    InvalidTenantToken(jsonwebtoken::errors::Error),

    /// The http client encountered an error.
    #[cfg(not(target_arch = "wasm32"))]
    HttpError(isahc::Error),
    /// The http client encountered an error.
    #[cfg(target_arch = "wasm32")]
    HttpError(String),
    // The library formating the query parameters encountered an error.
    Yaup(yaup::Error),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeilisearchError {
    /// The human readable error message
    #[serde(rename = "message")]
    pub error_message: String,
    /// The error code of the error.  Officially documented at
    /// <https://docs.meilisearch.com/errors>.
    #[serde(rename = "code")]
    pub error_code: ErrorCode,
    /// The type of error (invalid request, internal error, or authentication
    /// error)
    #[serde(rename = "type")]
    pub error_type: ErrorType,
    /// A link to the Meilisearch documentation for an error.
    #[serde(rename = "link")]
    pub error_link: String,
}

impl From<MeilisearchError> for Error {
    fn from(error: MeilisearchError) -> Self {
        Self::Meilisearch(error)
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(error: jsonwebtoken::errors::Error) -> Error {
        Error::InvalidTenantToken(error)
    }
}

impl From<yaup::Error> for Error {
    fn from(error: yaup::Error) -> Error {
        Error::Yaup(error)
    }
}

/// The type of error that was encountered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ErrorType {
    /// The submitted request was invalid.
    InvalidRequest,
    /// The Meilisearch instance encountered an internal error.
    Internal,
    /// Authentication was either incorrect or missing.
    Auth,

    /// That's unexpected. Please open a GitHub issue after ensuring you are
    /// using the supported version of the Meilisearch server.
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            fmt,
            "{}",
            // this can't fail
            serde_json::to_value(self).unwrap().as_str().unwrap()
        )
    }
}

/// The error code.
///
/// Officially documented at <https://docs.meilisearch.com/errors>.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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
    MissingParameter,
    InvalidApiKeyDescription,
    InvalidApiKeyActions,
    InvalidApiKeyIndexes,
    InvalidApiKeyExpiresAt,
    ApiKeyNotFound,

    /// That's unexpected. Please open a GitHub issue after ensuring you are
    /// using the supported version of the Meilisearch server.
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            fmt,
            "{}",
            // this can't fail
            serde_json::to_value(self).unwrap().as_str().unwrap()
        )
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Error::Meilisearch(MeilisearchError {
                error_message,
                error_code,
                error_type,
                error_link,
            }) => write!(
                fmt,
                "Meilisearch {}: {}: {}. {}",
                error_type,
                error_code,
                error_message,
                error_link,
            ),
            Error::UnreachableServer => write!(fmt, "The Meilisearch server can't be reached."),
            Error::InvalidRequest => write!(fmt, "Unable to generate a valid HTTP request. It probably comes from an invalid API key."),
            Error::ParseError(e) => write!(fmt, "Error parsing response JSON: {}", e),
            Error::HttpError(e) => write!(fmt, "HTTP request failed: {}", e),
            Error::Timeout => write!(fmt, "A task did not succeed in time."),
            Error::TenantTokensInvalidApiKey => write!(fmt, "The provided api_key is invalid."),
            Error::TenantTokensExpiredSignature => write!(fmt, "The provided expires_at is already expired."),
            Error::InvalidTenantToken(e) => write!(fmt, "Impossible to generate the token, jsonwebtoken encountered an error: {}", e),
            Error::Yaup(e) => write!(fmt, "Internal Error: could not parse the query parameters: {}", e)
        }
    }
}

impl std::error::Error for Error {}

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_meilisearch_error() {
        let error: MeilisearchError = serde_json::from_str(
            r#"
{
  "message": "The cool error message.",
  "code": "index_creation_failed",
  "type": "internal",
  "link": "https://the best link eveer"
}"#,
        )
        .unwrap();

        assert_eq!(error.error_message, "The cool error message.");
        assert_eq!(error.error_code, ErrorCode::IndexCreationFailed);
        assert_eq!(error.error_type, ErrorType::Internal);
        assert_eq!(error.error_link, "https://the best link eveer");

        let error: MeilisearchError = serde_json::from_str(
            r#"
{
  "message": "",
  "code": "An unknown error",
  "type": "An unknown type",
  "link": ""
}"#,
        )
        .unwrap();

        assert_eq!(error.error_code, ErrorCode::Unknown);
        assert_eq!(error.error_type, ErrorType::Unknown);
    }
}
