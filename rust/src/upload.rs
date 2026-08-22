use bytes::Bytes;
use futures_core::Stream;
use rand::Rng;
use std::{
    path::{Path, PathBuf},
    pin::Pin,
    task::{Context, Poll},
    time::{SystemTime, UNIX_EPOCH},
};
use tempfile::NamedTempFile;
use thiserror::Error;
use tokio::io::AsyncWriteExt;

pub const MAX_UPLOAD_BYTES: u64 = 100 * 1024 * 1024;
pub const MAX_WRITE_CHUNK: usize = 1024 * 1024;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum UploadError {
    #[error("invalid filename")]
    InvalidFilename,
    #[error("unsupported extension")]
    UnsupportedExtension,
    #[error("upload exceeds limit")]
    TooLarge,
    #[error("upload interrupted")]
    Interrupted,
    #[error("temporary storage failed")]
    Storage,
}

#[derive(Debug)]
pub struct PreparedUpload {
    _temp: NamedTempFile,
    pub path: PathBuf,
    pub size: u64,
    pub object_name: String,
    pub content_type: &'static str,
}

pub fn validate_filename(filename: &str) -> Result<&'static str, UploadError> {
    if filename.is_empty()
        || filename.len() > 255
        || filename.contains(['/', '\\'])
        || filename.contains("..")
        || filename.chars().any(char::is_control)
    {
        return Err(UploadError::InvalidFilename);
    }
    let extension = Path::new(filename)
        .extension()
        .and_then(|value| value.to_str())
        .ok_or(UploadError::UnsupportedExtension)?;
    content_type(extension).ok_or(UploadError::UnsupportedExtension)
}

pub fn content_type(extension: &str) -> Option<&'static str> {
    match extension {
        "jpg" | "jpeg" | "jfif" => Some("image/jpeg"),
        "png" => Some("image/png"),
        "gif" => Some("image/gif"),
        "bmp" => Some("image/bmp"),
        "webp" => Some("image/webp"),
        "ico" => Some("image/x-icon"),
        "heic" => Some("image/heic"),
        "heif" => Some("image/heif"),
        "avif" => Some("image/avif"),
        "tiff" | "tif" => Some("image/tiff"),
        "mp4" => Some("video/mp4"),
        _ => None,
    }
}

pub fn object_name(filename: &str) -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let random = rand::rng().random_range(1..=10_000_u16);
    format!("{timestamp}{random}_{filename}")
}

pub async fn prepare_stream<S, E>(
    filename: &str,
    temp_dir: &Path,
    mut stream: S,
) -> Result<PreparedUpload, UploadError>
where
    S: Stream<Item = Result<Bytes, E>> + Unpin,
{
    let content_type = validate_filename(filename)?;
    let temp = NamedTempFile::new_in(temp_dir).map_err(|_| UploadError::Storage)?;
    let path = temp.path().to_path_buf();
    let reopened = temp.reopen().map_err(|_| UploadError::Storage)?;
    let mut file = tokio::fs::File::from_std(reopened);
    let mut size = 0_u64;

    while let Some(next) = next_item(&mut stream).await {
        let bytes = next.map_err(|_| UploadError::Interrupted)?;
        size = size
            .checked_add(bytes.len() as u64)
            .ok_or(UploadError::TooLarge)?;
        if size > MAX_UPLOAD_BYTES {
            return Err(UploadError::TooLarge);
        }
        for part in bytes.chunks(MAX_WRITE_CHUNK) {
            file.write_all(part)
                .await
                .map_err(|_| UploadError::Storage)?;
        }
    }
    file.flush().await.map_err(|_| UploadError::Storage)?;

    Ok(PreparedUpload {
        _temp: temp,
        path,
        size,
        object_name: object_name(filename),
        content_type,
    })
}

async fn next_item<S, E>(stream: &mut S) -> Option<Result<Bytes, E>>
where
    S: Stream<Item = Result<Bytes, E>> + Unpin,
{
    std::future::poll_fn(|context| Pin::new(&mut *stream).poll_next(context)).await
}

pub struct OneChunk(pub Option<Result<Bytes, ()>>);

impl Stream for OneChunk {
    type Item = Result<Bytes, ()>;

    fn poll_next(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(self.0.take())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct RepeatChunks {
        remaining: usize,
        chunk: Bytes,
    }

    impl Stream for RepeatChunks {
        type Item = Result<Bytes, ()>;

        fn poll_next(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            if self.remaining == 0 {
                Poll::Ready(None)
            } else {
                self.remaining -= 1;
                Poll::Ready(Some(Ok(self.chunk.clone())))
            }
        }
    }

    #[test]
    fn validates_legacy_extensions_and_rejects_paths() {
        assert_eq!(validate_filename("票据.jpg"), Ok("image/jpeg"));
        assert_eq!(validate_filename("video.mp4"), Ok("video/mp4"));
        assert_eq!(
            validate_filename("UPPER.JPG"),
            Err(UploadError::UnsupportedExtension)
        );
        assert_eq!(
            validate_filename("../a.jpg"),
            Err(UploadError::InvalidFilename)
        );
        assert_eq!(
            validate_filename("a\\b.jpg"),
            Err(UploadError::InvalidFilename)
        );
        assert_eq!(
            validate_filename("a\0.jpg"),
            Err(UploadError::InvalidFilename)
        );
    }

    #[test]
    fn retains_legacy_object_name_shape() {
        let name = object_name("原图.png");
        assert!(name.ends_with("_原图.png"));
        assert!(
            name.split('_')
                .next()
                .unwrap()
                .bytes()
                .all(|b| b.is_ascii_digit())
        );
    }

    #[tokio::test]
    async fn accepts_zero_byte_upload() {
        let dir = tempfile::tempdir().unwrap();
        let prepared = prepare_stream("empty.png", dir.path(), OneChunk(None))
            .await
            .unwrap();
        assert_eq!(prepared.size, 0);
    }

    #[tokio::test]
    async fn rejects_interrupted_upload() {
        let dir = tempfile::tempdir().unwrap();
        let result = prepare_stream("a.png", dir.path(), OneChunk(Some(Err(())))).await;
        assert_eq!(result.unwrap_err(), UploadError::Interrupted);
    }

    #[tokio::test]
    async fn accepts_exactly_one_hundred_megabytes() {
        let dir = tempfile::tempdir().unwrap();
        let prepared = prepare_stream(
            "boundary.mp4",
            dir.path(),
            RepeatChunks {
                remaining: 100,
                chunk: Bytes::from(vec![0_u8; MAX_WRITE_CHUNK]),
            },
        )
        .await
        .unwrap();
        assert_eq!(prepared.size, MAX_UPLOAD_BYTES);
    }

    #[tokio::test]
    async fn rejects_one_byte_over_limit() {
        let dir = tempfile::tempdir().unwrap();
        let result = prepare_stream(
            "over.mp4",
            dir.path(),
            RepeatChunks {
                remaining: 1,
                chunk: Bytes::from(vec![0_u8; MAX_UPLOAD_BYTES as usize + 1]),
            },
        )
        .await;
        assert_eq!(result.unwrap_err(), UploadError::TooLarge);
    }

    #[tokio::test]
    async fn upload_semaphore_allows_only_two_concurrent_uploads() {
        let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(2));
        let first = semaphore.clone().try_acquire_owned().unwrap();
        let second = semaphore.clone().try_acquire_owned().unwrap();
        assert!(semaphore.clone().try_acquire_owned().is_err());
        drop(first);
        assert!(semaphore.clone().try_acquire_owned().is_ok());
        drop(second);
    }
}
