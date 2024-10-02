use std::cmp::Ordering;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

enum SortType {
    Lexicographical,
    Numeric,
    Month,
    HumanNumeric,
}

fn sort_lexicographically(a: &str, b: &str) -> Ordering {
    a.cmp(b)
}

fn sort_numeric(a: &str, b: &str) -> Ordering {
    a.trim()
        .parse::<f64>()
        .unwrap_or(f64::NAN)
        .partial_cmp(&b.trim().parse::<f64>().unwrap_or(f64::NAN))
        .unwrap_or(Ordering::Equal)
}

fn sort_by_month(a: &str, b: &str) -> Ordering {
    let months = vec![
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let a_month = months.iter().position(|&m| m == a).unwrap_or(usize::MAX);
    let b_month = months.iter().position(|&m| m == b).unwrap_or(usize::MAX);
    a_month.cmp(&b_month)
}

fn sort_human_numeric(a: &str, b: &str) -> Ordering {
    fn parse_suffix(s: &str) -> f64 {
        let last_char = s.chars().last();
        let num_part: f64 = s
            .chars()
            .filter(|c| c.is_numeric() || *c == '.')
            .collect::<String>()
            .parse()
            .unwrap_or(0.0);
        match last_char {
            Some('K') => num_part * 1_000.0,
            Some('M') => num_part * 1_000_000.0,
            Some('G') => num_part * 1_000_000_000.0,
            _ => num_part,
        }
    }
    parse_suffix(a)
        .partial_cmp(&parse_suffix(b))
        .unwrap_or(Ordering::Equal)
}

fn sort_by_criteria(a: &str, b: &str, sort_type: &SortType, column: Option<usize>) -> Ordering {
    let (a_val, b_val) = if let Some(col) = column {
        (
            a.split_whitespace().nth(col).unwrap_or(""),
            b.split_whitespace().nth(col).unwrap_or(""),
        )
    } else {
        (a, b)
    };

    match sort_type {
        SortType::Numeric => sort_numeric(a_val, b_val),
        SortType::Month => sort_by_month(a_val, b_val),
        SortType::HumanNumeric => sort_human_numeric(a_val, b_val),
        SortType::Lexicographical => sort_lexicographically(a_val, b_val),
    }
}

fn sort_file(
    filename: &str,
    column: Option<usize>,
    sort_type: SortType,
    reverse: bool,
    unique: bool,
    ignore_trailing_space: bool,
    check_sorted: bool,
) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    if ignore_trailing_space {
        for line in &mut lines {
            *line = line.trim_end().to_string();
        }
    }

    if unique {
        let mut set = HashSet::new();
        lines.retain(|line| set.insert(line.clone()));
    }

    if check_sorted {
        let mut sorted = true;
        for i in 1..lines.len() {
            if sort_by_criteria(&lines[i - 1], &lines[i], &sort_type, column) == Ordering::Greater {
                sorted = false;
                break;
            }
        }
        if sorted {
            println!("The file is already sorted.");
        } else {
            println!("The file is not sorted.");
        }
        return Ok(());
    }

    lines.sort_by(|a, b| sort_by_criteria(a, b, &sort_type, column));

    if reverse {
        lines.reverse();
    }

    let output_filename = "sorted_output.txt";
    let mut output_file = File::create(output_filename)?;
    for line in lines {
        writeln!(output_file, "{}", line)?;
    }

    println!("Sorted lines are written to {}", output_filename);
    Ok(())
}

fn print_usage() {
    println!(
        "Usage: sort [OPTIONS] <file>
Options:
  -k <column>       Specify column to sort by (default: 1)
  -n                Sort by numeric value
  -r                Sort in reverse order
  -u                Remove duplicate lines
  -M                Sort by month name (Jan, Feb, etc.)
  -b                Ignore trailing spaces
  -c                Check if the file is sorted
  -h                Sort by human-readable numeric values (e.g., 10K, 1M)"
    );
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Not enough arguments",
        )));
    }

    let mut filename = "";
    let mut column = None;
    let mut sort_type = SortType::Lexicographical;
    let mut reverse = false;
    let mut unique = false;
    let mut ignore_trailing_space = false;
    let mut check_sorted = false;

    let mut iter = args.iter().skip(1);
    // The latest sort type considerate as the main
    // No support for multiple sort keys
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-k" => {
                if let Some(col_str) = iter.next() {
                    column = Some(col_str.parse().unwrap_or(0) - 1);
                }
            }
            "-n" => sort_type = SortType::Numeric,
            "-r" => reverse = true,
            "-u" => unique = true,
            "-M" => sort_type = SortType::Month,
            "-b" => ignore_trailing_space = true,
            "-c" => check_sorted = true,
            "-h" => sort_type = SortType::HumanNumeric,
            _ => filename = arg,
        }
    }

    if filename.is_empty() {
        print_usage();
        return Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            "No file specified",
        )));
    }

    sort_file(
        filename,
        column,
        sort_type,
        reverse,
        unique,
        ignore_trailing_space,
        check_sorted,
    )?;

    Ok(())
}
