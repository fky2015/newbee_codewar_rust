mod solution {
    fn generate_range(a: &[i32], start: usize, end: usize) -> String {
        if start == end {
            format!("{}", a[start])
        } else if start == end - 1 {
            format!("{},{}", a[start], a[end])
        } else {
            format!("{}-{}", a[start], a[end])
        }
    }

    pub fn range_extraction(a: &[i32]) -> String {
        let mut start = 0;
        let mut end = 0;
        let mut ret = vec![];
        while start < a.len() {
            if end + 1 < a.len() && a[end + 1] == a[end] + 1 {
                end += 1;
            } else {
                ret.push(generate_range(&a, start, end));
                start = end + 1;
                end = start;
            }
        }
        println!("{:?}", ret);
        ret.join(",")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            "-6,-3-1,3-5,7-11,14,15,17-20",
            solution::range_extraction(&[
                -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20
            ])
        );
        assert_eq!(
            "-3--1,2,10,15,16,18-20",
            solution::range_extraction(&[-3, -2, -1, 2, 10, 15, 16, 18, 19, 20])
        );
    }
}
