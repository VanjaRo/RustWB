fn reverse_string(string: &mut String) {
    *string = string.chars().rev().collect::<String>();
}

fn main() {
    let mut s = String::from("главрыба");
    println!("Original string: {}", s);

    reverse_string(&mut s);

    println!("Reversed string: {}", s);
}
