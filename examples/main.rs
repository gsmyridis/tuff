use tuff::{os::apple::mach_timebase_info, Frequency};

fn main() {
    println!("{:?}", Frequency::read().in_gigas());
    println!("{}", tuff::arch::aarch64::cntpct_el0());
    println!("{}", tuff::arch::aarch64::cntvct_el0());
    println!("{:?}", mach_timebase_info());
}
