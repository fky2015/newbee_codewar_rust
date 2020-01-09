use std::char;

fn _dec2_fact_string(nb: u64, w: u64, d: u64) -> String {
    let a = nb / w;
    let c = char::from_digit(a as u32, 32).unwrap();

    if d == 0 {
        c.to_string()
    } else {
        format!("{}{}", c, _dec2_fact_string(nb % w, w / d, d - 1))
    }
}

fn dec2_fact_string(nb: u64) -> String {
    // your code
    let mut d = 1;
    let mut w = 1;
    while w * d < nb {
        w *= d;
        d += 1;
    }

    _dec2_fact_string(nb, w, d - 1).to_ascii_uppercase()
}

fn _fact_string_2dec(s: &mut String, w: u64, depth: u64) -> u64 {
    match s.pop() {
        Some(c) => c.to_digit(32).unwrap() as u64 * w + _fact_string_2dec(s, w * depth, depth + 1),
        None => 0,
    }
}

fn fact_string_2dec(mut s:String) -> u64 {
    // your code
    // let mut s = s;
    _fact_string_2dec(&mut s, 1, 1)
}
// https://www.codewars.com/kata/54e320dcebe1e583250008fd/solutions/rust
// fn fact_string_2dec(s: String) -> u64 {
//   s.chars().rev().enumerate().fold(0, |acc, c| acc + fact((c.0 + 1) as u64) * single_fact_to_dec(c.1))
// }

fn testing1(nb: u64, exp: &str) {
    assert_eq!(&dec2_fact_string(nb), exp)
}

fn testing2(s: &str, exp: u64) {
    assert_eq!(fact_string_2dec(s.to_string()), exp)
}

#[test]
fn basics_dec2_fact_string() {
    testing1(2982, "4041000");
    testing1(463, "341010");
}
#[test]
fn basics_fact_string_2dec() {
    testing2("4041000", 2982);
    testing2("341010", 463);
}
