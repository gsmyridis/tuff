use crate::arch::read_cpu_counter;

const GLOBAL_PROFILER_SIZE: usize = 1024;

static mut GLOBAL_PROFILER: Profiler = Profiler::new("Global Profiler");

pub struct Profiler {
    label: &'static str,
    anchors: [ProfileAnchor; GLOBAL_PROFILER_SIZE],
    current_open_block: usize,
    metric_init: Option<u64>,
    metric_final: Option<u64>,
}

impl Profiler {
    const fn new(label: &'static str) -> Self {
        Self {
            metric_init: Some(0),
            anchors: [ProfileAnchor::new("Uninit"); GLOBAL_PROFILER_SIZE],
            current_open_block: 0,
            metric_final: None,
            label,
        }
    }

    fn metric_elapsed_inclusive(anchor_index: usize) -> u64 {
        unsafe { GLOBAL_PROFILER.anchors[anchor_index].elapsed_inclusive }
    }

    #[cfg(feature = "profiling")]
    pub fn start_global() {
        unsafe { GLOBAL_PROFILER.metric_init = Some(read_cpu_counter()) }
    }

    #[cfg(not(feature = "profiling"))]
    pub fn start_global() {
        // no-op
    }

    #[cfg(feature = "profiling")]
    pub fn stop_global() {
        unsafe {
            GLOBAL_PROFILER.metric_final = Some(read_cpu_counter());
        }
    }

    #[cfg(not(feature = "profiling"))]
    pub fn stop_global() {
        // no-op
    }

    #[cfg(feature = "profiling")]
    pub fn report() {
        let mut sum = 0_u64;

        unsafe {
            let x = GLOBAL_PROFILER.label;
            println!("Profiler: {x}");

            let metric_init = GLOBAL_PROFILER
                .metric_init
                .expect("Profiling did not begin");
            let metric_final = GLOBAL_PROFILER
                .metric_final
                .expect("Profiling did not finish");
            let total_metric = metric_final - metric_init;
            println!("  - Total metric: {total_metric}");

            for anchor in &GLOBAL_PROFILER.anchors[..] {
                if anchor.hit_count == 0 {
                    continue;
                }

                sum += anchor.elapsed_exclusive as u64;

                // let average_metric = anchor.metric_elapsed_inclusive / anchor.hit_count;
                let proportion_inclusive =
                    anchor.elapsed_inclusive as f64 / total_metric as f64 * 100.0;
                let proportion_exclusive =
                    anchor.elapsed_exclusive as u64 as f64 / total_metric as f64 * 100.0;

                println!("- {}", anchor.label);
                println!("  - Total hit-count: {}", anchor.hit_count);
                println!("  - Metric inclusive: {}", anchor.elapsed_inclusive);
                println!("  - Proportion inclusive: {proportion_inclusive}");
                println!("  - Metric exclusive: {}", anchor.elapsed_exclusive);
                println!("  - Proportion exclusive: {proportion_exclusive}");
            }
            println!("{}", total_metric - sum);
        }
    }

    #[cfg(not(feature = "profiling"))]
    pub fn report() {
        // no-op
    }
}

#[derive(Debug, Clone, Copy)]
struct ProfileAnchor {
    /// Label to identify the profile block.
    label: &'static str,

    /// Number of times the anchor was hit.
    hit_count: u64,

    /// Metric elapsed not including children blocks.
    elapsed_exclusive: i64,

    /// Metric elapsed including children blocks.
    elapsed_inclusive: u64,
}

impl ProfileAnchor {
    const fn new(label: &'static str) -> Self {
        Self {
            label,
            hit_count: 0,
            elapsed_exclusive: 0,
            elapsed_inclusive: 0,
        }
    }
}

#[derive(Debug)]
pub struct ProfileBlock {
    label: &'static str,
    anchor_index: usize,
    parent_index: usize,
    start_counter: u64,
    elapsed_inclusive_prev: u64,
}

impl ProfileBlock {
    pub fn new(label: &'static str, anchor_index: usize) -> Self {
        unsafe {
            let parent_index = GLOBAL_PROFILER.current_open_block;
            GLOBAL_PROFILER.current_open_block = anchor_index;
            Self {
                label,
                anchor_index,
                parent_index,
                start_counter: read_cpu_counter(),
                elapsed_inclusive_prev: Profiler::metric_elapsed_inclusive(anchor_index),
            }
        }
    }
}

impl Drop for ProfileBlock {
    fn drop(&mut self) {
        let elapsed = read_cpu_counter() - self.start_counter;
        unsafe {
            // TODO: Why do I need to write it when dropping and not only when creating the block?
            let anchor = &mut GLOBAL_PROFILER.anchors[self.anchor_index];
            anchor.label = self.label;
            anchor.elapsed_exclusive += elapsed as i64;
            anchor.hit_count += 1;
            anchor.elapsed_inclusive = self.elapsed_inclusive_prev + elapsed;

            // Account for nested calls
            GLOBAL_PROFILER.current_open_block = self.parent_index;
            let parent = &mut GLOBAL_PROFILER.anchors[self.parent_index];
            parent.elapsed_exclusive -= elapsed as i64
        }
    }
}
