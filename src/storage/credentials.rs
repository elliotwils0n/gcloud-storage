use super::jwt::{Jwt, JwtResponse};
use crate::model::ServiceAccount;
use anyhow::{anyhow, Context, Result};
use std::{fs, path::PathBuf};

const CREDENTIALS_AUTH_URL: &str = "https://oauth2.googleapis.com/token";
const CREDENTIALS_SCOPE_READ_ONLY: &str = "https://www.googleapis.com/auth/devstorage.read_only";
const CREDENTIALS_SCOPE_READ_WRITE: &str = "https://www.googleapis.com/auth/devstorage.read_write";

#[derive(Debug)]
pub enum CredentialsScope {
    StorageReadOnly,
    StorageReadWrite,
}

impl CredentialsScope {
    fn value(&self) -> &str {
        match self {
            CredentialsScope::StorageReadOnly => CREDENTIALS_SCOPE_READ_ONLY,
            CredentialsScope::StorageReadWrite => CREDENTIALS_SCOPE_READ_WRITE,
        }
    }
}

pub struct Credentials {
    issuer: String,
    rsa_private_key: openssl::pkey::PKey<openssl::pkey::Private>,
}

impl Credentials {
    pub fn from_file(service_account_json_path: &PathBuf) -> Result<Credentials> {
        let file_content = fs::read_to_string(service_account_json_path)
            .context("Error while reading Service Account json file")?;
        Self::from_str(file_content.as_str())
    }

    pub fn from_str(service_account_json_str: &str) -> Result<Credentials> {
        let service_account: ServiceAccount = serde_json::from_str(service_account_json_str)
            .context("Error while parsing Service Account json")?;
        let rsa_private_key =
            openssl::rsa::Rsa::private_key_from_pem(service_account.private_key.as_bytes())
                .context("Error while reading private key of service account")?;
        let rsa_private_key = openssl::pkey::PKey::from_rsa(rsa_private_key)
            .context("Error while reading private key of service account")?;

        Ok(Credentials {
            issuer: service_account.client_email,
            rsa_private_key,
        })
    }

    pub async fn get_token(
        &self,
        scope: CredentialsScope,
        client: &reqwest::Client,
    ) -> Result<String> {
        let jwt: String = Jwt::generate(&self.issuer, scope.value(), &self.rsa_private_key)?;

        let response = client
            .post(CREDENTIALS_AUTH_URL)
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .form(&vec![
                ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
                ("assertion", &jwt),
            ])
            .send()
            .await
            .context("Error while fetching token")?;

        let token = match response.status().is_success() {
            true => response.json::<JwtResponse>().await,
            false => {
                return Err(anyhow!(format!(
                    "Error while fetching token. Status: {}",
                    response.status().as_u16()
                )));
            }
        };

        Ok(token
            .context("Error while parsing token fetch response")?
            .access_token)
    }
}
