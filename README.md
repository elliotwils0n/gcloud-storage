# gcloud-storage

Simple Api for Google Cloud Storage.

Features:
  - list bucket's objects,
  - reading object's metadata,
  - downloading objects,
  - uploading objects,
  - deletion of objects.

To use this library a service-account for GCP is required to authorize all of the operations.

Code example:

```rust
#[tokio::main]
fn main() -> Result<()> {
    // Credentials read from file
    let service_account_json_filepath = PathBuf::from("service/account/json/filepath");
    let storage = gcloud_storage::Storage::with_credentials_file(&service_account_json_filepath)?;

    // or credentials read from string
    //let service_account_json_str = "your_service_account_json_str";
    //let storage = gcloud_storage::Storage::with_credentials_str(service_account_json_str)?;

    // Get Bucket's objects list
    let objects_list: gcloud_storage::BucketObjectsList = storage
        .list_bucket_objects("bucket_name")
        .await?;

    // Get object's metadata
    let object_metadata: gcloud_storage::Resource = storage
        .get_object_metadata("bucket_name", "object_name")
        .await?;

    // Download object to desired destination
    let destination = PathBuf::from("/tmp/image.png");
    storage.get_object("bucket_name", "object_name", &destination).await?;


    // Upload file from filesystem
    let to_upload = PathBuf::from("/tmp/to_upload.tar.gz");
    storage.upload_object("bucket_name", "object_name", &to_upload).await?;

    // Delete object from bucket
    storage.delete_object("bucket_name", "object_name").await?;

    Ok(())
}
```
