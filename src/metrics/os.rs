#[cfg(target_os = "macos")]
use mach2::mach_time::{mach_absolute_time, mach_timebase_info, mach_timebase_info_data_t};

pub fn get_tick_frequency() -> f64 {
    #[cfg(target_os = "macos")]
    {
        unsafe {
            let mut timebase = mach_timebase_info_data_t { numer: 0, denom: 0 };
            mach_timebase_info(&mut timebase);
            timebase.numer as f64 / timebase.denom as f64
        }
    }

    #[cfg(target_os = "windows")]
    {
        todo!("Windows is not supported yet")
    }

    #[cfg(target_os = "linux")]
    {
        todo!("Linux is not supported yet")
    }
}

pub fn get_time() -> f64 {
    #[cfg(target_os = "macos")]
    unsafe {
        let freq = get_tick_frequency();
        mach_absolute_time() as f64 * freq
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_os_frequency() {
        let freq = get_tick_frequency();
        assert!(freq > 0.0, "OS frequency should be positive");

        let freq2 = get_tick_frequency();
        assert!(freq2 > 0.0, "OS frequency should be positive");

        // I am not sure if this is true, but test passes on my machine.
        assert_eq!(freq, freq2, "OS frequency remains the same")
    }

    #[test]
    fn test_os_timer() {
        let time_1 = get_time();
        let time_2 = get_time();
        assert!(time_2 >= time_1, "OS timer should be monotonic");
    }

    #[test]
    fn test_os_timer_not_zero() {
        let time = get_time();
        assert!(time > 0.0, "OS timer should return a positive value");
    }
}
