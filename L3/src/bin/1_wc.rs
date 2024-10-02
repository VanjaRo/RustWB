use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::process;

fn count_lines(file: &File) -> io::Result<usize> {
    let reader = BufReader::new(file);
    Ok(reader.lines().count())
}

fn count_words(file: &File) -> io::Result<usize> {
    let reader = BufReader::new(file);
    let mut word_count = 0;
    for line in reader.lines() {
        let line = line?;
        word_count += line.split_whitespace().count();
    }
    Ok(word_count)
}

fn count_chars(file: &mut File) -> io::Result<usize> {
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content.chars().count())
}

fn print_usage() {
    eprintln!("Usage: wc [-c|-l|-w] <file>");
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        print_usage();
        process::exit(1);
    }

    let option = if args.len() == 3 { &args[1] } else { "-w" };
    let filename = if args.len() == 3 { &args[2] } else { &args[1] };

    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: Could not open file '{}'", filename);
            process::exit(1);
        }
    };

    match option {
        "-l" => {
            let line_count = count_lines(&file)?;
            println!("{}", line_count);
        }
        "-w" => {
            let word_count = count_words(&file)?;
            println!("{}", word_count);
        }
        "-c" => {
            let char_count = count_chars(&mut file)?;
            println!("{}", char_count);
        }
        _ => {
            print_usage();
            process::exit(1);
        }
    }

    Ok(())
}
