fn isrp(mut a: i64, mut b: i64) -> bool {
    if a == 1 || b == 1 {
        return true;
    }
    loop {
        let t = a % b;
        if t == 0 {
            break;
        } else {
            a = b;
            b = t;
        }
    }
    b <= 1
}

fn from_nb_2str(n: i64, sys: Vec<i64>) -> String {
    // your code
    //
    let mut product = 1;
    for i in 0..sys.len() {
        for j in 0..i {
            if !isrp(sys[i], sys[j]) {
                return "Not applicable".to_string();
            }
        }
        product *= sys[i];
    }
    if product <= n {
        return "Not applicable".to_string();
    }
    format!(
        "-{}-",
        sys.iter()
            .map(|i| (n % i).to_string())
            .collect::<Vec<String>>()
            .join("--")
    )
}

fn testing(n: i64, sys: Vec<i64>, exp: &str) -> () {
    assert_eq!(&from_nb_2str(n, sys), exp)
}

#[test]
fn basics_from_nb_2str() {
    testing(779, vec![8, 7, 5, 3], "-3--2--4--2-");
    testing(187, vec![8, 7, 5, 3], "-3--5--2--1-");
    testing(3450, vec![13, 11, 7, 5, 3, 2], "-5--7--6--0--0--0-");
    testing(11, vec![2, 3, 5], "-1--2--1-");
    testing(6, vec![2, 3, 4], "Not applicable");
}
