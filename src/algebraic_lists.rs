// https://www.codewars.com/kata/529a92d9aba78c356b000353/solutions/rust
#[derive(Debug, PartialEq, Eq)]
enum Cons<T: Clone> {
    Cons(T, Box<Cons<T>>),
    Null,
}

impl<T: Clone> Cons<T> {
    pub fn new(head: T, tail: Self) -> Self {
        Cons::Cons(head, Box::new(tail))
    }

    pub fn to_vec(&self) -> Vec<T> {
        match self {
            &Cons::Null => vec![],
            &Cons::Cons(ref head, ref tail) => {
                let mut head = vec![head.clone()];
                head.extend(tail.to_vec());
                head
            }
        }
    }
}

impl<T: Clone> Cons<T> {
    fn iter_helper<I>(mut iter: I::IntoIter) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        match iter.next() {
            Some(v) => Cons::Cons(v, Box::new(Self::iter_helper::<I>(iter))),
            None => Cons::Null,
        }
    }
    pub fn from_iter<I>(it: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        //TODO: provide a convenient method to convert any iterable
        //      to an algebraic list.
        let iter = it.into_iter();
        //        loop {
        //            Some(v) => Cons::Cons(v, Box::new(Self::from_iter(it))),
        //            None => Cons::Null,
        //        }
        Self::iter_helper::<I::IntoIter>(iter)
    }

    pub fn filter<F>(&self, fun: F) -> Self
    where
        F: Fn(&T) -> bool,
    {
        //TODO: return a new algebraic list containing only elements
        //      that satisfy the predicate function.
        match self {
            Cons::Cons(v, vs) => {
                if fun(v) {
                    Cons::Cons(v.clone(), Box::new(vs.filter(fun)))
                } else {
                    vs.filter(fun)
                }
            }
            &Cons::Null => Cons::Null,
        }
    }

    pub fn map<F, S>(&self, fun: F) -> Cons<S>
    where
        F: Fn(T) -> S,
        S: Clone,
    {
        //TODO: return a new algebraic list containing all elements
        //      resulting from applying the mapper function to them.
        match self {
            Cons::Cons(v, vs) => Cons::Cons(fun(v.clone()), Box::new(vs.map(fun))),
            Cons::Null => Cons::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_from_vec() {
        assert_eq!(Cons::from_iter(Vec::<i32>::new()), Cons::Null);

        assert_eq!(
            Cons::from_iter(vec![1, 2, 3, 4, 5]).to_vec(),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn should_filter() {
        assert_eq!(
            Cons::from_iter(vec![1, 2, 3, 4, 5])
                .filter(|&n| n > 3)
                .to_vec(),
            vec![4, 5]
        );

        assert_eq!(
            Cons::from_iter(vec![1, 2, 3, 4, 5]).filter(|&n| n > 5),
            Cons::Null
        );
    }

    #[test]
    fn should_map() {
        assert_eq!(
            Cons::from_iter(vec!["1", "2", "3", "4", "5"])
                .map(str::parse::<i32>)
                .map(Result::unwrap)
                .to_vec(),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn should_filter_map() {
        assert_eq!(
            Cons::from_iter(vec![1, 2, 3, 4, 5])
                .filter(|n| n % 2 == 0)
                .map(|x| x.to_string())
                .to_vec(),
            ["2", "4"]
        );
    }
}
