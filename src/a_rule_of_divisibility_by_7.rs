
fn seven(n: i64) -> (i64, i32) {
    let mut num = n;
    let mut count = 0;
    while num > 99 {
        count += 1;
        let h = num / 10;
        let l = num % 10;
        num = h - 2 * l;
    }
    (num, count)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(n: i64, exp: (i64, i32)) -> () {
        println!(" n: {:?};", n);
        let ans = seven(n);
        println!(" actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(477557101, (28, 7));
        dotest(477557102, (47, 7));
        dotest(1603, (7, 2));
    }
}
