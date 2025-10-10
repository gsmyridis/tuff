#[inline(always)]
pub fn cntfrq_el0() -> u64 {
    let freq: u64;
    unsafe {
        core::arch::asm!(
            "mrs {freq}, cntfrq_el0",
            freq = lateout(reg) freq,
            options(nomem, nostack, preserves_flags),
        );
    }
    freq
}

#[inline(always)]
pub fn cntvct_el0() -> u64 {
    let cnt: u64;
    unsafe {
        core::arch::asm!(
            "mrs {cnt}, cntvct_el0",
            cnt = lateout(reg) cnt,
            options(nomem, nostack, preserves_flags),
        );
    }
    cnt
}
