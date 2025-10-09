use crate::os::read_os_time;

pub const NANOS_PER_SEC: u64 = 1_000_000_000;
pub const NANOS_PER_MILLI: u64 = 1_000_000;
pub const NANOS_PER_MICRO: u64 = 1_000;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant(pub u64);

impl Instant {
    pub fn now() -> Self {
        Self(read_os_time())
    }

    pub fn duration_since(&self, earlier: Instant) -> Duration {
        Duration(self.0 - earlier.0)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Duration(pub u64);

impl Duration {
    pub fn from_nsecs(nsecs: u64) -> Self {
        Self(nsecs)
    }

    pub const fn as_nanos(&self) -> u64 {
        self.0
    }

    pub fn as_micros(&self) -> u64 {
        (self.0 as f64 / NANOS_PER_MICRO as f64) as u64
    }

    pub fn as_millis(&self) -> u64 {
        (self.0 as f64 / NANOS_PER_MILLI as f64) as u64
    }

    pub fn as_secs(&self) -> u64 {
        (self.0 as f64 / NANOS_PER_SEC as f64) as u64
    }
}
