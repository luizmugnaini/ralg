///! Miscellaneous functions

/// Given a number `n`, computes and returns the smallest power of 2 greater
/// than or equal to `n`. Assumes 0 as a power of 2.
pub fn next_power_of_2(n: usize) -> usize {
    // The zero case needs to be treated separately because we may cause usize overflow
    if n == 0 {
        1
    } else if (n & (n - 1)) == 0 {
        n
    } else {
        0x8000000000000000 >> (n.leading_zeros() - 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn next_power() {
        assert_eq!(next_power_of_2(0), 1);
        assert_eq!(next_power_of_2(1), 1);
        assert_eq!(next_power_of_2(2), 2);
        assert_eq!(next_power_of_2(3), 4);
        assert_eq!(next_power_of_2(8), 8);
        assert_eq!(next_power_of_2(33), 64);
        assert_eq!(next_power_of_2(55), 64);
        assert_eq!(next_power_of_2(1291), 2048);
    }
}
