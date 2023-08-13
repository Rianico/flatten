pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outter_iter: O,
    prev_iter: Option<<O::Item as IntoIterator>::IntoIter>,
    back_iter: Option<<O::Item as IntoIterator>::IntoIter>,
}

pub fn flatten<O>(iter: O) -> Flatten<O::IntoIter>
where
    O: IntoIterator,
    O::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    pub fn new(iter: O) -> Self {
        Flatten {
            outter_iter: iter,
            prev_iter: None,
            back_iter: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(it) = self.prev_iter.as_mut() {
                match it.next() {
                    v @ Some(_) => return v,
                    None => {}
                }
            }
            self.prev_iter = self.outter_iter.next().map(|i| i.into_iter());
            let None = self.prev_iter else {
                continue;
            };
            return self.back_iter.as_mut()?.next();
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(back) = self.back_iter.as_mut() {
                match back.next_back() {
                    v @ Some(_) => return v,
                    None => {}
                }
            }
            self.back_iter = self.outter_iter.next_back().map(|i| i.into_iter());
            let None = self.back_iter else {
                continue;
            };
            return self.prev_iter.as_mut()?.next_back();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::flatten;

    #[test]
    fn test_new_flatten() {
        let _ = flatten(vec![vec![1, 2, 3], vec![4, 5, 6]]);
    }

    #[test]
    fn test_into_iter() {
        let mut f = flatten(vec![vec![1, 2, 3], vec![4]]).into_iter();
        assert_eq!(f.next(), Some(1));
        assert_eq!(f.next(), Some(2));
        assert_eq!(f.next(), Some(3));
        assert_eq!(f.next(), Some(4));
        assert_eq!(f.next(), None);
    }

    #[test]
    fn test_empty_vec() {
        let mut f = flatten(vec![vec![1], vec![]]).into_iter();
        assert_eq!(f.next(), Some(1));
        assert_eq!(f.next(), None);

        let mut f = flatten(vec![vec![], vec![]]).into_iter();
        assert_eq!(f.next(), None::<i32>);
    }

    #[test]
    fn test_one() {
        assert_eq!(flatten(std::iter::once(vec!["a"])).count(), 1);
    }

    #[test]
    fn test_two() {
        assert_eq!(flatten(std::iter::once(vec!["a", "b"])).count(), 2);
    }

    #[test]
    fn two_wide() {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]]).count(), 2);
    }

    #[test]
    fn both_ends() {
        let mut iter = flatten(vec![vec!["a1", "a2", "a3"], vec!["b1", "b2", "b3"]]);
        assert_eq!(iter.next(), Some("a1"));
        assert_eq!(iter.next_back(), Some("b3"));
        assert_eq!(iter.next(), Some("a2"));
        assert_eq!(iter.next_back(), Some("b2"));
        assert_eq!(iter.next(), Some("a3"));
        assert_eq!(iter.next_back(), Some("b1"));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn inf() {
        let mut iter = flatten((0..).map(|i| 0..i));
        // 0 => 0..0 => empty
        // 1 => 0..1 => [0]
        // 2 => 0..2 => [0, 1]
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
    }

    #[test]
    fn deep() {
        assert_eq!(flatten(flatten(vec![vec![vec![0, 1]]])).count(), 2);
    }

    #[test]
    fn reverse() {
        assert_eq!(
            flatten(std::iter::once(vec!["a", "b"]))
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }

    #[test]
    fn reverse_wide() {
        assert_eq!(
            flatten(vec![vec!["a"], vec!["b"]])
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }
}
