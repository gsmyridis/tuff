pub mod time;
pub use time::{Duration, Instant};

pub mod freq;
pub use freq::Frequency;

pub mod counter;
pub use counter::Counter;

pub enum Metric {
    OsClock,
    CpuCounter,
    CpuCounterSerialized,
}
