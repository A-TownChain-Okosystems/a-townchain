use std::time::{Duration, Instant};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Counter {
    pub name: String,
    pub value: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Sample {
    pub name: String,
    pub duration: Duration,
}
#[derive(Default)]
pub struct Profiler {
    active: Vec<(String, Instant)>,
    samples: Vec<Sample>,
}
impl Profiler {
    pub fn begin(&mut self, name: impl Into<String>) {
        self.active.push((name.into(), Instant::now()));
    }
    pub fn end(&mut self) -> Option<Sample> {
        let (name, start) = self.active.pop()?;
        let sample = Sample {
            name,
            duration: start.elapsed(),
        };
        self.samples.push(sample.clone());
        Some(sample)
    }
    pub fn samples(&self) -> &[Sample] {
        &self.samples
    }
}
