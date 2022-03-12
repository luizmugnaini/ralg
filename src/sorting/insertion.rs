use super::Sorter;
use std::cmp::PartialOrd;

pub struct InsertionSort;

impl Sorter for InsertionSort {
    fn sort<T: PartialOrd + Copy>(xs: &mut [T]) {
        for not_sorted in 1..xs.len() {
            let mut i = not_sorted;
            while i > 0 && xs[i - 1] > xs[i] {
                xs.swap(i - 1, i);
                i -= 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::sorting;

    #[test]
    fn insertion() {
        let mut xs = vec![123, 91847, 1, 0, -1, -450, 800, 555];
        InsertionSort::sort(&mut xs);
        assert!(sorting::is_sorted(&xs));
    }
}
