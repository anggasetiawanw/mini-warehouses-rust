use anyhow::Context;
use chrono::{ Utc };
use std::collections::HashSet;
use serde::{ Serialize, Deserialize };

#[derive(Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expires_in_minutes: i64,
    pub issuer: String,
}
impl JwtConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv::dotenv().ok();
        let secret = std::env::var("JWT_SECRET").context("JWT_SECRET must be set")?;
        let expires_in_minutes: i64 = std::env
            ::var("JWT_EXPIRES_IN")
            .ok()
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(60 * 24);
        let issuer = std::env::var("JWT_ISSUER").unwrap_or_else(|_| "mini-warehouse".to_string());
        Ok(Self { secret, expires_in_minutes, issuer })
    }

    pub fn encoding_key(&self) -> jsonwebtoken::EncodingKey {
        jsonwebtoken::EncodingKey::from_secret(self.secret.as_ref())
    }

    pub fn decoding_key(&self) -> jsonwebtoken::DecodingKey {
        jsonwebtoken::DecodingKey::from_secret(self.secret.as_ref())
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iss: String,
    pub iat: i64,
    pub name: String,
    pub email: String,
    pub role: String,
}

pub fn sign_jwt(
    cfg: &JwtConfig,
    user_id: i64,
    name: &str,
    email: &str,
    role: &str
) -> anyhow::Result<String> {
    let now = Utc::now();
    let exp = now + chrono::Duration::minutes(cfg.expires_in_minutes);
    let claims = Claims {
        sub: user_id.to_string(),
        exp: exp.timestamp(),
        iss: cfg.issuer.clone(),
        iat: now.timestamp(),
        name: name.to_string(),
        email: email.to_string(),
        role: role.to_string(),
    };
    let mut header = jsonwebtoken::Header::default();
    header.alg = jsonwebtoken::Algorithm::HS256;
    let token = jsonwebtoken::encode(&header, &claims, &cfg.encoding_key())?;
    Ok(token)
}

pub fn verify_jwt(cfg: &JwtConfig, token: &str) -> anyhow::Result<Claims> {
    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.set_required_spec_claims(&["sub", "exp", "iss", "iat"]);
    validation.validate_exp = true;
    validation.iss = Some(HashSet::from([cfg.issuer.clone()]));

    let token_data = jsonwebtoken::decode::<Claims>(token, &cfg.decoding_key(), &validation)?;
    Ok(token_data.claims)
}
