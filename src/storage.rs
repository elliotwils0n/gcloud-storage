mod bucket;
mod credentials;
mod jwt;
mod object;

use self::{bucket::Bucket, credentials::Credentials, object::Object};
use crate::model::{BucketObjectsList, Resource};
use anyhow::Result;
use std::path::PathBuf;

const GOOGLE_API_BASE_URL: &str = "https://storage.googleapis.com/storage/v1";
const GOOGLE_API_UPLOAD_BASE_URL: &str = "https://storage.googleapis.com/upload/storage/v1";

pub struct Storage {
    client: reqwest::Client,
    credentials: Credentials,
}

impl Storage {
    pub fn with_credentials_file(credentials: &PathBuf) -> Result<Storage> {
        Ok(Storage {
            credentials: Credentials::from_file(credentials)?,
            client: reqwest::Client::new(),
        })
    }

    pub fn with_credentials_str(credentials: &str) -> Result<Storage> {
        Ok(Storage {
            credentials: Credentials::from_str(credentials)?,
            client: reqwest::Client::new(),
        })
    }

    pub async fn list_bucket_objects(&self, bucket_name: &str) -> Result<BucketObjectsList> {
        Bucket::list(&self.credentials, bucket_name, &self.client).await
    }

    pub async fn get_object_metadata(
        &self,
        bucket_name: &str,
        object_name: &str,
    ) -> Result<Resource> {
        Object::metadata(&self.credentials, bucket_name, object_name, &self.client).await
    }

    pub async fn get_object(
        &self,
        bucket_name: &str,
        object_name: &str,
        destination: &PathBuf,
    ) -> Result<()> {
        Object::get(
            &self.credentials,
            bucket_name,
            object_name,
            destination,
            &self.client,
        )
        .await
    }

    pub async fn upload_object(
        &self,
        bucket_name: &str,
        object_name: &str,
        file_path: &PathBuf,
    ) -> Result<()> {
        Object::upload(
            &self.credentials,
            bucket_name,
            object_name,
            file_path,
            &self.client,
        )
        .await
    }

    pub async fn delete_object(&self, bucket_name: &str, object_name: &str) -> Result<()> {
        Object::delete(&self.credentials, bucket_name, object_name, &self.client).await
    }
}
