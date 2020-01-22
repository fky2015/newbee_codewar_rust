use std::cmp::Ordering;
use std::collections::HashMap;

fn mix(s1: &str, s2: &str) -> String {
    let mut hmap: HashMap<char, (usize, usize)> = HashMap::new();
    for c in s1.replace(' ', "").chars().filter(|c| c.is_lowercase()) {
        hmap.entry(c).or_insert((0, 0)).0 += 1;
    }

    for c in s2.replace(' ', "").chars().filter(|c| c.is_lowercase()) {
        hmap.entry(c).or_insert((0, 0)).1 += 1;
    }

    println!("{:?}", hmap);
    let mut v = hmap
        .iter()
        .map(|(&k, &v)| match v.0.cmp(&v.1) {
            Ordering::Equal => (v.0, k, "="),
            Ordering::Greater => (v.0, k, "1"),
            Ordering::Less => (v.1, k, "2"),
        })
        .filter(|&(n, _, _)| n > 1)
        .collect::<Vec<_>>();

    v.sort_by(|&a, &b| {
        if a.0 != b.0 {
            b.0.cmp(&a.0)
        } else if a.2 != b.2 {
            a.2.cmp(&b.2)
        } else {
            a.1.cmp(&b.1)
        }
    });

    println!("{:?}", v);
    v.into_iter()
        .map(|(n, c, s)| format!("{}:{}", s, c.to_string().repeat(n)))
        .collect::<Vec<String>>()
        .join("/")
}

fn testing(s1: &str, s2: &str, exp: &str) -> () {
    assert_eq!(&mix(s1, s2), exp)
}

#[test]
fn basics_mix() {
    testing(
        "Are they here",
        "yes, they are here",
        "2:eeeee/2:yy/=:hh/=:rr",
    );
    testing(
        "looping is fun but dangerous",
        "less dangerous than coding",
        "1:ooo/1:uuu/2:sss/=:nnn/1:ii/2:aa/2:dd/2:ee/=:gg",
    );
    testing(
        " In many languages",
        " there's a pair of functions",
        "1:aaa/1:nnn/1:gg/2:ee/2:ff/2:ii/2:oo/2:rr/2:ss/2:tt",
    );
    testing("Lords of the Fallen", "gamekult", "1:ee/1:ll/1:oo");
    testing("codewars", "codewars", "");
    testing(
        "A generation must confront the looming ",
        "codewarrs",
        "1:nnnnn/1:ooooo/1:tttt/1:eee/1:gg/1:ii/1:mm/=:rr",
    );
}
