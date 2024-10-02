use serde_json::json;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: lf [-t threads] file");
        return;
    }

    let file_path = args.last().unwrap();
    let num_threads = match args.iter().position(|arg| arg == "-t") {
        Some(pos) if pos + 1 < args.len() => args[pos + 1].parse::<usize>().unwrap_or(1),
        _ => 1,
    };

    let file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);

    let start_time = Instant::now();
    let result = count_letter_frequency(reader, num_threads);
    let elapsed_time = start_time.elapsed();

    let output = json!({
        "elapsed": format!("{:.3?}", elapsed_time),
        "result": result,
    });

    println!("{}", output.to_string());
}

fn count_letter_frequency(reader: BufReader<File>, num_threads: usize) -> serde_json::Value {
    let text = reader
        .lines()
        .collect::<Result<String, _>>()
        .expect("Unable to read file");
    let text_len = text.len();
    let chunk_size = (text_len + num_threads - 1) / num_threads;

    let frequency = Arc::new(Mutex::new([0; 26]));
    let mut handles = vec![];

    for i in 0..num_threads {
        let start = i * chunk_size;
        let end = if i == num_threads - 1 {
            text_len
        } else {
            (i + 1) * chunk_size
        };
        let text_chunk = text[start..end].to_string();
        let frequency = Arc::clone(&frequency);

        let handle = thread::spawn(move || {
            let mut local_frequency = [0; 26];
            for c in text_chunk.chars() {
                if c.is_ascii_alphabetic() {
                    let index = (c.to_ascii_lowercase() as u8 - b'a') as usize;
                    local_frequency[index] += 1;
                }
            }
            let mut freq = frequency.lock().unwrap();
            for i in 0..26 {
                freq[i] += local_frequency[i];
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let frequency = frequency.lock().unwrap();
    let mut result = serde_json::Map::new();
    for (i, &count) in frequency.iter().enumerate() {
        let letter = (b'a' + i as u8) as char;
        result.insert(letter.to_string(), json!(count));
    }

    serde_json::Value::Object(result)
}
