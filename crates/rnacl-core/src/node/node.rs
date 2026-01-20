use serde::{Deserialize, Serialize};

use crate::pipeline::Pipeline;

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    #[serde(skip)]
    pub id: String,
    pub description: Option<String>,
    pub pipeline: Pipeline,
}
