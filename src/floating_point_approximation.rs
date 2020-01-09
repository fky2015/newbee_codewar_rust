fn interp(f: fn(f64) -> f64, l: f64, u: f64, n: i32) -> Vec<f64> {
    let d = (u - l) / n as f64;
    //    let mut base = l;
    //    let mut ret = Vec::new();
    //    for _ in 0..n {
    //        ret.push((f(base) * 100.0).floor() / 100.0);
    //        base += d
    //    }
    //    ret
    (0..n)
        .map(|i| (f(l + d * (i as f64)) * 100.0).floor() / 100.0)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(x: f64) -> f64 {
        return x;
    }

    fn sn(x: f64) -> f64 {
        return x.sin();
    }

    fn cs(x: f64) -> f64 {
        return x.cos();
    }

    fn dotest(fname: &str, f: fn(f64) -> f64, l: f64, u: f64, n: i32, exp: Vec<f64>) {
        println!("fct: {:?}", fname);
        println!("l: {:?}", l);
        println!("u: {:?}", u);
        println!("n: {:?}", n);
        let ans = interp(f, l, u, n);
        println!("{}", ans == exp);
        println!("actual:\n{:?}", ans);
        println!("expect:\n{:?}", exp);
        assert_eq!(ans, exp);
        println!("-");
    }

    #[test]
    fn basic_tests() {
        dotest("id", id, 0.0, 9.0, 4, vec![0.0, 2.25, 4.5, 6.75]);

        dotest(
            "sn",
            sn,
            0.0,
            18.0,
            12,
            vec![
                0.0, 0.99, 0.14, -0.98, -0.28, 0.93, 0.41, -0.88, -0.54, 0.8, 0.65, -0.72,
            ],
        );

        dotest(
            "cs",
            cs,
            1.0,
            21.0,
            7,
            vec![0.54, -0.76, 0.9, -0.99, 0.99, -0.92, 0.76],
        );
    }
}
