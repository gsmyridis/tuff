pub mod cpu;
pub use cpu::get_timestamp_counter;

pub mod os;
pub use os::{get_tick_frequency, get_time};
