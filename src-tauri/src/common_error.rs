use std::sync::mpsc::RecvError;

#[derive(Debug, thiserror::Error)]
pub enum CommonError {
    #[error("Failed to read file: {0}")]
    Io(#[from] std::io::Error),
    #[error("File is not valid utf8: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("Seshat error: {0}")]
    Seshat(#[from] seshat::Error),
    #[error("RecvError seshat error: {0}")]
    RecvError(#[from] RecvError),
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error("{0}")]
    String(String),
    #[error("Unknown error")]
    Unknown,
}

// we must also implement serde::Serialize
impl serde::Serialize for CommonError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
