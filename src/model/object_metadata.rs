use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
pub struct Retention {
    #[serde(rename(deserialize = "retainUntilTime"))]
    pub retain_until_time: Option<chrono::DateTime<chrono::Local>>,
    pub mode: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Owner {
    pub entity: Option<String>,
    #[serde(rename(deserialize = "entityId"))]
    pub entity_id: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CustomerEncryption {
    #[serde(rename(deserialize = "encryptionAlgorithm"))]
    pub encryption_algorithm: Option<String>,
    #[serde(rename(deserialize = "keySha256"))]
    pub key_sha_256: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ProjectTeam {
    #[serde(rename(deserialize = "projectNumber"))]
    pub project_number: Option<String>,
    pub team: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Acl {
    pub kind: String,
    pub object: Option<String>,
    pub generation: Option<String>,
    pub id: String,
    #[serde(rename(deserialize = "selfLink"))]
    pub self_link: Option<String>,
    pub bucket: Option<String>,
    pub entity: Option<String>,
    pub role: Option<String>,
    pub email: Option<String>,
    pub domain: Option<String>,
    #[serde(rename(deserialize = "entityId"))]
    pub entity_id: Option<String>,
    pub etag: Option<String>,
    #[serde(rename(deserialize = "projectTeam"))]
    pub project_team: Option<ProjectTeam>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Resource {
    pub kind: String,
    pub id: String,
    #[serde(rename(deserialize = "selfLink"))]
    pub self_link: Option<String>,
    #[serde(rename(deserialize = "mediaLink"))]
    pub media_link: Option<String>,
    pub name: String,
    pub bucket: String,
    pub generation: Option<String>,
    pub metageneration: Option<String>,
    #[serde(rename(deserialize = "contentType"))]
    pub content_type: Option<String>,
    #[serde(rename(deserialize = "storageClass"))]
    pub storage_class: Option<String>,
    pub size: String,
    #[serde(rename(deserialize = "md5Hash"))]
    pub md5_hash: String,
    #[serde(rename(deserialize = "contentEncoding"))]
    pub content_encoding: Option<String>,
    #[serde(rename(deserialize = "contentDisposition"))]
    pub content_disposition: Option<String>,
    #[serde(rename(deserialize = "contentLanguage"))]
    pub content_language: Option<String>,
    #[serde(rename(deserialize = "cacheControl"))]
    pub cache_control: Option<String>,
    pub crc32c: Option<String>,
    #[serde(rename(deserialize = "componentCount"))]
    pub component_count: Option<i32>,
    pub etag: Option<String>,
    #[serde(rename(deserialize = "kmsKeyName"))]
    pub kms_key_name: Option<String>,
    #[serde(rename(deserialize = "temporaryHold"))]
    pub temporary_hold: Option<bool>,
    #[serde(rename(deserialize = "eventBasedHold"))]
    pub event_based_hold: Option<bool>,
    #[serde(rename(deserialize = "retentionExpirationTime"))]
    pub retention_expiration_time: Option<chrono::DateTime<chrono::Local>>,
    pub retention: Option<Retention>,
    #[serde(rename(deserialize = "timeCreated"))]
    pub time_created: chrono::DateTime<chrono::Local>,
    pub updated: chrono::DateTime<chrono::Local>,
    #[serde(rename(deserialize = "timeDeleted"))]
    pub time_deleted: Option<chrono::DateTime<chrono::Local>>,
    #[serde(rename(deserialize = "timeStorageClassUpdated"))]
    pub time_storage_class_updated: Option<chrono::DateTime<chrono::Local>>,
    #[serde(rename(deserialize = "customTime"))]
    pub custom_time: Option<chrono::DateTime<chrono::Local>>,
    pub metadata: Option<HashMap<String, String>>,
    pub acl: Option<Vec<Acl>>,
    pub owner: Option<Owner>,
    #[serde(rename(deserialize = "customerEncryption"))]
    pub customer_encryption: Option<CustomerEncryption>,
}

#[derive(Debug, serde::Deserialize)]
pub struct BucketObjectsList {
    pub kind: String,
    #[serde(rename(deserialize = "nextPageToken"))]
    pub next_page_token: Option<String>,
    pub prefixes: Option<Vec<String>>,
    pub items: Vec<Resource>,
}
