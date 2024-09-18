fn quicksort<T: PartialOrd + Copy>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let pivot_index = partition(arr);
    quicksort(&mut arr[0..pivot_index]);
    quicksort(&mut arr[pivot_index + 1..len]);
}

fn partition<T: PartialOrd + Copy>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot_index = len - 1;
    let pivot = arr[pivot_index];
    let mut i = 0;

    for j in 0..pivot_index {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, pivot_index);
    i
}

fn main() {
    let mut arr = [52, 33, 44, 1, 2, 42];
    quicksort(&mut arr);
    println!("Sorted array of ints: {:?}", arr);

    let mut float_arr = [5.2, 3.3, 4.4, 1.1, 2.5, 4.2];
    quicksort(&mut float_arr);
    println!("Sorted array of floats: {:?}", float_arr);
}
