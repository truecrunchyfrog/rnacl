use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    pipeline::PipelineError,
    registry::{self},
    sample::batch::Batch,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Stage {
    operation_id: String,
    options: Value,
}

impl Stage {
    pub fn new(operation_id: String, options: Value) -> Self {
        Self {
            operation_id,
            options,
        }
    }

    pub fn operation_id(&self) -> &str {
        &self.operation_id
    }

    pub fn options(&self) -> &Value {
        &self.options
    }

    pub fn eval(&self, input: Batch) -> Result<Batch, PipelineError> {
        let operation = registry::resolve_op(&self.operation_id)?;
        Ok(operation.eval(input, &self.options)?)
    }
}
