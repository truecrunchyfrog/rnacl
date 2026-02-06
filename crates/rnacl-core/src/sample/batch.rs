use serde::{Deserialize, Serialize};

use crate::sample::Sample;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Batch(Vec<Sample>);

impl Batch {
    pub fn new(samples: Vec<Sample>) -> Self {
        Self(samples)
    }

    pub fn transform<F>(self, f: F) -> Self
    where
        F: Fn(Sample) -> Option<Sample>,
    {
        Self(self.0.into_iter().filter_map(f).collect::<Vec<_>>())
    }

    pub fn into_samples(self) -> Vec<Sample> {
        self.0
    }

    pub fn samples(&self) -> &[Sample] {
        &self.0
    }

    pub fn samples_mut(&mut self) -> &mut Vec<Sample> {
        &mut self.0
    }
}

impl Default for Batch {
    fn default() -> Self {
        Self(Vec::new())
    }
}
