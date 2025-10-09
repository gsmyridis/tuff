use super::read_os_time;

#[test]
fn test_os_timer() {
    let time_1 = read_os_time();
    let time_2 = read_os_time();
    assert!(time_2 >= time_1, "OS timer should be monotonic");
}

#[test]
fn test_os_timer_not_zero() {
    let time = read_os_time();
    assert!(time > 0, "OS timer should return a positive value");
}
