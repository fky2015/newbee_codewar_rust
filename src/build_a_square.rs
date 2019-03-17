use std::iter;

fn generate_shape(n: i32) -> String {
    let line = "+".repeat(n as usize);
    let ans: Vec<String> = iter::repeat(line).take(n as usize).collect::<Vec<String>>();
    ans.join("\n")
}
// fn generate_shape(n: usize) -> String {
//    vec!["+".repeat(n); n].join("\n")
// }

#[test]
fn sample_test() {
    assert_eq!(generate_shape(3), "+++\n+++\n+++");
}
