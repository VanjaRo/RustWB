use std::io;
use std::sync::mpsc;
use std::thread;

fn main() {
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let n: usize = input_line.trim().parse().expect("Input not an usize");

    // looking at the function description –– this way of getting cpu count has it's limitations
    let cores = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    let (tx, rx) = mpsc::channel();
    // splitting work by chunks to make the workload fair
    let chunk_size = (n + cores - 1) / cores;
    for i in 0..cores {
        // cloning receiver for each thread
        let tx = tx.clone();
        let start = i * chunk_size + 1;
        let end = ((i + 1) * chunk_size).min(n);

        thread::spawn(move || {
            // might quickly hit the type boundary
            let mut sum: u64 = 0;
            for x in start..=end {
                sum = sum.saturating_add((x as u64).saturating_mul(x as u64));
            }
            tx.send(sum).unwrap();
        });
    }

    // Close main sender
    drop(tx);

    let mut sum: u64 = 0;
    for result in rx {
        sum = sum.saturating_add(result);
    }

    println!("{}", sum);
}
