use std::io::{self, BufRead};

// imitating uniq behaviour from linux/unix
fn main() {
    let stdin = io::stdin();
    let mut previous_line = String::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        if line != previous_line {
            println!("=={}", line);
            previous_line = line;
        }
    }
}
