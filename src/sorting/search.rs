/// Binary searches for the `search`. Assumes `xs` is sorted. If the element is
/// found at `xs[idx]`, returns `Some(idx)`, otherwise, returns `None`.
pub fn binary_search<T: PartialOrd>(xs: &[T], search: &T) -> Option<usize> {
    _binary_search(xs, 0, xs.len() - 1, search)
}

fn _binary_search<T: PartialOrd>(
    xs: &[T],
    low: usize,
    top: usize,
    search: &T,
) -> Option<usize> {
    if top < low {
        // Check if this is be the last search, if so, the element was not found
        return None;
    }

    let mid = (top + low) / 2;
    let x = &xs[mid];

    if x == search {
        // Element found
        Some(mid)
    } else if x < search {
        // Search the right slice
        _binary_search(xs, mid + 1, top, search)
    } else {
        // Search the left slice
        _binary_search(xs, low, mid - 1, search)
    }
}

/// Given `sum`, determines if there exists at least one pair of distinct
/// elements in `xs` whose sum is equal to `sum`.
pub fn has_two_sum<T: PartialOrd>(xs: &[T], sum: T) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn binary_search() {
        let xs = vec![3];
        assert_eq!(super::binary_search(&xs, &3), Some(0));

        let xs = vec![3, 4];
        assert_eq!(super::binary_search(&xs, &3), Some(0));
        assert_eq!(super::binary_search(&xs, &4), Some(1));

        let xs = vec![3, 5, 7, 11, 35, 90];
        assert_eq!(super::binary_search(&xs, &90), Some(xs.len() - 1));
        assert_eq!(super::binary_search(&xs, &6), None);
    }
}
