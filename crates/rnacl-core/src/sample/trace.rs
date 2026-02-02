use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Trace(HashMap<String, Value>);

impl Trace {
    pub fn new(values: HashMap<String, Value>) -> Self {
        Self(values)
    }
}
