use zoomin::Profiler;

#[test]
fn test_simple() {
    let mut profiler = Profiler::new();
    profiler.register_start();
    // profile! {
    let mut v = Vec::new();
    for i in 1..10 {
        v.push(i);
    }
    // }
    profiler.register_end();
    println!("{}", profiler.duration());

    let mut profiler = Profiler::new();
    profiler.register_start();
    let mut v = Vec::new();
    for i in 1..100000 {
        v.push(i);
    }
    profiler.register_end();
    println!("{}", profiler.duration());
}
