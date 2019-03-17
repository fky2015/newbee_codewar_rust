fn nb_dig(n: i32, d: i32) -> i32 {
    let d = &d.to_string()[..];
    (0..=n)
        .map(|x| x * x)
        .map(|x| x.to_string().matches(d).count() as i32)
        .sum()
}

//  (0..=n).map(|x| {
//         x.pow(2)
//             .to_string()
//             .chars()
//             .filter(|s| s.eq(&d_char))
//             .count()
//     }).sum::<usize>() as i32

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(n: i32, d: i32, exp: i32) -> () {
        println!("n: {:?}", n);
        println!("d: {:?}", d);
        let ans = nb_dig(n, d);
        println!("actual:\n{:?}", ans);
        println!("expect:\n{:?}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(550, 5, 213);
        dotest(5750, 0, 4700);
    }
}
