use super::credentials::Credentials;
use crate::{
    model::Resource,
    storage::{credentials::CredentialsScope, GOOGLE_API_BASE_URL, GOOGLE_API_UPLOAD_BASE_URL},
};
use anyhow::{anyhow, Context, Result};
use std::{io::Write, path::PathBuf};

pub struct Object;

impl Object {
    pub async fn metadata(
        credentials: &Credentials,
        bucket_name: &str,
        object_name: &str,
        client: &reqwest::Client,
    ) -> Result<Resource> {
        let object_name = object_name.replace("/", "%2F");
        let token = credentials
            .get_token(CredentialsScope::StorageReadOnly, client)
            .await?;

        let response = client
            .get(format!(
                "{}/b/{}/o/{}",
                GOOGLE_API_BASE_URL, bucket_name, object_name
            ))
            .bearer_auth(token)
            .send()
            .await
            .context("Error occurred while trying to fetch object's metadata")?;

        let response_json = match response.status().is_success() {
            true => response.json::<Resource>().await,
            false => {
                return Err(anyhow!(format!(
                    "Error while fetching buckets's object metadata. Status: {}",
                    response.status().as_u16()
                )));
            }
        };

        let resource = response_json.context("Error parsing response to object metadata")?;

        Ok(resource)
    }

    pub async fn get(
        credentials: &Credentials,
        bucket_name: &str,
        object_name: &str,
        destination: &PathBuf,
        client: &reqwest::Client,
    ) -> Result<()> {
        let object_name = object_name.replace("/", "%2F");
        let token = credentials
            .get_token(CredentialsScope::StorageReadOnly, client)
            .await?;

        let response = client
            .get(format!(
                "{}/b/{}/o/{}?alt=media",
                GOOGLE_API_BASE_URL, bucket_name, object_name
            ))
            .bearer_auth(token)
            .send()
            .await
            .context("Error occurred while trying to get object")?;

        let object_bytes = match response.status().is_success() {
            true => response.bytes().await,
            false => {
                return Err(anyhow!(format!(
                    "Error while fetching buckets's object metadata. Status: {}",
                    response.status().as_u16()
                )));
            }
        };

        let object_bytes = object_bytes.context("Error retriving object bytes")?;

        let mut file = std::fs::File::create(destination)
            .context("Error while file creation after geting object")?;
        file.write_all(&object_bytes)
            .context("Error while saving content of object")?;

        Ok(())
    }

    pub async fn upload(
        credentials: &Credentials,
        bucket_name: &str,
        object_name: &str,
        file_path: &PathBuf,
        client: &reqwest::Client,
    ) -> Result<()> {
        let object_name = object_name.replace("/", "%2F");
        let token = credentials
            .get_token(CredentialsScope::StorageReadWrite, client)
            .await?;

        let file = tokio::fs::File::open(file_path)
            .await
            .context("Error while opening file to upload")?;
        let stream = tokio_util::codec::FramedRead::new(file, tokio_util::codec::BytesCodec::new());
        let body = reqwest::Body::wrap_stream(stream);

        let response = client
            .post(format!(
                "{}/b/{}/o?uploadType=media&name={}",
                GOOGLE_API_UPLOAD_BASE_URL, bucket_name, object_name
            ))
            .bearer_auth(token)
            .header(reqwest::header::CONTENT_TYPE, "octet/stream")
            .body(body)
            .send()
            .await
            .context("Error while uploading object")?;

        if !response.status().is_success() {
            return Err(anyhow!(format!(
                "Error while uploading an object. Status: {}",
                response.status().as_u16()
            )));
        }

        Ok(())
    }

    pub async fn delete(
        credentials: &Credentials,
        bucket_name: &str,
        object_name: &str,
        client: &reqwest::Client,
    ) -> Result<()> {
        let object_name = object_name.replace("/", "%2F");
        let token = credentials
            .get_token(CredentialsScope::StorageReadWrite, client)
            .await?;

        let response = client
            .delete(format!(
                "{}/b/{}/o/{}",
                GOOGLE_API_BASE_URL, bucket_name, object_name
            ))
            .bearer_auth(token)
            .send()
            .await
            .context("Error while deleting object")?;

        if !response.status().is_success() {
            return Err(anyhow!(format!(
                "Error occurred during object deletion. Status: {}",
                response.status().as_u16()
            )));
        }

        Ok(())
    }
}
