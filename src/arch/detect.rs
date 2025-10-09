/// Checks whether the CPU has an invariant TSC.
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[inline]
pub fn has_counter_support() -> bool {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        use raw_cpuid::CpuId;
    }

    #[cfg(target_arch = "aarch64")]
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn test_aarch64_has_counter_support() {
        assert!(has_counter_support());
    }
}
