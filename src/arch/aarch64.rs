pub unsafe fn _cntfrq_el0() -> u64 {
    let freq: u64;
    core::arch::asm!("mrs {}, CNTFRQ_EL0", out(reg) freq);
    freq
}

pub unsafe fn _cntvct_el0() -> u64 {
    let mut cnt: u64;
    core::arch::asm!("mrs {cnt}, CNTVCT_EL0", cnt = out(reg) cnt);
    cnt
}
