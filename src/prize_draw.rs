use std::cmp::Reverse;
fn rank(st: &str, we: Vec<i32>, n: usize) -> &str {
    if st.is_empty() {
        return "No participants";
    }
    let v: Vec<&str> = st.split(',').collect();
    if n > v.len() {
        return "Not enough participants";
    }
    let v = v.iter().enumerate().collect::<Vec<_>>();
    let mut v = v
        .iter()
        .map(|&(i, &name)| {
            (
                i,
                name.to_ascii_lowercase()
                    .chars()
                    .map(|c: char| c as u8 - b'a' + 1u8)
                    .sum::<u8>() as usize
                    + name.len(),
                name,
            )
        })
        .map(|(idx, num, name): (usize, usize, &str)| (we[idx] as u32 * num as u32, name))
        .collect::<Vec<_>>();
    v.sort_by(|&a, &b| {
        if a.0 != b.0 {
            b.0.cmp(&a.0)
        } else {
            a.1.cmp(b.1)
        }
    });
    println!("{:?}", v);
    v[n - 1].1
}

fn testing(st: &str, we: Vec<i32>, n: usize, exp: &str) -> () {
    assert_eq!(rank(st, we, n), exp)
}

#[test]
fn basics_rank() {
    testing(
        "COLIN,AMANDBA,AMANDAB,CAROL,PauL,JOSEPH",
        vec![1, 4, 4, 5, 2, 1],
        4,
        "PauL",
    );
    testing(
        "Addison,Jayden,Sofia,Michael,Andrew,Lily,Benjamin",
        vec![4, 2, 1, 4, 3, 1, 2],
        4,
        "Benjamin",
    );
    testing(
        "Elijah,Chloe,Elizabeth,Matthew,Natalie,Jayden",
        vec![1, 3, 5, 5, 3, 6],
        2,
        "Matthew",
    );
    testing(
        "Aubrey,Olivai,Abigail,Chloe,Andrew,Elizabeth",
        vec![3, 1, 4, 4, 3, 2],
        4,
        "Abigail",
    );
    testing("Lagon,Lily", vec![1, 5], 2, "Lagon");
}
