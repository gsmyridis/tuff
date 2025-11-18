#[macro_use]
pub mod profile;
pub use profile::{CallSite, ProfileBlock, Profiler};

pub mod report;

pub mod metrics;
pub use metrics::{Counter, Duration, Frequency, Instant};

// Re-exports
pub use paste;
pub use tuff_core::*;
pub use tuff_macro::profile_fn;
