fn p(n: isize, low: isize) -> isize {
    if n == low {
        return 1;
    }
    let mut ret = 0;
    let mut par = low;
    while n - par >= par {
        ret += p(n - par, par);
        par += 1;
    }
    ret + 1
}

fn partitions(n: isize) -> isize {
    p(n, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test_01() {
        assert_eq!(partitions(1), 1);
    }

    #[test]
    fn basic_test_05() {
        assert_eq!(partitions(5), 7);
    }

    #[test]
    fn basic_test_10() {
        assert_eq!(partitions(10), 42);
    }

    #[test]
    fn basic_test_25() {
        assert_eq!(partitions(25), 1958);
    }
}
