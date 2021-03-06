mod insertion;
mod merge;
mod search;
mod tim;

use std::cmp::PartialOrd;

trait Sorter {
    fn sort<T: PartialOrd + Copy>(xs: &mut [T]);
}

/// Checks if array `xs` is sorted
pub fn is_sorted(xs: &[impl PartialOrd]) -> bool {
    let mut last = &xs[0];
    for next in xs {
        if last > next {
            return false;
        }
        last = next;
    }
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_sorted() {
        let v1 = vec![1, 2, 3, 4, 5, 5];
        assert!(super::is_sorted(&v1));

        let v2 = vec![0, 3, 9, 8, 10];
        assert!(!super::is_sorted(&v2));
    }
}
