use thiserror::Error;

#[derive(Error, Debug)]
pub enum TypeError {
    #[error("Failed to convert str to enum.")]
    FromStrError,
    #[error("Failed to convert str. This is because {0} is unsupported")]
    Unsupported(String),
    #[error("Unknown error.")]
    Unknown,
}
