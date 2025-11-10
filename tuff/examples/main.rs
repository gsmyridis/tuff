use tuff::{Frequency, os::apple::mach_timebase_info};
use tuff_macro::profile_fn;

pub struct Structure {
    x: u8,
}

impl Structure {
    #[profile_fn]
    pub unsafe fn f<'a>(&'a mut self) -> &'a u8 {
        &self.x
    }
}

#[profile_fn]
pub unsafe fn func_name<T, G>(_x: T, _y: G) -> u8 {
    0
}

fn main() {
    for _ in 1..=10 {
        tuff::profile_block! { [label="label", id=1]
            println!("{:?}", Frequency::read().in_gigas());
            println!("{}", tuff::arch::aarch64::cntpct_el0());
            println!("{}", tuff::arch::aarch64::cntvct_el0());
            println!("{:?}", mach_timebase_info());
        }
    }

    unsafe {
        func_name(1, 2);
    }

    let mut x = Structure { x: 10 };
    unsafe {
        x.f();
    }
}
