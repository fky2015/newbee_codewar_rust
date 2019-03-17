// just simple repeat

fn repeater(string: &str, n: u32) -> String {
    string.repeat(n as usize)
}

// Rust test example:
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn basic_test() {
    assert_eq!(repeater("a", 5), "aaaaa");
    assert_eq!(repeater("Na", 16), "NaNaNaNaNaNaNaNaNaNaNaNaNaNaNaNa");
    assert_eq!(repeater("Wub ", 6), "Wub Wub Wub Wub Wub Wub ");
}
