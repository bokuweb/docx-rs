use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReaderError {
    #[error("Failed to read from zip.")]
    ZipError(#[from] zip::result::ZipError),
    #[error("Failed to read xml.")]
    XMLReadError,
    #[error("Unknown error")]
    Unknown,
}
