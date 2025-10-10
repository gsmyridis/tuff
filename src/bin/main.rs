use tuff::{read_os_time, ProfileBlock, Profiler};

fn main() {
    Profiler::start_global();

    for _ in 0..100_000 {
        tuff::profile_block! {["automatic_q", 1]
            let _z = read_os_time();
            for _ in 0..1000 {
                tuff::profile_block! {["automatic", 4]
                    let _x = std::time::Instant::now();
                };
            }
        };
    }

    for _ in 0..10_000_000 {
        let _x = ProfileBlock::new("os_time_mine", 5);
        let _z = read_os_time();
    }

    for _ in 0..10_000_000 {
        let _y = ProfileBlock::new("os_time", 6);
        let _z = std::time::Instant::now();
    }

    Profiler::stop_global();
    Profiler::report();
}
