use itertools::Itertools;

fn reverse_words_string(string: &mut String) {
    // the word here is a string that is not a whitespace
    *string = string.split_whitespace().rev().join(" ");
}

fn main() {
    let mut s = String::from("snow dog sun");
    println!("Original string: {}", s);

    reverse_words_string(&mut s);

    println!("Reversed string: {}", s);
}
