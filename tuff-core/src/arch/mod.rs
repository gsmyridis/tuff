#[cfg(target_arch = "aarch64")]
pub mod aarch64;

pub mod detect;

#[cfg(test)]
mod test;

#[cfg(target_arch = "x86")]
use core::arch::x86::_rdtsc;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::_rdtsc;

/// Returns the frequency of the CPU counter.
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[inline]
pub fn read_cpu_counter_frequency() -> u64 {
    #[cfg(target_arch = "aarch64")]
    return crate::arch::aarch64::cntfrq_el0();
}

/// Reads the CPU timer, or timestamp counter (TSC).
///
/// Both in `x86`, and `aarch64` architectures, the TSC is a 64-bit register that
/// counts the number of cycles since the last reset.
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[inline]
pub fn read_cpu_counter() -> u64 {
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
    panic!("Target architecture is not supported. File an issue on GitHub.");

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    return unsafe { _rdtsc() };

    #[cfg(target_arch = "aarch64")]
    return crate::arch::aarch64::cntvct_el0();
}
