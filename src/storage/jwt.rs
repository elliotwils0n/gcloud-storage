use anyhow::{anyhow, Context, Result};
use base64::Engine as _;
use std::ops::Add;

const JWT_ALG: &str = "RS256";
const JWT_TYP: &str = "JWT";
const JWT_VALID_TIME_MINUTES: i64 = 2;
const JWT_AUD: &str = "https://oauth2.googleapis.com/token";

#[derive(serde::Serialize)]
struct JwtHeaders {
    alg: &'static str,
    typ: &'static str,
}

impl JwtHeaders {
    pub fn new() -> Self {
        JwtHeaders {
            alg: JWT_ALG,
            typ: JWT_TYP,
        }
    }
}

#[derive(serde::Serialize)]
struct JwtClaims<'a> {
    iss: &'a str,
    scope: &'a str,
    aud: &'a str,
    exp: i64,
    iat: i64,
}

impl<'a> JwtClaims<'a> {
    pub fn from(iss: &'a str, scope: &'a str) -> JwtClaims<'a> {
        let iat = chrono::Local::now();
        let exp = iat.add(chrono::Duration::minutes(JWT_VALID_TIME_MINUTES));
        JwtClaims {
            iss,
            scope,
            aud: JWT_AUD,
            exp: exp.timestamp(),
            iat: iat.timestamp(),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct JwtResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

pub struct Jwt;

impl Jwt {
    pub fn generate<'a>(
        iss: &'a str,
        scope: &'a str,
        rsa_private_key: &openssl::pkey::PKey<openssl::pkey::Private>,
    ) -> Result<String> {
        let headers = JwtHeaders::new();
        let claims = JwtClaims::from(iss, scope);

        let headers_json = serde_json::to_string(&headers)
            .context("Error while converting token headers to json string")?;
        let claims_json = serde_json::to_string(&claims)
            .context("Error while converting token claims to json string")?;

        let headers_b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&headers_json);
        let claims_b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&claims_json);

        let mut token = format!("{}.{}", headers_b64, claims_b64);

        let mut signer =
            openssl::sign::Signer::new(openssl::hash::MessageDigest::sha256(), &rsa_private_key)
                .context("Error while creating signer for token")?;

        if let Err(err) = signer.update(token.as_bytes()) {
            return Err(anyhow!(err));
        };

        let signature = signer
            .sign_to_vec()
            .context("Error while converting signature")?;
        let signature = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(signature);

        token.push('.');
        token.push_str(&signature);
        Ok(token)
    }
}
