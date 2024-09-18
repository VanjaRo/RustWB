use crossbeam::channel;
use std::io;
use std::thread;

// This implementation uses crossbeam unbounded mpmc
// However the needed solution is spmc, the most trivial version of which could be implemented using Vec with Mutex
// Also could be done with standard std::sync::mpsc channel by wrapping the receiver into Arc + Mutex
fn main() {
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let n: usize = input_line.trim().parse().expect("Input not an usize");

    let (sender, receiver) = channel::unbounded();

    let producer_thread = thread::spawn(move || {
        for i in 0..100 {
            if sender.send(i).is_err() {
                break;
            }
            println!("Producer: sent {}", i);
        }
    });

    let mut worker_threads = Vec::new();
    for i in 0..n {
        let worker_receiver = receiver.clone();
        let worker_thread = thread::spawn(move || {
            for msg in worker_receiver {
                println!("Worker {}: received {}", i, msg);
            }
            println!("Worker {}: stopped", i);
        });
        worker_threads.push(worker_thread);
    }
    // waiting the threads to finish
    producer_thread.join().expect("Producer thread panicked");

    for worker in worker_threads {
        worker.join().expect("Worker thread panicked");
    }
}
