use crate::Error;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;
#[cfg(not(target_arch = "wasm32"))]
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[cfg(not(target_arch = "wasm32"))]
#[serde(rename_all = "camelCase")]
struct TenantTokenClaim {
    api_key_uid: String,
    search_rules: Value,
    #[serde(with = "time::serde::timestamp::option")]
    exp: Option<OffsetDateTime>,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn generate_tenant_token(
    api_key_uid: String,
    search_rules: Value,
    api_key: impl AsRef<str>,
    expires_at: Option<OffsetDateTime>,
) -> Result<String, Error> {
    // Validate uuid format
    let uid = Uuid::try_parse(&api_key_uid)?;

    // Validate uuid version
    if uid.get_version_num() != 4 {
        return Err(Error::InvalidUuid4Version);
    }

    if expires_at.map_or(false, |expires_at| OffsetDateTime::now_utc() > expires_at) {
        return Err(Error::TenantTokensExpiredSignature);
    }

    let claims = TenantTokenClaim {
        api_key_uid,
        exp: expires_at,
        search_rules,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(api_key.as_ref().as_bytes()),
    );

    Ok(token?)
}

#[cfg(test)]
mod tests {
    use crate::tenant_tokens::*;
    use big_s::S;
    use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
    use serde_json::json;
    use std::collections::HashSet;

    const SEARCH_RULES: [&str; 1] = ["*"];
    const VALID_KEY: &str = "a19b6ec84ee31324efa560cd1f7e6939";

    fn build_validation() -> Validation {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = false;
        validation.required_spec_claims = HashSet::new();

        validation
    }

    #[test]
    fn test_generate_token_with_given_key() {
        let api_key_uid = S("76cf8b87-fd12-4688-ad34-260d930ca4f4");
        let token =
            generate_tenant_token(api_key_uid, json!(SEARCH_RULES), VALID_KEY, None).unwrap();

        let valid_key = decode::<TenantTokenClaim>(
            &token,
            &DecodingKey::from_secret(VALID_KEY.as_ref()),
            &build_validation(),
        );
        let invalid_key = decode::<TenantTokenClaim>(
            &token,
            &DecodingKey::from_secret("not-the-same-key".as_ref()),
            &build_validation(),
        );

        assert!(valid_key.is_ok());
        assert!(invalid_key.is_err());
    }

    #[test]
    fn test_generate_token_without_uid() {
        let api_key_uid = S("");
        let key = S("");
        let token = generate_tenant_token(api_key_uid, json!(SEARCH_RULES), key, None);

        assert!(token.is_err());
    }

    #[test]
    fn test_generate_token_with_expiration() {
        let api_key_uid = S("76cf8b87-fd12-4688-ad34-260d930ca4f4");
        let exp = OffsetDateTime::now_utc() + time::Duration::HOUR;
        let token =
            generate_tenant_token(api_key_uid, json!(SEARCH_RULES), VALID_KEY, Some(exp)).unwrap();

        let decoded = decode::<TenantTokenClaim>(
            &token,
            &DecodingKey::from_secret(VALID_KEY.as_ref()),
            &Validation::new(Algorithm::HS256),
        );

        assert!(decoded.is_ok());
    }

    #[test]
    fn test_generate_token_with_expires_at_in_the_past() {
        let api_key_uid = S("76cf8b87-fd12-4688-ad34-260d930ca4f4");
        let exp = OffsetDateTime::now_utc() - time::Duration::HOUR;
        let token = generate_tenant_token(api_key_uid, json!(SEARCH_RULES), VALID_KEY, Some(exp));

        assert!(token.is_err());
    }

    #[test]
    fn test_generate_token_contains_claims() {
        let api_key_uid = S("76cf8b87-fd12-4688-ad34-260d930ca4f4");
        let token =
            generate_tenant_token(api_key_uid.clone(), json!(SEARCH_RULES), VALID_KEY, None)
                .unwrap();

        let decoded = decode::<TenantTokenClaim>(
            &token,
            &DecodingKey::from_secret(VALID_KEY.as_ref()),
            &build_validation(),
        )
        .expect("Cannot decode the token");

        assert_eq!(decoded.claims.api_key_uid, api_key_uid);
        assert_eq!(decoded.claims.search_rules, json!(SEARCH_RULES));
    }

    #[test]
    fn test_generate_token_with_multi_byte_chars() {
        let api_key_uid = S("76cf8b87-fd12-4688-ad34-260d930ca4f4");
        let key = "Ëa1ทt9bVcL-vãUทtP3OpXW5qPc%bWH5ทvw09";
        let token =
            generate_tenant_token(api_key_uid.clone(), json!(SEARCH_RULES), key, None).unwrap();

        let decoded = decode::<TenantTokenClaim>(
            &token,
            &DecodingKey::from_secret(key.as_ref()),
            &build_validation(),
        )
        .expect("Cannot decode the token");

        assert_eq!(decoded.claims.api_key_uid, api_key_uid);
    }

    #[test]
    fn test_generate_token_with_wrongly_formated_uid() {
        let api_key_uid = S("xxx");
        let key = "Ëa1ทt9bVcL-vãUทtP3OpXW5qPc%bWH5ทvw09";
        let token = generate_tenant_token(api_key_uid, json!(SEARCH_RULES), key, None);

        assert!(token.is_err());
    }

    #[test]
    fn test_generate_token_with_wrong_uid_version() {
        let api_key_uid = S("6a11eb96-2485-11ed-861d-0242ac120002");
        let key = "Ëa1ทt9bVcL-vãUทtP3OpXW5qPc%bWH5ทvw09";
        let token = generate_tenant_token(api_key_uid, json!(SEARCH_RULES), key, None);

        assert!(token.is_err());
    }
}
