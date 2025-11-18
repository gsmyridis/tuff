use std::ops::{Add, Sub};

use crate::os::read_os_time;

pub const NANOS_PER_SEC: u64 = 1_000_000_000;
pub const NANOS_PER_MILLI: u64 = 1_000_000;
pub const NANOS_PER_MICRO: u64 = 1_000;

pub enum TimeUnit {
    Minutes,
    Seconds,
    Milliseconds,
    Nanoseconds,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant(u64);

impl Instant {
    pub fn now() -> Self {
        Self(read_os_time())
    }

    pub fn duration_since(&self, earlier: Instant) -> Duration {
        Duration(self.0 - earlier.0)
    }
}

/// TODO: Change duration to u128
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Duration(u64);

impl Duration {
    pub fn from_nanos(nsecs: u64) -> Self {
        Self(nsecs)
    }

    pub const fn as_nanos(&self) -> u64 {
        self.0
    }

    pub fn as_micros(&self) -> u64 {
        self.0 / NANOS_PER_MICRO
    }

    pub fn as_millis(&self) -> u64 {
        self.0 / NANOS_PER_MILLI
    }

    pub fn as_secs(&self) -> u64 {
        self.0 / NANOS_PER_SEC
    }
}

impl Add for Duration {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl Sub for Duration {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}
