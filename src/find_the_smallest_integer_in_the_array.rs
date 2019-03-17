#[test]
fn sample_tests() {
    assert_eq!(find_smallest_int(&[34, 15, 88, 2]), 2);
    assert_eq!(find_smallest_int(&[34, -345, -1, 100]), -345);
}


fn find_smallest_int(arr: &[i32]) -> i32 {
    match arr.iter().min(){
        Some (&i) => i,
        None => 0,
    }
}