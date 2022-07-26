use super::Sorter;

pub struct TimSort;

impl Sorter for TimSort {
    fn sort<T: PartialOrd + Copy>(xs: &mut [T]) {
        todo!()
    }
}
