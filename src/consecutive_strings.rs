fn testing(strarr: Vec<&str>, k: usize, exp: &str) -> () {
    assert_eq!(&longest_consec(strarr, k), exp)
}

#[test]
fn basics_longest_consec() {
    testing(
        vec!["zone", "abigail", "theta", "form", "libe", "zas"],
        2,
        "abigailtheta",
    );
    testing(
        vec![
            "ejjjjmmtthh",
            "zxxuueeg",
            "aanlljrrrxx",
            "dqqqaaabbb",
            "oocccffuucccjjjkkkjyyyeehh",
        ],
        1,
        "oocccffuucccjjjkkkjyyyeehh",
    );
    testing(vec![], 3, "");
    testing(
        vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"],
        3,
        "ixoyx3452zzzzzzzzzzzz",
    );
    testing(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 15, "");
    testing(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 0, "");
}


fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    if strarr.len() == 0 || k > strarr.len() || k <= 0 {
        "".to_string()
    } else {
        let mut left = 0;
        let mut right = k;
        let mut sum = 0;

        for i in 0..k {
            sum += strarr[i].len();
        }
        let mut maxn = sum;
        let mut maxn_index = 0;
        while right < strarr.len() {
            sum -= strarr[left].len();
            left += 1;
            right += 1;
            sum += strarr[right - 1].len();
            if sum > maxn {
                maxn = sum;
                maxn_index = left;
            }
        }
        // 这是怎么做到的
        let ret = &strarr[maxn_index..maxn_index + k];
        let ret: String = ret.iter().fold("".to_string(), |sum, x| sum + x);
        println!("{}", ret);
        ret
    }
}