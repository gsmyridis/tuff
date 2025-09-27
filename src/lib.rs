#![feature(trace_macros)]

pub mod metrics;
pub use metrics::{
    cpu::{get_timestamp_counter_frequency, read_timestamp_counter},
    os::{get_tick_frequency, get_time},
};

pub mod detection;
pub use detection::has_counter_support;

#[macro_use]
pub mod profile;
pub use profile::{
    get_total_counter, start_global_profiler, stop_global_profiler, Anchor, Block, Profiler,
};
