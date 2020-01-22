use std::mem::swap;
fn next_smaller_number(n: u64) -> Option<u64> {
    let mut v: Vec<char> = n.to_string().chars().collect();
    if v.len() <= 1 {
        return None;
    }
    println!("{:?}", v);
    if (1..v.len() - 1).all(|i| v[i] <= v[i + 1])
        && (1..v.len()).all(|i| v[0] <= v[i] || v[i] == '0')
    {
        return None;
    }
    if v[v.len() - 1] < v[v.len() - 2] {
        let l = v.len();
        v.swap(l - 1, l - 2);
    } else {
        let mut head = 0;
        for i in (0..v.len() - 1).rev() {
            if v[i] > v[i + 1] {
                head = i;
                break;
            }
        }
        let mut v_tail = v.drain(head + 1..).collect::<Vec<_>>();
        println!("head: {}, {:?}, {:?}", head, v, v_tail);
        let pos = v_tail.len() - 1 - v_tail.iter().rev().position(|&c| v[head] > c).unwrap();
        swap(&mut v_tail[pos], &mut v[head]);
        println!("after swap: {:?}, {:?}", v, v_tail);
        v_tail.sort_by(|a, b| b.cmp(a));
        v.extend(v_tail);
    }
    Some(v.iter().collect::<String>().parse::<u64>().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(Some(12), next_smaller_number(21));
        assert_eq!(Some(790), next_smaller_number(907));
        assert_eq!(Some(513), next_smaller_number(531));
        assert_eq!(None, next_smaller_number(1027));
        assert_eq!(Some(414), next_smaller_number(441));
    }
}
