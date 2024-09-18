use itertools::{self, Itertools};

fn check_unique(inpt: &String) -> bool {
    // total chars count
    let inpt_size = inpt.chars().count();
    // comparing total chars count to the unique case insensitive chars count
    inpt_size == inpt.to_lowercase().chars().unique().count()
}

fn main() {
    assert_eq!(check_unique(&"abcd".to_string()), true);
    assert_eq!(check_unique(&"abCdefAaf".to_string()), false);
    assert_eq!(check_unique(&"aabcd".to_string()), false);
}
