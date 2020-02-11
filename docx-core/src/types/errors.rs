use thiserror::Error;

#[derive(Error, Debug)]
pub enum TypeError {
    #[error("Failed to convert str to enum.")]
    FromStrError,
    #[error("Unknown error.")]
    Unknown,
}
