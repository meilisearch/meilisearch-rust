use serde::{Deserialize, Serialize};
use thiserror::Error;

/// An enum representing the errors that can occur.

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// The exhaustive list of Meilisearch errors: <https://github.com/meilisearch/specifications/blob/main/text/0061-error-format-and-definitions.md>
    /// Also check out: <https://github.com/meilisearch/Meilisearch/blob/main/meilisearch-error/src/lib.rs>
    #[error(transparent)]
    Meilisearch(#[from] MeilisearchError),
    /// There is no Meilisearch server listening on the [specified host]
    /// (../client/struct.Client.html#method.new).
    #[error("The Meilisearch server can't be reached.")]
    UnreachableServer,
    /// The Meilisearch server returned an invalid JSON for a request.
    #[error("Error parsing response JSON: {}", .0)]
    ParseError(#[from] serde_json::Error),
    /// A timeout happened while waiting for an update to complete.
    #[error("A task did not succeed in time.")]
    Timeout,
    /// This Meilisearch SDK generated an invalid request (which was not sent).
    /// It probably comes from an invalid API key resulting in an invalid HTTP header.
    #[error("Unable to generate a valid HTTP request. It probably comes from an invalid API key.")]
    InvalidRequest,

    /// It is not possible to generate a tenant token with a invalid api key.
    /// Empty strings or with less than 8 characters are considered invalid.
    #[error("The provided api_key is invalid.")]
    TenantTokensInvalidApiKey,
    /// It is not possible to generate an already expired tenant token.
    #[error("The provided expires_at is already expired.")]
    TenantTokensExpiredSignature,

    /// When jsonwebtoken cannot generate the token successfully.
    #[error(transparent)]
    InvalidTenantToken(#[from] jsonwebtoken::errors::Error),

    /// The http client encountered an error.
    #[cfg(not(target_arch = "wasm32"))]
    #[error("HTTP request failed: {}", .0)]
    HttpError(isahc::Error),

    /// The http client encountered an error.
    #[cfg(target_arch = "wasm32")]
    #[error("HTTP request failed: {}", .0)]
    HttpError(String),

    // The library formating the query parameters encountered an error.
    #[error("Internal Error: could not parse the query parameters: {}", .0)]
    Yaup(#[from] yaup::Error),
    // The library validating the format of an uuid.
    #[cfg(not(target_arch = "wasm32"))]
    #[error("The uid of the token has bit an uuid4 format: {}", .0)]
    Uuid(#[from] uuid::Error),
    // Error thrown in case the version of the Uuid is not v4.
    #[error("The uid provided to the token is not of version uuidv4")]
    InvalidUuid4Version,
}

#[derive(Debug, Clone, Deserialize, Error)]
#[serde(rename_all = "camelCase")]
#[error("Meilisearch {}: {}: {}. {}", .error_type, .error_code, .error_message, .error_link)]
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
        assert!(error.to_string().contains("Meilisearch"));
        assert!(error.to_string().contains("internal"));
        assert!(error.to_string().contains("index_creation_failed"));
        assert!(error.to_string().contains("The cool error message"));
        assert!(error.to_string().contains("https://the best link eveer"));

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
