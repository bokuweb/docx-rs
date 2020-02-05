use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReaderError {
    #[error("Failed to read from zip.")]
    ZipError(#[from] zip::result::ZipError),
    #[error("Failed to read xml.")]
    XMLReadError,
    #[error("Failed to find document.")]
    DocumentNotFoundError,
    #[error("Unknown error")]
    Unknown,
}
