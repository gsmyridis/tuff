use std::io::{self, stdout, Write};
use tabwriter::{Alignment, TabWriter};

use super::fmt::{format_index, format_number, format_pct};
use crate::metrics::ProfileMetric;

#[derive(Debug, Clone, Copy)]
pub struct Measurement {
    /// Label to identify the profile block.
    pub(crate) label: &'static str,

    /// Number of times the anchor was hit.
    pub(crate) hit_count: u64,

    /// Metric elapsed not including children blocks.
    pub(crate) elapsed_exclusive: ProfileMetric,

    /// Metric elapsed including children blocks.
    pub(crate) elapsed_inclusive: ProfileMetric,

    /// Minimum elapsed metric for single execution.
    pub(crate) elapsed_min: ProfileMetric,

    /// Maximum elapsed metric for single execution.
    pub(crate) elapsed_max: ProfileMetric,
}

pub struct ProfileReport {
    metric_init: ProfileMetric,
    metric_final: ProfileMetric,
    measurements: Vec<Measurement>,
}

impl ProfileReport {
    pub(crate) fn new(metric_init: ProfileMetric, metric_final: ProfileMetric) -> Self {
        Self {
            metric_init,
            metric_final,
            measurements: Vec::new(),
        }
    }

    fn calculate_transpose(&self) -> Columns {
        use ProfileMetric::{CpuCounter, OsClock};

        let mut transpose = Columns::new();

        for meas in &self.measurements {
            transpose.hit_count.insert_value(meas.hit_count);

            match meas.elapsed_inclusive {
                CpuCounter(c) => {
                    transpose.elapsed_inclusive.insert_value(c.cycles());
                    let proportion = c.cycles() as f64 / self.total_metric() as f64 * 100.0;
                    transpose
                        .proportion_inclusive
                        .insert_value(proportion as u64);
                }
                OsClock(d) => {
                    transpose.elapsed_inclusive.insert_value(d.as_nanos());
                    let proportion = (d.as_nanos() as f64 / self.total_metric() as f64) * 100.0;
                    transpose
                        .proportion_inclusive
                        .insert_value(proportion as u64);
                }
            }

            match meas.elapsed_exclusive {
                CpuCounter(c) => {
                    transpose.elapsed_exclusive.insert_value(c.cycles());
                    transpose.proportion_exclusive.insert_value(
                        (c.cycles() as f64 / self.total_metric() as f64 * 100.0) as u64,
                    );
                }
                OsClock(d) => {
                    transpose.elapsed_exclusive.insert_value(d.as_nanos());
                    transpose.proportion_exclusive.insert_value(
                        (d.as_nanos() as f64 / self.total_metric() as f64 * 100.0) as u64,
                    );
                }
            }

            match meas.elapsed_min {
                CpuCounter(c) => transpose.elapsed_min.insert_value(c.cycles()),
                OsClock(d) => transpose.elapsed_min.insert_value(d.as_nanos()),
            }

            match meas.elapsed_max {
                CpuCounter(c) => transpose.elapsed_max.insert_value(c.cycles()),
                OsClock(d) => transpose.elapsed_max.insert_value(d.as_nanos()),
            }

            match (meas.elapsed_min, meas.elapsed_max) {
                (CpuCounter(min), CpuCounter(max)) => {
                    transpose.range.insert_value(max.cycles() - min.cycles())
                }
                (OsClock(min), OsClock(max)) => transpose
                    .range
                    .insert_value(max.as_nanos() - min.as_nanos()),
                _ => unimplemented!(),
            };
        }
        debug_assert_eq!(transpose.len(), self.measurements.len());
        transpose
    }

    fn total_metric(&self) -> u64 {
        use ProfileMetric::{CpuCounter, OsClock};

        match (self.metric_init, self.metric_final) {
            (OsClock(init), OsClock(fin)) => fin.as_nanos() - init.as_nanos(),
            (CpuCounter(init), CpuCounter(fin)) => fin.cycles() - init.cycles(),
            _ => unimplemented!("This should not be reached"),
        }
    }

    pub(crate) fn push_measurement(&mut self, meas: Measurement) {
        self.measurements.push(meas)
    }

    pub fn to_csv(&self, path: impl AsRef<std::path::Path>) -> io::Result<()> {
        let file = std::fs::File::create(path)?;
        let mut writer = std::io::BufWriter::new(file);
        writeln!(&mut writer, "label,elapsed_exc,elapsed_inc,minimum,maximum")?;
        for meas in self.measurements.iter() {
            writeln!(
                &mut writer,
                "{},{:?},{:?},{:?},{:?}",
                meas.label,
                meas.elapsed_exclusive,
                meas.elapsed_inclusive,
                meas.elapsed_min,
                meas.elapsed_max
            )?;
        }
        Ok(())
    }

    pub fn print(self) -> io::Result<()> {
        let transposed = self.calculate_transpose();

        let stdout = stdout().lock();
        let mut tabwriter = TabWriter::new(stdout).alignment(Alignment::Right);
        writeln!(
            &mut tabwriter,
            "\n{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            format_index("Label"),
            format_index("Elapsed Excl"),
            format_index("Proportion Excl"),
            format_index("Elapsed Incl"),
            format_index("Proportion Incl"),
            format_index("Minimum"),
            format_index("Maximum"),
            format_index("Range"),
        )
        .expect("Failed to create table column index");

        for i in 0..transposed.len() {
            writeln!(
                &mut tabwriter,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                format_index(self.measurements[i].label),
                format_number(
                    transposed.elapsed_exclusive.min_value,
                    transposed.elapsed_exclusive.max_value,
                    transposed.elapsed_exclusive.values[i]
                ),
                format_pct(
                    transposed.proportion_exclusive.min_value as f64,
                    transposed.proportion_exclusive.max_value as f64,
                    transposed.proportion_exclusive.values[i] as f64,
                ),
                format_number(
                    transposed.elapsed_inclusive.min_value,
                    transposed.elapsed_inclusive.max_value,
                    transposed.elapsed_inclusive.values[i]
                ),
                format_pct(
                    transposed.proportion_inclusive.min_value as f64,
                    transposed.proportion_inclusive.max_value as f64,
                    transposed.proportion_inclusive.values[i] as f64,
                ),
                format_number(
                    transposed.elapsed_min.min_value,
                    transposed.elapsed_min.max_value,
                    transposed.elapsed_min.values[i],
                ),
                format_number(
                    transposed.elapsed_max.min_value,
                    transposed.elapsed_max.max_value,
                    transposed.elapsed_max.values[i],
                ),
                format_number(
                    transposed.range.min_value,
                    transposed.range.max_value,
                    transposed.range.values[i],
                )
            )?;
        }
        tabwriter.flush()
    }
}

#[derive(Debug)]
struct Columns {
    hit_count: Column,
    elapsed_exclusive: Column,
    proportion_exclusive: Column,
    elapsed_inclusive: Column,
    proportion_inclusive: Column,
    elapsed_min: Column,
    elapsed_max: Column,
    range: Column,
}

impl Columns {
    fn new() -> Self {
        Self {
            hit_count: Column::new(),
            elapsed_exclusive: Column::new(),
            proportion_exclusive: Column::new(),
            elapsed_inclusive: Column::new(),
            proportion_inclusive: Column::new(),
            elapsed_min: Column::new(),
            elapsed_max: Column::new(),
            range: Column::new(),
        }
    }

    fn len(&self) -> usize {
        let len = self.hit_count.len();
        debug_assert!(
            self.elapsed_exclusive.len() == len
                && self.elapsed_inclusive.len() == len
                && self.proportion_inclusive.len() == len
                && self.proportion_exclusive.len() == len
                && self.elapsed_min.len() == len
                && self.elapsed_max.len() == len
                && self.range.len() == len
        );
        len
    }
}

#[derive(Debug)]
struct Column {
    values: Vec<u64>,
    min_value: u64,
    max_value: u64,
}

impl Column {
    fn new() -> Self {
        Self {
            values: Vec::new(),
            min_value: u64::MAX,
            max_value: 0,
        }
    }

    fn insert_value(&mut self, value: u64) {
        self.values.push(value);
        self.min_value = std::cmp::min(self.min_value, value);
        self.max_value = std::cmp::max(self.max_value, value);
    }

    fn len(&self) -> usize {
        self.values.len()
    }
}
