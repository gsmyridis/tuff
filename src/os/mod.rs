#[cfg(target_os = "macos")]
pub mod apple;

/// Returns an OS managed low resolution timer in nanoseconds.
pub fn read_os_time() -> u64 {
    #[cfg(target_os = "macos")]
    return crate::os::apple::mach_absolute_time_nanos();
}

#[cfg(test)]
mod test;
