fn arrange(s: &str) -> String {
    let mut v: Vec<&str> = s.split(' ').collect();

    for i in 0..(v.len() - 1) {
        if (i % 2 == 1 && v[i].len() < v[i + 1].len())
            || (i % 2 == 0 && v[i].len() > v[i + 1].len())
        {
            v.swap(i, i + 1);
        }
    }
    println!("{:?}", v);
    v.iter()
        .enumerate()
        .map(|t| {
            if t.0 % 2 == 1 {
                t.1.to_ascii_uppercase()
            } else {
                t.1.to_ascii_lowercase()
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn testing(s: &str, exp: &str) -> () {
    assert_eq!(arrange(s), exp.to_string())
}

#[test]
fn basics_arrange() {
    testing(
        "who hit retaining The That a we taken",
        "who RETAINING hit THAT a THE we TAKEN",
    ); // 3
    testing(
        "on I came up were so grandmothers",
        "i CAME on WERE up GRANDMOTHERS so",
    ); // 4
}
