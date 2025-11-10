use std::cell::RefCell;

use crate::arch::read_cpu_counter;
use crate::metrics::{Counter, Duration, MetricType, ProfileMetric};
use crate::os::read_os_time;
use crate::report::{Measurement, ProfileReport};

const PROFILER_SIZE: usize = 1024;

thread_local! {
    static THREAD_PROFILER: RefCell<Profiler> = RefCell::new(Profiler::new());
    static LOCAL_INDEX: RefCell<usize> = RefCell::new(1);
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct CallSite {
    file: &'static str,
    line: u32,
    column: u32,
}

impl CallSite {
    #[inline(always)]
    pub const fn new(file: &'static str, line: u32, column: u32) -> Self {
        Self { file, line, column }
    }
}

pub struct Profiler {
    current_open_block: usize,
    anchors: [ProfileAnchor; PROFILER_SIZE],

    metric_type: MetricType,
    metric_init: Option<u64>,
    metric_final: Option<u64>,
}

impl Profiler {
    fn new() -> Self {
        Self {
            current_open_block: 0,
            anchors: [ProfileAnchor::new("Uninit"); PROFILER_SIZE],
            metric_type: MetricType::OsClock,
            metric_init: Some(0),
            metric_final: None,
        }
    }

    pub fn next_id() -> usize {
        LOCAL_INDEX.with_borrow_mut(|index| {
            let current_index = *index;
            *index += 1;
            current_index
        })
    }

    pub fn start_global(metric_type: MetricType) {
        THREAD_PROFILER.with(|p| {
            let mut profiler = p.borrow_mut();
            profiler.metric_type = metric_type;
            profiler.metric_init = Some(profiler.read_current_metric());
        });
    }

    pub fn stop_global() {
        THREAD_PROFILER.with(|p| {
            let mut profiler = p.borrow_mut();
            profiler.metric_final = Some(profiler.read_current_metric());
        });
    }

    // TODO: Change this
    #[inline(always)]
    fn read_current_metric(&self) -> u64 {
        match self.metric_type {
            MetricType::OsClock => read_os_time(),
            MetricType::CpuCounter => read_cpu_counter(),
            MetricType::CpuCounterSerialized => todo!(),
        }
    }

    pub fn report() -> ProfileReport {
        THREAD_PROFILER.with(|p| {
            let profiler = p.borrow();
            let into_metric = |value: u64| match profiler.metric_type {
                MetricType::OsClock => ProfileMetric::OsClock(Duration::from_nanos(value)),
                MetricType::CpuCounter | MetricType::CpuCounterSerialized => {
                    ProfileMetric::CpuCounter(Counter::from_cycles(value))
                }
            };

            let metric_init_value = profiler.metric_init.expect("Profiler not started");
            let metric_final_value = profiler.metric_final.expect("Profiler not finished");

            let metric_init = into_metric(metric_init_value);
            let metric_final = into_metric(metric_final_value);

            let mut report = ProfileReport::new(metric_init, metric_final);
            for anchor in profiler.anchors.iter() {
                if anchor.hit_count == 0 {
                    continue;
                }
                let stat = Measurement {
                    label: anchor.label,
                    hit_count: anchor.hit_count,
                    elapsed_exclusive: into_metric(anchor.elapsed_exclusive as u64),
                    elapsed_inclusive: into_metric(anchor.elapsed_inclusive),
                    elapsed_min: into_metric(anchor.elapsed_min),
                    elapsed_max: into_metric(anchor.elapsed_max),
                };
                report.push_measurement(stat);
            }

            report
        })
    }
}

#[repr(align(64))]
#[derive(Debug, Clone, Copy)]
pub(crate) struct ProfileAnchor {
    /// Label to identify the profile block.
    label: &'static str,

    /// Number of times the anchor was hit.
    hit_count: u64,

    /// Metric elapsed not including children blocks.
    elapsed_exclusive: i64,

    /// Metric elapsed including children blocks.
    elapsed_inclusive: u64,

    /// Minimum elapsed metric for single execution.
    elapsed_min: u64,

    /// Maximum elapsed metric for single execution.
    elapsed_max: u64,
}

impl ProfileAnchor {
    const fn new(label: &'static str) -> Self {
        Self {
            label,
            hit_count: 0,
            elapsed_exclusive: 0,
            elapsed_inclusive: 0,
            elapsed_min: u64::MAX,
            elapsed_max: 0,
        }
    }
}

#[derive(Debug)]
pub struct ProfileBlock {
    anchor_index: usize,
    parent_index: usize,
    start_counter: u64,
    elapsed_inclusive_prev: u64,
}

impl ProfileBlock {
    pub fn new(label: &'static str, anchor_index: usize) -> Self {
        THREAD_PROFILER.with(|p| {
            let mut profiler = p.borrow_mut();
            let parent_index = profiler.current_open_block;
            profiler.current_open_block = anchor_index;
            {
                let anchor = &mut profiler.anchors[anchor_index];
                if anchor.hit_count == 0 {
                    anchor.label = label;
                }
            }
            let start_counter = profiler.read_current_metric();
            let elapsed_inclusive_prev = profiler.anchors[anchor_index].elapsed_inclusive;
            Self {
                anchor_index,
                parent_index,
                start_counter,
                elapsed_inclusive_prev,
            }
        })
    }
}

impl Drop for ProfileBlock {
    fn drop(&mut self) {
        THREAD_PROFILER.with(|p| {
            let mut profiler = p.borrow_mut();
            let elapsed = profiler.read_current_metric() - self.start_counter;

            let anchor = &mut profiler.anchors[self.anchor_index];
            anchor.hit_count += 1;
            anchor.elapsed_exclusive += elapsed as i64;
            anchor.elapsed_inclusive = self.elapsed_inclusive_prev + elapsed;
            anchor.elapsed_min = std::cmp::min(anchor.elapsed_min, elapsed);
            anchor.elapsed_max = std::cmp::max(anchor.elapsed_max, elapsed);

            // Account for nested calls
            profiler.current_open_block = self.parent_index;
            let parent = &mut profiler.anchors[self.parent_index];
            parent.elapsed_exclusive -= elapsed as i64;
        });
    }
}
