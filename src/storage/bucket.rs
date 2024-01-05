use super::{
    credentials::{Credentials, CredentialsScope},
    GOOGLE_API_BASE_URL,
};
use crate::model::BucketObjectsList;
use anyhow::{anyhow, Context, Result};

pub struct Bucket;

impl Bucket {
    pub async fn list(
        credentials: &Credentials,
        bucket_name: &str,
        client: &reqwest::Client,
    ) -> Result<BucketObjectsList> {
        let token = credentials
            .get_token(CredentialsScope::StorageReadOnly, client)
            .await?;

        let response = client
            .get(format!("{}/b/{}/o", GOOGLE_API_BASE_URL, bucket_name))
            .bearer_auth(token)
            .send()
            .await
            .context("Error occured while fetching bucket's objects list.")?;

        let response_json = match response.status().is_success() {
            true => response.json::<BucketObjectsList>().await,
            false => {
                return Err(anyhow!(format!(
                    "Error while fetching buckets's object lists. Status: {}",
                    response.status().as_u16()
                )));
            }
        };

        let bucket_objects_list =
            response_json.context("Error while parsing response bucket's objects list")?;

        Ok(bucket_objects_list)
    }
}
