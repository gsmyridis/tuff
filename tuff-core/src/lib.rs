pub mod arch;
pub use arch::{read_cpu_counter, read_cpu_counter_frequency};

pub mod os;
pub use os::read_os_time;
