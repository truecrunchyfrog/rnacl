use std::path::PathBuf;

use crate::node::error::NodeError;

#[derive(thiserror::Error, Debug)]
pub enum LedgerError {
    #[error("ledger already initialized at {0}")]
    AlreadyExists(PathBuf),

    #[error("path already exists, but is not a directory: {0}")]
    PathNotDirectory(PathBuf),

    #[error(transparent)]
    Node(#[from] NodeError),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
