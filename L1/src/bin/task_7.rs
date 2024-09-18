use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel::<()>();

    let handle = thread::spawn(move || {
        // Will stop after all elements in channel are processed
        for _ in rx {
            println!("Thread is working...");
            thread::sleep(Duration::from_secs(1));
        }
        println!("Stopping thread...");
    });
    for _ in 0..10 {
        tx.send(()).unwrap();
    }
    // Wait 3 sec before stop
    thread::sleep(Duration::from_secs(3));
    drop(tx);

    handle.join().unwrap();
}
