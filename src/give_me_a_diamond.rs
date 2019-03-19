fn print(n: i32) -> Option<String> {
    if n % 2 == 0 || n < 0 {
        return None;
    }
    let mut result = Vec::new();
    let n = (n + 1) / 2;
    for i in 0..n {
        let space_num = (n - i - 1) as usize;
        let char_num = (2 * i + 1) as usize;

        let line = format!("{}{}{}", " ".repeat(space_num), "*".repeat(char_num), "\n");
        result.push(line);
    }
    for i in (0..n-1).rev() {
        let space_num = (n - i - 1) as usize;
        let char_num = (2 * i + 1) as usize;

        let line = format!("{}{}{}", " ".repeat(space_num), "*".repeat(char_num), "\n");
        result.push(line);
    }
    // for i in (0..result.len() - 1).rev() {
    //     result.push(result[i].clone());
    // }
    let result = result.into_iter().collect();
    Some(result)
}

// fn format_row(mut row: usize, n: usize) -> String {
//     if row > n / 2 {
//         row = n - row - 1;
//     }
//     [" ".repeat(n / 2 - row), "*".repeat(2 * row + 1), "\n".to_owned()].concat()
// }

// fn print(n: i32) -> Option<String> {
//     if n < 0 || n % 2 == 0 {
//         return None;
//     }
//     let n = n as usize;
//     Some((0..n).map(|x| format_row(x, n)).collect::<Vec<String>>().concat())
// }

#[test]
fn basic_test() {
    assert_eq!(print(3), Some(" *\n***\n *\n".to_string()));
    assert_eq!(print(-3), None);
    assert_eq!(print(1), Some("*\n".to_string()));
}
