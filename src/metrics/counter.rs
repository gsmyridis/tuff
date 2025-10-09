use crate::arch::read_cpu_counter;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Counter(u64);

impl Counter {
    pub fn read() -> Self {
        let counter = read_cpu_counter();
        Self(counter)
    }

    pub fn read_serializing() -> Self {
        todo!()
    }
}
