use crate::arch::read_cpu_counter;

const GLOBAL_PROFILER_SIZE: usize = 1024;

static mut GLOBAL_PROFILER: Profiler = Profiler::new("Global Profiler");

pub struct Profiler {
    pub anchors: [ProfileAnchor; GLOBAL_PROFILER_SIZE],
    pub started: Option<u64>,
    pub ended: Option<u64>,

    #[allow(dead_code)]
    pub label: &'static str,
}

#[cfg(feature = "profiling")]
impl Profiler {
    pub const fn new(label: &'static str) -> Self {
        Self {
            started: Some(0),
            anchors: [ProfileAnchor::new("Uninit"); GLOBAL_PROFILER_SIZE],
            ended: None,
            label,
        }
    }

    pub fn start_global() {
        unsafe { GLOBAL_PROFILER.started = Some(read_cpu_counter()) }
    }

    pub fn stop_global() {
        unsafe {
            GLOBAL_PROFILER.ended = Some(read_cpu_counter());
            for anchor in &GLOBAL_PROFILER.anchors[..] {
                if anchor.hit_count == 0 {
                    continue;
                }
                let avg_tsc = anchor.time_elapsed as f64 / anchor.hit_count as f64;
                let avg_time = avg_tsc;

                println!("- {}", anchor.label);
                println!("  - Total hitcount: {}", anchor.hit_count);
                println!("  - Total timestamp count: {}", anchor.time_elapsed);
                println!("  - Total time: {} ns", anchor.time_elapsed as f64);
                println!("  - Average timestamp count: {avg_tsc}");
                println!("  - Average time: {avg_time} ns\n");
            }
        }
    }
}

#[cfg(not(feature = "profiling"))]
impl Profiler {
    pub const fn new(label: &'static str) -> Self {
        Self {
            started: None,
            anchors: [ProfileAnchor::new("Uninit"); GLOBAL_PROFILER_SIZE],
            ended: None,
            label,
        }
    }

    pub fn start_global() {}

    pub fn stop_global() {}
}

#[derive(Debug, Clone, Copy)]
pub struct ProfileAnchor {
    pub time_elapsed: u64,
    pub hit_count: u64,
    pub label: &'static str,
}

impl ProfileAnchor {
    pub const fn new(label: &'static str) -> Self {
        Self {
            time_elapsed: 0,
            hit_count: 0,
            label,
        }
    }
}

#[derive(Debug)]
pub struct ProfileBlock {
    pub anchor_index: usize,
    pub start_counter: u64,
    pub label: &'static str,
}

impl ProfileBlock {
    pub fn new(label: &'static str, anchor_index: usize) -> Self {
        Self {
            label,
            anchor_index,
            start_counter: read_cpu_counter(),
        }
    }
}

impl Drop for ProfileBlock {
    fn drop(&mut self) {
        let elapsed = read_cpu_counter() - self.start_counter;
        unsafe {
            GLOBAL_PROFILER.anchors[self.anchor_index].time_elapsed += elapsed;
            GLOBAL_PROFILER.anchors[self.anchor_index].hit_count += 1;
            GLOBAL_PROFILER.anchors[self.anchor_index].label = self.label;
        }
    }
}

static mut INDEX: usize = 1;

/// Returns the next unique anchor index.
pub fn next_anchor_index() -> usize {
    unsafe {
        let current_index = INDEX;
        INDEX += 1;
        current_index
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

#[cfg(feature = "profiling")]
#[macro_export]
macro_rules! profile_block {
    ([ $label: literal, $index:expr ] $($body:tt)*) => {
        paste::paste! {
            let [<__profile_block _ $label _ $index>] = $crate::ProfileBlock::new($label, $index);
            $($body)*
            drop([<__profile_block _ $label _ $index>]);
        }
    };
}

#[cfg(not(feature = "profiling"))]
#[macro_export]
macro_rules! profile_block {
    ([ $label: literal, $index:expr ] $($body:tt)*) => {
        $($body)*
    };
}
