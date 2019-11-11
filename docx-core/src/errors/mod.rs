use thiserror::Error;

#[derive(Error, Debug)]
pub enum DocxError {
    #[error("Failed to write XML to buffer.")]
    EmitterError(#[from] xml::writer::Error),
    #[error("Failed to zip XML documents.")]
    ZipError(#[from] zip::result::ZipError),
    #[error("Unknown error")]
    Unknown,
}
