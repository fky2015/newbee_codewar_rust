use std::collections::HashSet;

fn swap(v: &mut Vec<char>, a: char, b: char) -> bool {
    let a_idx = v.iter().position(|&s| s == a).unwrap();
    let b_idx = v.iter().position(|&s| s == b).unwrap();
    if a_idx < b_idx {
        false
    } else {
        v.swap(a_idx, b_idx);
        true
    }
}

fn recover_secret(triplets: Vec<[char; 3]>) -> String {
    let mut hset = HashSet::new();
    triplets.iter().for_each(|&a| {
        a.iter().for_each(|&c| {
            hset.insert(c);
        });
    });
    let mut v = hset.into_iter().collect::<Vec<char>>();
    let mut swap_count = 0;
    loop {
        for item in &triplets {
            if swap(&mut v, item[0], item[1]) {
                swap_count += 1;
            }
            if swap(&mut v, item[1], item[2]) {
                swap_count += 1;
            }
        }

        if swap_count == 0 {
            break;
        } else {
            swap_count = 0;
        }
    }
    v.iter().collect()
}

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn example_test() {
    assert_eq!(
        recover_secret(vec![
            ['t', 'u', 'p'],
            ['w', 'h', 'i'],
            ['t', 's', 'u'],
            ['a', 't', 's'],
            ['h', 'a', 'p'],
            ['t', 'i', 's'],
            ['w', 'h', 's']
        ]),
        "whatisup"
    );
}
