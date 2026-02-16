use thiserror::Error;

#[derive(Error, Debug)]
pub enum DocxError {
    #[error("FromStr error.{0}")]
    ConvertError(String),
    #[error("Failed to write XML to buffer.")]
    EmitterError(#[from] crate::xml::writer::Error),
    #[error("Failed to zip XML documents.")]
    ZipError(#[from] zip::result::ZipError),
    #[error("Unknown error")]
    Unknown,
}
