//! High-level file upload and download helpers.
//!
//! These wrap the low-level attachment initialize → PUT → finalize flow
//! into single method calls.

use std::path::Path;

use reqwest::header::CONTENT_TYPE;

use crate::client::TofuPilotClient;
use crate::error::{Error, Result};

/// Result of a completed file upload, containing the upload ID for linking
/// to runs/units and the signed download URL.
#[derive(Debug, Clone)]
pub struct UploadResult {
    /// Upload ID — pass this to `runs().update().attachments(vec![id])` or
    /// `units().update().attachments(vec![id])` to link the file.
    pub id: String,
    /// Signed URL for accessing the uploaded file.
    pub url: String,
}

/// Detect MIME content type from file extension.
fn content_type_for(filename: &str) -> &'static str {
    let ext = filename.rsplit('.').next().unwrap_or("");
    match ext.to_ascii_lowercase().as_str() {
        "pdf" => "application/pdf",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "csv" => "text/csv",
        "json" => "application/json",
        "xml" => "application/xml",
        "zip" => "application/zip",
        "txt" | "log" => "text/plain",
        "html" | "htm" => "text/html",
        _ => "application/octet-stream",
    }
}

impl TofuPilotClient {
    /// Upload a file from disk in one call.
    ///
    /// Performs the full three-step upload flow:
    /// 1. Initialize the upload to get a pre-signed URL
    /// 2. PUT the file bytes to the pre-signed URL
    /// 3. Finalize the upload
    ///
    /// Returns an [`UploadResult`] with the upload `id` (for linking to
    /// runs/units) and the signed `url` (for download).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use tofupilot::TofuPilotClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> tofupilot::Result<()> {
    ///     let client = TofuPilotClient::new("your-api-key");
    ///
    ///     // Upload a file
    ///     let upload = client.upload_file("report.pdf").await?;
    ///     println!("Uploaded {}, download: {}", upload.id, upload.url);
    ///
    ///     // Link it to a run
    ///     client.runs().update()
    ///         .id("run-id")
    ///         .attachments(vec![upload.id])
    ///         .send()
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn upload_file(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<UploadResult> {
        let path = path.as_ref();
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| Error::Validation(
                "could not extract file name from path".to_string(),
            ))?
            .to_string();

        let file_bytes = tokio::fs::read(path).await?;

        self.upload_bytes(&file_name, file_bytes).await
    }

    /// Upload raw bytes with a given file name.
    ///
    /// Same three-step flow as [`upload_file`](Self::upload_file), but
    /// accepts in-memory bytes instead of a file path.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use tofupilot::TofuPilotClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> tofupilot::Result<()> {
    ///     let client = TofuPilotClient::new("your-api-key");
    ///
    ///     let csv = b"col_a,col_b\n1,2\n3,4";
    ///     let upload = client.upload_bytes("data.csv", csv.to_vec()).await?;
    ///     println!("Uploaded {}", upload.id);
    ///     Ok(())
    /// }
    /// ```
    pub async fn upload_bytes(
        &self,
        file_name: &str,
        bytes: impl Into<Vec<u8>>,
    ) -> Result<UploadResult> {
        let bytes = bytes.into();

        // Step 1: Initialize
        let init = self
            .attachments()
            .initialize()
            .name(file_name)
            .send()
            .await?;

        // Step 2: PUT to pre-signed URL with Content-Type
        let mime = content_type_for(file_name);
        let put_response = reqwest::Client::new()
            .put(&init.upload_url)
            .header(CONTENT_TYPE, mime)
            .body(bytes)
            .send()
            .await?;

        if !put_response.status().is_success() {
            let status = put_response.status().as_u16();
            let body = put_response.text().await.unwrap_or_default();
            return Err(Error::UnexpectedStatus {
                status,
                body: format!("pre-signed URL upload failed: {body}"),
            });
        }

        // Step 3: Finalize
        let finalized = self
            .attachments()
            .finalize()
            .id(&init.id)
            .send()
            .await?;

        Ok(UploadResult {
            id: init.id,
            url: finalized.url,
        })
    }

    /// Download a file by URL to a local path.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use tofupilot::TofuPilotClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> tofupilot::Result<()> {
    ///     let client = TofuPilotClient::new("your-api-key");
    ///
    ///     client.download_file(
    ///         "https://storage.example.com/signed-url",
    ///         "local-report.pdf",
    ///     ).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn download_file(
        &self,
        url: &str,
        path: impl AsRef<Path>,
    ) -> Result<()> {
        if url.is_empty() {
            return Err(Error::Validation("download URL cannot be empty".to_string()));
        }

        let response = reqwest::Client::new().get(url).send().await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(Error::UnexpectedStatus {
                status,
                body: format!("download failed: {body}"),
            });
        }

        let bytes = response.bytes().await?;
        tokio::fs::write(path.as_ref(), &bytes).await?;
        Ok(())
    }
}
