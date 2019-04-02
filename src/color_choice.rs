fn check_choose(m: u64, n: u64) -> i64 {
    if m == 1 {
        return 0;
    }
    let mut res = n;
    let mut devider = 2;
    let mut count = 1;
    for timer in (1..n).rev() {
        if res == m {
            return count;
        } else if res > m {
            return -1;
        }
        res *= timer;
        res /= devider;
        // println!("{}\n", res);
        devider += 1;
        count += 1;
    }

    return -1;
}

fn dotest(m: u64, n: u64, exp: i64) -> () {
    assert_eq!(check_choose(m, n), exp)
}
#[test]
fn basics_check_choose() {
    dotest(6, 4, 2);
    dotest(4, 4, 1);
    dotest(35, 7, 3);
    dotest(36, 7, -1);
    dotest(184756, 20, 10);
}
