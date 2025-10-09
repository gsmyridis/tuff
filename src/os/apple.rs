use mach2::mach_time;

pub unsafe fn mach_absolute_time() -> u64 {
    unsafe {
        // `mach_timebase_info` returns fraction to multiply a value in mach tick units
        // with to convert it to nanoseconds.
        //
        // Resource: https://developer.apple.com/documentation/driverkit/mach_timebase_info_t
        let mut timebase = mach_time::mach_timebase_info_data_t { numer: 0, denom: 0 };
        mach_time::mach_timebase_info(&mut timebase);
        let factor = timebase.numer as f64 / timebase.denom as f64;
        let mach_time = mach_time::mach_absolute_time() as f64;
        (mach_time * factor) as u64
    }
}
