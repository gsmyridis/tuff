use crate::{get_tick_frequency, read_timestamp_counter};

const GLOBAL_PROFILER_SIZE: usize = 4096;

static mut GLOBAL_PROFILER: Profiler<GLOBAL_PROFILER_SIZE> = Profiler::new("Global Profiler");

pub struct Profiler<const N: usize> {
    anchors: [Anchor; N],
    started: Option<u64>,
    ended: Option<u64>,

    #[allow(dead_code)]
    label: &'static str,
}

impl<const N: usize> Profiler<N> {
    const fn new(label: &'static str) -> Self {
        Self {
            started: Some(0),
            anchors: [Anchor::new("Uninit"); N],
            ended: None,
            label,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Anchor {
    time_elapsed: u64,
    hit_count: u64,
    label: &'static str,
}

impl Anchor {
    pub const fn new(label: &'static str) -> Self {
        Self {
            time_elapsed: 0,
            hit_count: 0,
            label,
        }
    }
}

#[derive(Debug)]
pub struct Block {
    anchor_index: usize,
    start_counter: u64,
    label: &'static str,
}

impl Block {
    pub fn new(label: &'static str, anchor_index: usize) -> Self {
        Self {
            label,
            anchor_index,
            start_counter: read_timestamp_counter(),
        }
    }
}

impl Drop for Block {
    fn drop(&mut self) {
        let elapsed = read_timestamp_counter() - self.start_counter;
        unsafe {
            GLOBAL_PROFILER.anchors[self.anchor_index].time_elapsed += elapsed;
            GLOBAL_PROFILER.anchors[self.anchor_index].hit_count += 1;
            GLOBAL_PROFILER.anchors[self.anchor_index].label = self.label;
        }
    }
}

pub fn start_global_profiler() {
    unsafe { GLOBAL_PROFILER.started = Some(read_timestamp_counter()) }
}

pub fn stop_global_profiler() {
    unsafe {
        GLOBAL_PROFILER.ended = Some(read_timestamp_counter());
        for anchor in &GLOBAL_PROFILER.anchors[..] {
            if anchor.hit_count == 0 {
                return;
            }
            println!("{anchor:?}");
            let avg_tsc = anchor.time_elapsed as f64 / anchor.hit_count as f64;
            println!("Average timestamp count: {avg_tsc}",);
            let avg_time = avg_tsc / get_tick_frequency();
            println!("Average time: {avg_time}");
        }
    }
}

pub fn get_total_counter() -> Option<u64> {
    unsafe {
        match (GLOBAL_PROFILER.started, GLOBAL_PROFILER.ended) {
            (Some(started), Some(ended)) => Some(ended - started),
            _ => None,
        }
    }
}

#[macro_export]
macro_rules! start_profiling {
    () => {
        $crate::start_global_profiler();
    };
}

#[macro_export]
macro_rules! end_profiling {
    () => {
        $crate::stop_global_profiler();
    };
}
