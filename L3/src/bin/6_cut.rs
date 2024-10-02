use std::env;
use std::io::{self, BufRead};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut fields: Vec<usize> = Vec::new();

    // Default values
    let mut delimiter = '\t';
    let mut only_separated = false;

    let mut iter = args.iter().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            // The list specifies fields
            "-f" => {
                if let Some(field_str) = iter.next() {
                    fields = field_str
                        .split(',')
                        .filter_map(|s| s.parse().ok())
                        .collect();
                }
            }
            // Choosing a delimeter
            "-d" => {
                if let Some(delim_str) = iter.next() {
                    delimiter = delim_str.chars().next().unwrap_or('\t');
                }
            }
            // Only lines containg the delimeter
            "-s" => {
                only_separated = true;
            }
            _ => {
                eprintln!("Unknown argument: {}", arg);
                process::exit(1);
            }
        }
    }

    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let line = line.unwrap();
        if only_separated && !line.contains(delimiter) {
            continue;
        }

        let parts: Vec<&str> = line.split(delimiter).collect();
        let mut output = Vec::new();

        for field in &fields {
            if *field <= parts.len() {
                output.push(parts[*field - 1]);
            }
        }

        println!("{}", output.join(&delimiter.to_string()));
    }
}
