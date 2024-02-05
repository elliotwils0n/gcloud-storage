# gcloud-storage

### Description

Simple Api for Google Cloud Storage.

Features:
  - list bucket's objects,
  - reading object's metadata,
  - downloading objects,
  - uploading objects,
  - deletion of objects.

To use this library a service-account for GCP is required to authorize all of the operations.

### Example 

Cargo.toml
```toml
[dependencies]
gcloud-storage = { git = "https://github.com/elliotwils0n/gcloud-storage.git", branch = "master" }
tokio = { version = "1.35.0", features = [ "rt-multi-thread", "macros" ] }
anyhow = "1.0.77"
```

Code:
```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Credentials read from file
    let service_account_json_filepath = std::path::PathBuf::from("/path/to/credentials/json");
    let storage = gcloud_storage::Storage::with_credentials_file(&service_account_json_filepath)?;

    // or credentials read from string
    //let service_account_json_str = "your_service_account_json_str";
    //let storage = gcloud_storage::Storage::with_credentials_str(service_account_json_str)?;

    // Get Bucket's objects list
    let objects_list: gcloud_storage::BucketObjectsList = storage
        .list_bucket_objects("bucket-name")
        .await?;
    let objects_list = objects_list.items
        .into_iter()
        .map(|i| i.name)
        .collect::<Vec<String>>();
    println!("Before upload: {:?}", objects_list);

    // Upload file from filesystem
    let to_upload = std::path::PathBuf::from("../test.txt");
    storage.upload_object("bucket-name", "test.txt", &to_upload).await?;

    // Get Bucket's objects list
    let objects_list: gcloud_storage::BucketObjectsList = storage
        .list_bucket_objects("bucket-name")
        .await?;
    let objects_list = objects_list.items
        .into_iter()
        .map(|i| i.name)
        .collect::<Vec<String>>();
    println!("After upload: {:?}", objects_list);

    // Get object's metadata
    let object_metadata: gcloud_storage::Resource = storage
        .get_object_metadata("bucket-name", "test.txt")
        .await?;
    println!("Object metadata: {:#?}", object_metadata);

    // Download object to desired destination
    let destination = std::path::PathBuf::from("/tmp/test.txt");
    storage.get_object("bucket-name", "test.txt", &destination).await?;

    // Delete object from bucket
    storage.delete_object("bucket-name", "test.txt").await?;

    // Get Bucket's objects list
    let objects_list: gcloud_storage::BucketObjectsList = storage
        .list_bucket_objects("bucket-name")
        .await?;
    let objects_list = objects_list.items
        .into_iter()
        .map(|i| i.name)
        .collect::<Vec<String>>();
    println!("After deletion: {:?}", objects_list);

    Ok(())
}
```
