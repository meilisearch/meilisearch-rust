use crate::{
    errors::* 
};
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, Header, EncodingKey};
use time::{OffsetDateTime};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")] 
struct TenantTokenClaim {
    api_key_prefix: String,
    search_rules: Value,
    #[serde(with = "time::serde::timestamp::option")]
    exp: Option<OffsetDateTime>,
}

pub fn generate_tenant_token(search_rules: Value, api_key: impl AsRef<str>, expires_at: Option<OffsetDateTime>) -> Result<String, Error> {
    if api_key.as_ref().chars().count() < 8 {
        return Err(Error::TenantTokensInvalidApiKey)
    }

    if expires_at.map_or(false, |expires_at| OffsetDateTime::now_utc() > expires_at) {
        return Err(Error::TenantTokensExpiredSignature)
    }

    let key_prefix = api_key.as_ref().chars().take(8).collect();
    let claims = TenantTokenClaim {
        api_key_prefix: key_prefix,
        exp: expires_at,
        search_rules
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
    use serde_json::json;
    use crate::tenant_tokens::*;
    use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
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
        let token = generate_tenant_token(json!(SEARCH_RULES), VALID_KEY, None).unwrap();

        let valid_key = decode::<TenantTokenClaim>(
            &token, &DecodingKey::from_secret(VALID_KEY.as_ref()), &build_validation()
        );
        let invalid_key = decode::<TenantTokenClaim>(
            &token, &DecodingKey::from_secret("not-the-same-key".as_ref()), &build_validation()
        );

        assert!(valid_key.is_ok());
        assert!(invalid_key.is_err());
    }

    #[test]
    fn test_generate_token_without_key() {
        let key = String::from("");
        let token = generate_tenant_token(json!(SEARCH_RULES), &key, None);

        assert!(token.is_err());
    }

    #[test]
    fn test_generate_token_with_expiration() {
        let exp = OffsetDateTime::now_utc() + time::Duration::HOUR;
        let token = generate_tenant_token(json!(SEARCH_RULES), VALID_KEY, Some(exp)).unwrap();

        let decoded = decode::<TenantTokenClaim>(
            &token, &DecodingKey::from_secret(VALID_KEY.as_ref()), &Validation::new(Algorithm::HS256)
        );

        assert!(decoded.is_ok());
    }

    #[test]
    fn test_generate_token_with_expires_at_in_the_past() {
        let exp = OffsetDateTime::now_utc() - time::Duration::HOUR;
        let token = generate_tenant_token(json!(SEARCH_RULES), VALID_KEY, Some(exp));

        assert!(token.is_err());
    }

    #[test]
    fn test_generate_token_contains_claims() {
        let token = generate_tenant_token(json!(SEARCH_RULES), VALID_KEY, None).unwrap();

        let decoded = decode::<TenantTokenClaim>(
            &token, &DecodingKey::from_secret(VALID_KEY.as_ref()), &build_validation()
        ).expect("Cannot decode the token");

        assert_eq!(decoded.claims.api_key_prefix, &VALID_KEY[..8]);
        assert_eq!(decoded.claims.search_rules, json!(SEARCH_RULES));
    }

    #[test]
    fn test_generate_token_with_multi_byte_chars() {
        let key = "Ëa1ทt9bVcL-vãUทtP3OpXW5qPc%bWH5ทvw09";
        let token = generate_tenant_token(json!(SEARCH_RULES), key, None).unwrap();

        let decoded = decode::<TenantTokenClaim>(
            &token, &DecodingKey::from_secret(key.as_ref()), &build_validation()
        ).expect("Cannot decode the token");

        assert_eq!(decoded.claims.api_key_prefix, "Ëa1ทt9bV");
    }
}
