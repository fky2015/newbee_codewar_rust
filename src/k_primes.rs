fn is_kprimes(mut n: i32, k: i32) -> bool {
    let mut count = 0;
    for i in 2..((n as f64).sqrt() + 1.0) as i32 {
        while n >= 1 && n % i == 0 {
            n /= i;
            count += 1;
            if count > k {
                return false;
            }
        }
        if n == 1 {
            break;
        }
    }
    if n > 1 {
        count += 1;
    }
    count == k
}

fn count_kprimes(k: i32, start: i32, nd: i32) -> Vec<i32> {
    (start..=nd).filter(|&i| is_kprimes(i, k)).collect()
}

fn puzzle(s: i32) -> i32 {
    // your code
    let mut count = 0;
    for i in 1..s {
        if is_kprimes(i, 1) {
            for j in 8..s - i {
                if is_kprimes(j, 3) && is_kprimes(s - i - j, 7) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn testing_count_kprimes(k: i32, start: i32, nd: i32, exp: Vec<i32>) -> () {
    assert_eq!(count_kprimes(k, start, nd), exp)
}

#[test]
fn basics_count_kprimes() {
    // 1008 = 2 * 2 * 2 * 2 * 7 * 9
    // 1020 = 2 * 5 * 2 * 17 * 3
    testing_count_kprimes(
        5,
        1000,
        1100,
        vec![1020, 1026, 1032, 1044, 1050, 1053, 1064, 1072, 1092, 1100],
    );
    testing_count_kprimes(12, 100000, 100100, vec![]);
}

fn testing(n: i32, exp: i32) -> () {
    assert_eq!(puzzle(n), exp)
}

#[test]
fn basics_puzzle() {
    testing(100, 0);
    testing(144, 0);
    testing(143, 2);
}
