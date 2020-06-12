use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReaderError {
    #[error("Failed to read from zip.")]
    ZipError(#[from] zip::result::ZipError),
    #[error("Failed to parse int.")]
    NumError(#[from] std::num::ParseIntError),
    #[error("Failed to parse float.")]
    FloatError(#[from] std::num::ParseFloatError),
    #[error("Failed to convert type.")]
    TypeError(#[from] crate::types::TypeError),
    #[error("Failed to read xml.")]
    XMLReadError,
    #[error("Failed to find document.")]
    DocumentNotFoundError,
    #[error("Failed to find document rels.")]
    DocumentRelsNotFoundError,
    #[error("Failed to find styles.")]
    DocumentStylesNotFoundError,
    #[error("Failed to find numberings.")]
    DocumentNumberingsNotFoundError,
    #[error("Unknown error")]
    Unknown,
}
