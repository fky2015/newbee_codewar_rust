fn func(x: f64) -> f64 {
    x.powi(2)
}

fn dist(y1: f64, y2: f64, step: f64) -> f64 {
    ((y1 - y2).powi(2) + step.powi(2)).sqrt()
}

fn len_curve(n: i32) -> f64 {
    let step: f64 = 1.0 / n as f64;
    let mut len: f64 = 0.0;
    let mut last_y = 0.0;
    let mut x = 0.0;
    for i in 0..n {
        x += step;
        len += dist(func(x), last_y, step);
        last_y = func(x);
    }

    len
}

fn assert_fuzzy_equals(actual: f64, expected: f64) {
    let merr = 1.0e-6;
    let inrange = if expected == 0.0 {
        (actual.abs() <= merr)
    } else {
        ((actual - expected).abs() / expected <= merr)
    };
    if inrange == false {
        println!(
            "Expected value must be near: {:e} but was:{:e}",
            expected, actual
        );
    } else {

    }
    assert_eq!(true, inrange);
}

fn dotest(n: i32, exp: f64) -> () {
    assert_fuzzy_equals(len_curve(n), exp);
}

#[test]
fn basic_tests_len_curve() {
    dotest(1, 1.414213562);
    dotest(10, 1.478197397);
    dotest(40, 1.478896272);
    dotest(200, 1.478940994);
    dotest(1200, 1.478942805);
}
