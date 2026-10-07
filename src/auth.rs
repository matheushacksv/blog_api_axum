use crate::errors::Error;
use crate::state::AppState;
use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Claims {
    pub sub: uuid::Uuid,
    pub exp: i64,
}

impl FromRequestParts<AppState> for Claims {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // get token from Header striping Bearer
        let token = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .ok_or(Error::Unauthorized)?;

        // decode_token
        let claims = decode_token(token, state.config.jwt_secret())?;

        Ok(claims)
    }
}

pub fn create_token(user_id: uuid::Uuid, secret: &str) -> Result<String, Error> {
    // calculate exp = now + 24h in seconds
    let exp = (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp();

    // instance claims (user_id, exp)
    let claims = Claims { sub: user_id, exp };

    // signs and generate string token
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| Error::Internal)
}

pub fn decode_token(token: &str, secret: &str) -> Result<Claims, Error> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| Error::Unauthorized)?;

    Ok(data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_decode_secret_token() {
        let id = uuid::Uuid::new_v4();
        let token = create_token(id, "secret-a").unwrap();
        let claims = decode_token(&token, "secret-a").unwrap();
        assert_eq!(claims.sub, id);
    }

    #[test]
    fn incorrect_decode_secret_token() {
        let id = uuid::Uuid::new_v4();
        let token = create_token(id, "secret-a").unwrap();
        let result = decode_token(&token, "secret-b");
        assert!(matches!(result, Err(Error::Unauthorized)));
    }
}
