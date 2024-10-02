fn main() {
    // Here, we declare a variable a which is an array of integers.
    let a = [76, 77, 78, 79, 80];
    // This creates a slice b of the array a.
    // The slice operation &a[1..4] means we are taking a reference to a portion of the array a,
    // starting from index 1 (inclusive) and going up to index 4 (exclusive).
    let b = &a[1..4];
    // This prints the contents of the slice b using the debug formatting option :?.
    // The output will be in the form of an array-like structure.
    println!("{b:?}");
}
