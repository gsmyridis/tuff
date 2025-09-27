use tuff::{end_profiling, start_profiling, Block};

fn main() {
    start_profiling!();
    let mut v = Vec::new();
    for i in 0..100_000_000 {
        let block = Block::new("Body loop", 0);
        v.push(i);
        drop(block)
    }
    end_profiling!();
}
