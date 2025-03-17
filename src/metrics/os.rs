#[cfg(target_os = "macos")]
use libc::{mach_absolute_time, mach_timebase_info, mach_timebase_info_data_t};

pub fn get_frequency() -> f64 {
    unsafe {
        let mut timebase = mach_timebase_info_data_t { numer: 0, denom: 0 };
        mach_timebase_info(&mut timebase);
        timebase.numer as f64 / timebase.denom as f64
    }
}

pub fn get_high_resolution_time() -> u64 {
    #[cfg(target_os = "macos")]
    {
        return 10000;
    }
}
