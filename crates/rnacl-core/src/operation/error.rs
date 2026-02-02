#[derive(thiserror::Error, Debug)]
pub enum OperationError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
