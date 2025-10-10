pub mod arch;
pub use arch::{read_cpu_counter, read_cpu_counter_frequency};

pub mod metrics;
pub use metrics::{Counter, Duration, Frequency, Instant};

pub mod os;
pub use os::read_os_time;

#[macro_use]
pub mod profile;
pub use profile::{ProfileBlock, Profiler};
