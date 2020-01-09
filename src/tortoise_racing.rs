extern crate chrono;
use chrono::Duration;

fn race(v1: i32, v2: i32, g: i32) -> Option<Vec<i32>> {
    if v2 <= v1 {
        None
    } else {
        let du = Duration::hours(g as i64);
        let du = du / (v2 - v1);
        Some(vec![
            du.num_hours() as i32,
            du.num_minutes() as i32 % (60),
            du.num_seconds() as i32 % 60,
        ])
    }
}

#[test]
fn basic_tests() {
    assert_eq!(race(720, 850, 70), Some(vec![0, 32, 18]));
    assert_eq!(race(80, 100, 40), Some(vec![2, 0, 0]));
    assert_eq!(race(80, 91, 37), Some(vec![3, 21, 49]));
    assert_eq!(race(820, 81, 550), None);
}
