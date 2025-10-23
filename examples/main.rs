use tuff::{os::apple::mach_timebase_info, Frequency};

fn main() {
    println!("{:?}", Frequency::read().in_gigas());

    println!("{:?}", mach_timebase_info());
}
