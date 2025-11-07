use mach2::mach_time;
use std::sync::OnceLock;

/// `mach_timebase_info` returns fraction to multiply a value in mach tick units
/// with to convert it to nanoseconds.
///
/// Resource: https://developer.apple.com/documentation/driverkit/mach_timebase_info_t
///
/// Panics when...
pub fn mach_timebase_info() -> (u32, u32) {
    static TIME_BASE: OnceLock<(u32, u32)> = OnceLock::new();
    *TIME_BASE.get_or_init(|| {
        let mut timebase = mach_time::mach_timebase_info_data_t { numer: 0, denom: 0 };
        let kr = unsafe { mach_time::mach_timebase_info(&mut timebase) };
        assert_eq!(kr, 0, "mach_timebase_info failed");
        assert!(timebase.denom != 0, "mach_timebase_info returned denom = 0");
        (timebase.numer, timebase.denom)
    })
}

pub fn mach_absolute_time_nanos() -> u64 {
    let ticks = unsafe { mach_time::mach_absolute_time() };
    let (numer, denom) = mach_timebase_info();
    let ns = (ticks as u128).saturating_mul(numer as u128) / (denom as u128);
    ns as u64
}
