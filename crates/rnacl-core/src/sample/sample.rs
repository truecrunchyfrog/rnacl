use serde::{Deserialize, Serialize};

use crate::sample::trace::Trace;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Sample {
    traces: Vec<Trace>,
    content: String,
}

impl Sample {
    pub fn new(trace: Trace, content: String) -> Self {
        Self {
            traces: vec![trace],
            content,
        }
    }

    pub fn transform<F>(mut self, f: F) -> Self
    where
        F: FnOnce(String) -> (Trace, String),
    {
        let (trace, content) = f(self.content);
        self.traces.push(trace);
        self.content = content;

        self
    }
}
