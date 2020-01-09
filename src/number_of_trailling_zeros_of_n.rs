//fn zeros(n: u64) -> u64 {
//     println!("N -> {}", n);
//if n == 0 { 0 }
//else { n / 5 + zeros(n / 5) }
//}

fn cal(n: u64, step: u64) -> u64 {
    let mut step_e = 1;
    let mut count = 0;
    loop {
        let number = n / step.pow(step_e) - n / step.pow(step_e + 1);
        if number == 0 {
            break;
        }
        count += number * step_e as u64;
        step_e += 1;
    }
    count
}

fn zeros(n: u64) -> u64 {
    cal(n, 5)
}

#[test]
fn sample_tests() {
    assert_eq!(zeros(0), 0);
    assert_eq!(zeros(6), 1);
    assert_eq!(zeros(14), 2);
    assert_eq!(zeros(30), 7);
    assert_eq!(zeros(1000), 249);
    assert_eq!(zeros(100000), 24999);
    assert_eq!(zeros(1000000000), 249999998);
}
