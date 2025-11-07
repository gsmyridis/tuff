pub mod time;
pub use time::{Duration, Instant, TimeUnit};

pub mod freq;
pub use freq::Frequency;

pub mod counter;
pub use counter::Counter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricType {
    OsClock,
    CpuCounter,
    CpuCounterSerialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileMetric {
    OsClock(Duration),
    CpuCounter(Counter),
}
