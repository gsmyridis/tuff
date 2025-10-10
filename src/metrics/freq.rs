use crate::arch::read_cpu_counter_frequency;

const GIGAS_IN_HERTZ: u64 = 1_000_000_000;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frequency(u64);

impl Frequency {
    pub fn read() -> Self {
        let freq = read_cpu_counter_frequency();
        Self(freq)
    }

    pub fn from_hertz(freq: u64) -> Self {
        Self(freq)
    }

    pub fn from_gigas(freq: u64) -> Self {
        Self(freq * GIGAS_IN_HERTZ)
    }

    pub fn in_hertz(self) -> u64 {
        self.0
    }

    pub fn in_gigas(self) -> u64 {
        self.0 / GIGAS_IN_HERTZ
    }
}
