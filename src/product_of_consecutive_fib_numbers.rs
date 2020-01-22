fn product_fib(prod: u64) -> (u64, u64, bool) {
    // your code
    let mut a = 0u64;
    let mut b = 1u64;
    loop {
        if a * b >= prod {
            return (a, b, a * b == prod);
        }
        let t = a + b;
        a = b;
        b = t;
    }
//    while a * b < prod {
//        let c = a + b;
//        a = b;
//        b = c;
//    }
//    (a, b, a * b == prod)
}

fn dotest(prod: u64, exp: (u64, u64, bool)) -> () {
    assert_eq!(product_fib(prod), exp)
}

#[test]
fn basics_product_fib() {
    dotest(4895, (55, 89, true));
    dotest(5895, (89, 144, false));
}
