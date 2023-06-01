use serde::{Deserialize, Serialize};
use thiserror::Error;

/// An enum representing the errors that can occur.

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// The exhaustive list of Meilisearch errors: <https://github.com/meilisearch/specifications/blob/main/text/0061-error-format-and-definitions.md>
    ///
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
    ///
    /// It probably comes from an invalid API key resulting in an invalid HTTP header.
    #[error("Unable to generate a valid HTTP request. It probably comes from an invalid API key.")]
    InvalidRequest,

    /// Can't call this method without setting an api key in the client.
    #[error("You need to provide an api key to use the `{0}` method.")]
    CantUseWithoutApiKey(String),
    /// It is not possible to generate a tenant token with a invalid api key.
    ///
    /// Empty strings or with less than 8 characters are considered invalid.
    #[error("The provided api_key is invalid.")]
    TenantTokensInvalidApiKey,
    /// It is not possible to generate an already expired tenant token.
    #[error("The provided expires_at is already expired.")]
    TenantTokensExpiredSignature,

    /// When jsonwebtoken cannot generate the token successfully.
    #[error("Impossible to generate the token, jsonwebtoken encountered an error: {}", .0)]
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
    /// <https://www.meilisearch.com/docs/reference/errors/error_codes>.
    #[serde(rename = "code")]
    pub error_code: ErrorCode,
    /// The type of error (invalid request, internal error, or authentication error)
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
/// Officially documented at <https://www.meilisearch.com/docs/reference/errors/error_codes>.
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
    InvalidStoreFile,
    MaxFieldsLimitExceeded,
    MissingDocumentId,
    InvalidDocumentId,
    BadParameter,
    BadRequest,
    DatabaseSizeLimitReached,
    DocumentNotFound,
    InternalError,
    InvalidApiKey,
    MissingAuthorizationHeader,
    TaskNotFound,
    DumpNotFound,
    MssingMasterKey,
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
    InvalidApiKeyDescription,
    InvalidApiKeyActions,
    InvalidApiKeyIndexes,
    InvalidApiKeyExpiresAt,
    ApiKeyNotFound,
    MissingTaskFilters,
    MissingIndexUid,
    InvalidIndexOffset,
    InvalidIndexLimit,
    InvalidIndexPrimaryKey,
    InvalidDocumentFields,
    InvalidDocumentLimit,
    InvalidDocumentOffset,
    InvalidDocumentGeoField,
    InvalidSearchQ,
    InvalidSearchOffset,
    InvalidSearchLimit,
    InvalidSearchPage,
    InvalidSearchHitsPerPage,
    InvalidSearchAttributesToRetrieve,
    InvalidSearchAttributesToCrop,
    InvalidSearchCropLength,
    InvalidSearchAttributesToHighlight,
    InvalidSearchShowMatchesPosition,
    InvalidSearchFilter,
    InvalidSearchSort,
    InvalidSearchFacets,
    InvalidSearchHighlightPreTag,
    InvalidSearchHighlightPostTag,
    InvalidSearchCropMarker,
    InvalidSearchMatchingStrategy,
    ImmutableApiKeyUid,
    ImmutableApiKeyActions,
    ImmutableApiKeyIndexes,
    ImmutableExpiresAt,
    ImmutableCreatedAt,
    ImmutableUpdatedAt,
    InvalidSwapDuplicateIndexFound,
    InvalidSwapIndexes,
    MissingSwapIndexes,
    InvalidTaskTypes,
    InvalidTaskUids,
    InvalidTaskStatuses,
    InvalidTaskLimit,
    InvalidTaskFrom,
    InvalidTaskCanceledBy,
    InvalidTaskFilters,
    TooManyOpenFiles,
    IoError,
    InvalidTaskIndexUids,
    ImmutableIndexUid,
    ImmutableIndexCreatedAt,
    ImmutableIndexUpdatedAt,
    InvalidSettingsDisplayedAttributes,
    InvalidSettingsSearchableAttributes,
    InvalidSettingsFilterableAttributes,
    InvalidSettingsSortableAttributes,
    InvalidSettingsRankingRules,
    InvalidSettingsStopWords,
    InvalidSettingsSynonyms,
    InvalidSettingsDistinctAttributes,
    InvalidSettingsTypoTolerance,
    InvalidSettingsFaceting,
    InvalidSettingsPagination,
    InvalidTaskBeforeEnqueuedAt,
    InvalidTaskAfterEnqueuedAt,
    InvalidTaskBeforeStartedAt,
    InvalidTaskAfterStartedAt,
    InvalidTaskBeforeFinishedAt,
    InvalidTaskAfterFinishedAt,
    MissingApiKeyActions,
    MissingApiKeyIndexes,
    MissingApiKeyExpiresAt,
    InvalidApiKeyLimit,
    InvalidApiKeyOffset,

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

    use jsonwebtoken::errors::ErrorKind::InvalidToken;
    use uuid::Uuid;

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

    #[test]
    fn test_error_message_parsing() {
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

        assert_eq!(error.to_string(), ("Meilisearch internal: index_creation_failed: The cool error message.. https://the best link eveer"));

        let error = Error::UnreachableServer;
        assert_eq!(
            error.to_string(),
            "The Meilisearch server can't be reached."
        );

        let error = Error::Timeout;
        assert_eq!(error.to_string(), "A task did not succeed in time.");

        let error = Error::InvalidRequest;
        assert_eq!(
            error.to_string(),
            "Unable to generate a valid HTTP request. It probably comes from an invalid API key."
        );

        let error = Error::TenantTokensInvalidApiKey;
        assert_eq!(error.to_string(), "The provided api_key is invalid.");

        let error = Error::TenantTokensExpiredSignature;
        assert_eq!(
            error.to_string(),
            "The provided expires_at is already expired."
        );

        let error = Error::InvalidUuid4Version;
        assert_eq!(
            error.to_string(),
            "The uid provided to the token is not of version uuidv4"
        );

        let error = Error::Uuid(Uuid::parse_str("67e55044").unwrap_err());
        assert_eq!(error.to_string(), "The uid of the token has bit an uuid4 format: invalid length: expected length 32 for simple format, found 8");

        let data = r#"
        {
            "name": "John Doe"
            "age": 43,
        }"#;

        let error = Error::ParseError(serde_json::from_str::<String>(data).unwrap_err());
        assert_eq!(
            error.to_string(),
            "Error parsing response JSON: invalid type: map, expected a string at line 2 column 8"
        );

        let error = Error::HttpError(isahc::post("test_url", "test_body").unwrap_err());
        assert_eq!(
            error.to_string(),
            "HTTP request failed: failed to resolve host name"
        );

        let error = Error::InvalidTenantToken(jsonwebtoken::errors::Error::from(InvalidToken));
        assert_eq!(
            error.to_string(),
            "Impossible to generate the token, jsonwebtoken encountered an error: InvalidToken"
        );

        let error = Error::Yaup(yaup::error::Error::Custom("Test yaup error".to_string()));
        assert_eq!(
            error.to_string(),
            "Internal Error: could not parse the query parameters: Test yaup error"
        );
    }
}
