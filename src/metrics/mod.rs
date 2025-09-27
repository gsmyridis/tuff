pub mod cpu;
pub use cpu::read_timestamp_counter;

pub mod os;
pub use os::{get_tick_frequency, get_time};
