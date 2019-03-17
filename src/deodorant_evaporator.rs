fn evaporator(content: f64, evap_per_day: i32, threshold: i32) -> i32 {
    let mut counter = 1;
    let mut a = (100 - evap_per_day) as f64 / 100.0;
    let c = a;
    let mut b = threshold as f64 / 100.0;
    while a > b {
        counter += 1;
    println!("{} {} {}", a,b,c);
        a *= c;
    }
    return counter;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(_content: f64, evap_per_day: i32, threshold: i32, exp: i32) -> () {
        println!(" evap_per_day: {:?}", evap_per_day);
        println!("threshold: {:?}", threshold);
        let ans = evaporator(_content, evap_per_day, threshold);
        println!(" actual:\n{:?}", ans);
        println!("expect:\n{:?}", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(10.0, 10, 10, 22);
        dotest(10.0, 10, 5, 29);
    }
}
