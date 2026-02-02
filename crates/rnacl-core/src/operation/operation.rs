use crate::{operation::OperationError, sample::batch::Batch};

type OperationFunction =
    Box<dyn Fn(Batch, &serde_json::Value) -> Result<Batch, OperationError> + Send + Sync>;

pub struct Operation {
    description: String,
    function: OperationFunction,
}

impl Operation {
    pub fn new(description: String, function: OperationFunction) -> Self {
        Self {
            description,
            function,
        }
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn eval(&self, batch: Batch, options: &serde_json::Value) -> Result<Batch, OperationError> {
        (self.function)(batch, options)
    }
}
