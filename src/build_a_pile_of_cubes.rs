fn func(n: i32) -> u64 {
    println!("in func: {}", n);
    let n = n as u64;
    if n & 1 != 0 {
        u64::pow(n, 2) * u64::pow(n + 1, 2) / 4
    } else {
        u64::pow(n, 2) / 4 * u64::pow(n + 1, 2)
    }
}


fn find_nb(n: u64) -> i32 {
    let mut max: i32 = 94000;
    let mut min: i32 = 0;
    while max > min {
        let mid = (max + min) / 2;
        let ans = func(mid);
        if ans < n {
            min = mid + 1;
        }
        if ans > n {
            max = mid - 1;
        }
        if ans == n{
            return mid;
        }
    }
    if func(max) == n {
        max
    } else {
        -1
    }
}

fn testing(n: u64, exp: i32) -> () {
    assert_eq!(find_nb(n), exp);
}

#[test]
fn basics_find_nb() {
    testing(4183059834009, 2022);
    testing(24723578342962, -1);
    testing(135440716410000, 4824);
    testing(40539911473216, 3568);
    testing(26825883955641, 3218);
}