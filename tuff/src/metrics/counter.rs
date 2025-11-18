use crate::arch::read_cpu_counter;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Counter(u64);

impl Counter {
    pub fn from_cycles(cycles: u64) -> Self {
        Self(cycles)
    }

    pub fn cycles(&self) -> u64 {
        self.0
    }

    pub fn read() -> Self {
        let counter = read_cpu_counter();
        Self(counter)
    }

    pub fn read_serializing() -> Self {
        todo!()
    }
}

impl Add for Counter {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl Sub for Counter {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}
