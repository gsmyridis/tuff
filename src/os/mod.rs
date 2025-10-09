#[cfg(target_os = "macos")]
mod apple;

/// Returns an OS managed low resolution timer in nanoseconds.
pub fn read_os_time() -> u64 {
    #[cfg(target_os = "macos")]
    return unsafe { crate::os::apple::mach_absolute_time() };
}

#[cfg(test)]
mod test;
