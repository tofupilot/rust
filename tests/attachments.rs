mod common;
use common::*;

#[tokio::test]
async fn initialize_returns_id_and_upload_url() {
    let result = client()
        .attachments()
        .initialize()
        .name(format!("test-{}.txt", uid()))
        .send()
        .await
        .unwrap();

    assert!(!result.id.is_empty());
    assert!(!result.upload_url.is_empty());
}

#[tokio::test]
async fn full_lifecycle_initialize_upload_finalize() {
    let c = client();

    // Step 1: Initialize
    let init = c
        .attachments()
        .initialize()
        .name(format!("lifecycle-{}.txt", uid()))
        .send()
        .await
        .unwrap();
    assert!(!init.upload_url.is_empty());

    // Step 2: PUT to pre-signed URL
    let put_response = reqwest::Client::new()
        .put(&init.upload_url)
        .header("Content-Type", "text/plain")
        .body("test content")
        .send()
        .await
        .unwrap();
    assert!(put_response.status().is_success());

    // Step 3: Finalize
    let finalized = c
        .attachments()
        .finalize()
        .id(&init.id)
        .send()
        .await
        .unwrap();
    assert!(!finalized.url.is_empty());
}

#[tokio::test]
async fn finalize_nonexistent_returns_not_found() {
    let result = client()
        .attachments()
        .finalize()
        .id(uuid::Uuid::new_v4().to_string())
        .send()
        .await;

    assert!(matches!(result, Err(tofupilot::Error::NotFound(_))));
}

#[tokio::test]
async fn upload_file_helper() {
    let dir = std::env::temp_dir().join(format!("tofupilot-test-{}", uid()));
    tokio::fs::create_dir_all(&dir).await.unwrap();
    let path = dir.join("upload-test.txt");
    tokio::fs::write(&path, "upload helper test content").await.unwrap();

    let upload = client().upload_file(&path).await.unwrap();

    assert!(!upload.id.is_empty());
    assert!(!upload.url.is_empty());

    // Cleanup
    tokio::fs::remove_dir_all(&dir).await.ok();
}

#[tokio::test]
async fn upload_bytes_helper() {
    let upload = client()
        .upload_bytes("data.csv", b"col_a,col_b\n1,2\n3,4".to_vec())
        .await
        .unwrap();

    assert!(!upload.id.is_empty());
    assert!(!upload.url.is_empty());
}

#[tokio::test]
async fn upload_and_download_roundtrip() {
    let original = format!("roundtrip test {}", uid());

    // Upload
    let upload = client()
        .upload_bytes("roundtrip.txt", original.as_bytes().to_vec())
        .await
        .unwrap();

    // Download
    let dir = std::env::temp_dir().join(format!("tofupilot-dl-{}", uid()));
    tokio::fs::create_dir_all(&dir).await.unwrap();
    let dest = dir.join("downloaded.txt");

    client()
        .download_file(&upload.url, &dest)
        .await
        .unwrap();

    let downloaded = tokio::fs::read_to_string(&dest).await.unwrap();
    assert_eq!(original, downloaded);

    // Cleanup
    tokio::fs::remove_dir_all(&dir).await.ok();
}

#[tokio::test]
async fn upload_nonexistent_file_returns_io_error() {
    let result = client().upload_file("/nonexistent/file.txt").await;
    assert!(matches!(result, Err(tofupilot::Error::Io(_))));
}

#[tokio::test]
async fn download_empty_url_returns_validation_error() {
    let result = client().download_file("", "/tmp/test.txt").await;
    assert!(matches!(result, Err(tofupilot::Error::Validation(_))));
}
