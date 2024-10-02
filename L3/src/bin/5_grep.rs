use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

struct GrepOptions {
    after: usize,
    before: usize,
    context: usize,
    count: bool,
    ignore_case: bool,
    invert: bool,
    fixed: bool,
    line_num: bool,
}

impl GrepOptions {
    fn new() -> Self {
        GrepOptions {
            after: 0,
            before: 0,
            context: 0,
            count: false,
            ignore_case: false,
            invert: false,
            fixed: false,
            line_num: false,
        }
    }
}

fn grep(pattern: &str, filename: &str, options: GrepOptions) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        lines.push(line?);
    }

    let mut matched_lines = Vec::new();
    let mut match_count = 0;

    for (index, line) in lines.iter().enumerate() {
        let line_to_compare = if options.ignore_case {
            line.to_lowercase()
        } else {
            line.to_string()
        };

        let pattern_to_compare = if options.ignore_case {
            pattern.to_lowercase()
        } else {
            pattern.to_string()
        };

        let is_match = if options.fixed {
            line_to_compare == pattern_to_compare
        } else {
            line_to_compare.contains(&pattern_to_compare)
        };

        let is_inverted_match = if options.invert { !is_match } else { is_match };

        if is_inverted_match {
            match_count += 1;

            // (options -A, -B, -C)
            let start = if index >= options.before {
                index - options.before
            } else {
                0
            };

            let end = if index + options.after + 1 < lines.len() {
                index + options.after + 1
            } else {
                lines.len()
            };

            matched_lines.push((index + 1, lines[start..end].to_vec()));
        }
    }

    if options.count {
        println!("{}", match_count);
        return Ok(());
    }

    for (line_number, lines_to_print) in matched_lines {
        for (i, line) in lines_to_print.iter().enumerate() {
            if options.line_num {
                println!("{}: {}", line_number + i, line);
            } else {
                println!("{}", line);
            }
        }
    }

    Ok(())
}

// Функция обработки аргументов командной строки
fn parse_args() -> Result<(String, String, GrepOptions), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Not enough arguments",
        )));
    }

    let mut options = GrepOptions::new();
    let mut pattern = String::new();
    let mut filename = String::new();

    let mut iter = args.iter().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-A" => {
                if let Some(num_str) = iter.next() {
                    options.after = num_str.parse().unwrap_or(0);
                }
            }
            "-B" => {
                if let Some(num_str) = iter.next() {
                    options.before = num_str.parse().unwrap_or(0);
                }
            }
            "-C" => {
                if let Some(num_str) = iter.next() {
                    options.context = num_str.parse().unwrap_or(0);
                    options.after = options.context;
                    options.before = options.context;
                }
            }
            "-c" => options.count = true,
            "-i" => options.ignore_case = true,
            "-v" => options.invert = true,
            "-F" => options.fixed = true,
            "-n" => options.line_num = true,
            _ => {
                if pattern.is_empty() {
                    pattern = arg.clone();
                } else {
                    filename = arg.clone();
                }
            }
        }
    }

    if pattern.is_empty() || filename.is_empty() {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Pattern or filename is missing",
        )));
    }

    Ok((pattern, filename, options))
}

fn main() -> Result<(), Box<dyn Error>> {
    let (pattern, filename, options) = parse_args()?;
    grep(&pattern, &filename, options)?;

    Ok(())
}
