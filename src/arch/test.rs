use super::{read_cpu_counter, read_cpu_counter_frequency};

#[test]
fn test_tsc_monotonic() {
    let t1 = read_cpu_counter();
    let t2 = read_cpu_counter();
    assert!(t2 >= t1, "TSC should be monotonic");
}

#[test]
fn test_tsc_not_zero() {
    let t = read_cpu_counter();
    assert!(t > 0, "TSC should return a positive value");
}

#[test]
fn test_tsc_frequency_constant() {
    let freq = read_cpu_counter_frequency();
    let freq2 = read_cpu_counter_frequency();
    assert_eq!(freq, freq2, "TSC frequency should remain constant");
}

#[test]
fn test_tsc_frequency_not_zero() {
    let freq = read_cpu_counter_frequency();
    assert!(freq > 0, "TSC frequency should return a positive value");
}
