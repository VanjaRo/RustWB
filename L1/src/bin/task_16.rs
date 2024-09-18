use std::cmp::Ordering;

fn bin_search<T: Ord>(arr: &[T], to_find: T) -> Option<usize> {
    let mut l = 0;
    let mut r = arr.len() - 1;

    while l <= r {
        let mid = (l + r) / 2;
        match arr[mid].cmp(&to_find) {
            Ordering::Less => l = mid + 1,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => r = mid - 1,
        }
    }
    None
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    assert_eq!(bin_search(&arr, 3), Some(2));

    assert_eq!(bin_search(&arr, 2), Some(1));

    assert_eq!(bin_search(&arr, 6), None);
}
