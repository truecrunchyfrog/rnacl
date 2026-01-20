#[derive(thiserror::Error, Debug)]
pub enum NodeError {
    #[error("node {0} already exists")]
    AlreadyExists(String),

    #[error("node {0} not found")]
    NotFound(String),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
