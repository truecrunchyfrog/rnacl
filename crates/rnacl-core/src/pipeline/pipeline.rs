use serde::{Deserialize, Serialize};

use crate::pipeline::stage::Stage;

#[derive(Serialize, Deserialize, Debug)]
pub struct Pipeline(pub Vec<Stage>);
