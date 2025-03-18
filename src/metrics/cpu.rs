#[cfg(target_arch = "x86")]
use core::arch::x86::_rdtsc;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::_rdtsc;

/// Reads the CPU timer, or timestamp counter (TSC).
///
/// Both in `x86`, and `aarch64` architectures, the TSC is a 64-bit register that
/// counts the number of cycles since the last reset.
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[inline]
pub fn get_timestamp_counter() -> u64 {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    // TODO: Add serialization instructions or rdtscp.
    return unsafe { _rdtsc() };

    #[cfg(target_arch = "aarch64")]
    {
        let mut cnt: u64;
        unsafe { core::arch::asm!("mrs {cnt}, CNTVCT_EL0", cnt = out(reg) cnt) };
        return cnt;
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
    panic!("Target architecture is not supported. File an issue on GitHub.");
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[inline]
pub fn get_timestamp_counter_frequency() -> u64 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let freq: u64;
        core::arch::asm!("mrs {}, CNTFRQ_EL0", out(reg) freq);
        freq
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tsc_monotonic() {
        let t1 = get_timestamp_counter();
        let t2 = get_timestamp_counter();
        assert!(t2 >= t1, "TSC should be monotonic");
    }

    #[test]
    fn test_tsc_not_zero() {
        let t = get_timestamp_counter();
        assert!(t > 0, "TSC should return a positive value");
    }

    #[test]
    fn test_tsc_frequency_constant() {
        let freq = get_timestamp_counter_frequency();
        let freq2 = get_timestamp_counter_frequency();
        assert_eq!(freq, freq2, "TSC frequency should remain constant");
    }

    #[test]
    fn test_tsc_frequency_not_zero() {
        let freq = get_timestamp_counter_frequency();
        assert!(freq > 0, "TSC frequency should return a positive value");
    }
}
