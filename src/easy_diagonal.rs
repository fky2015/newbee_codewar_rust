fn diagonal(n: u32, p: u32) -> u64 {
    // your code
    let mut v = vec![1; n as usize + 1];
    for _ in 0..p {
        for j in 1..=(n - p) as usize {
            v[j] += v[j - 1];
        }
    }
    v.iter().take(n as usize + 1 - p as usize).sum()
}

fn testing(n: u32, p: u32, exp: u64) -> () {
    assert_eq!(diagonal(n, p), exp)
}
#[test]
fn basics_diagonal() {
    testing(7, 0, 8);
    testing(7, 1, 28);
    testing(20, 3, 5985);
    testing(20, 4, 20349);
    testing(20, 5, 54264);
    testing(20, 15, 20349);
    testing(55, 20, 1346766106565880);
}
