fn main() {
    // s1 is a string slice (&str) which points to a string literal.
    // String literals are statically allocated,
    // meaning they are stored in the program's binary, and s1 is a reference to this static data.
    let s1 = "hello";

    // s2 is a heap-allocated string of type String.
    // This allocates memory on the heap to store the contents of the string ("hello").
    // The String type manages ownership and allows for dynamic resizing.
    let s2 = String::from("hello");

    // s3 is another string slice (&str), but it points to the contents of the heap-allocated String (s2).
    // The as_str() method converts the String into a &str,
    // providing an immutable reference to the underlying string data.
    let s3 = s2.as_str();

    // This calculates the size in bytes of the &str slice s1. Since &str is a reference,
    // its size is constant regardless of the length of the string.
    // It typically holds the size of a pointer (which is 8 bytes on a 64-bit system)
    // and the length of the slice, making it 16 bytes in total on a 64-bit system.
    let size_of_s1 = std::mem::size_of_val(s1);

    // This calculates the size in bytes of the String type s2.
    // The size of String itself does not depend on the string's length. Instead, it holds:
    // A pointer to the heap-allocated data,
    // The length of the string,
    // The capacity (how much memory is allocated for it). On a 64-bit system, this would also typically be 24 bytes (3 pointers, each 8 bytes).
    let size_of_s2 = std::mem::size_of_val(&s2);

    // This calculates the size in bytes of the string slice s3.
    // Like s1, s3 is a &str, so its size is also constant,
    // typically 16 bytes on a 64-bit system (a pointer and a length).
    let size_of_s3 = std::mem::size_of_val(&s3);

    println!("{:?}", size_of_s1);

    println!("{:?}", size_of_s2);

    println!("{:?}", size_of_s3);
}
