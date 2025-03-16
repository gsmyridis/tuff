#[cfg(target_arch = "x86")]
use core::arch::x86::_rdtsc;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::_rdtsc;

#[cfg(target_arch = "aarch64")]
todo!();

#[inline]
pub fn read_cpu_timer() -> u64 {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        return unsafe { _rdtsc() };
    }

    #[cfg(target_arch = "aarch64")]
    {
        todo!()
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
    {
        panic!("Target architecture is not supported");
    }
}

mod tests {

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[cfg(test)]
    mod tests {
        use crate::metrics::read_cpu_timer;

        #[test]
        fn test_rdtsc_monotonic() {
            let t1 = read_cpu_timer();
            let t2 = read_cpu_timer();
            assert!(t2 >= t1, "TSC should be monotonic");
        }

        #[test]
        fn test_rdtsc_not_zero() {
            let t = read_cpu_timer();
            assert!(t > 0, "TSC should return a positive value");
        }
    }
}
