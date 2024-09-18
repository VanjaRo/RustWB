use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let n: usize = input_line.trim().parse().expect("Input not an usize");
    let numbers = 1..=n; // Массив чисел

    let (arr_tx, arr_rx) = mpsc::channel();
    let (square_tx, square_rx) = mpsc::channel();

    // Second thread: squaring and passing
    thread::spawn(move || {
        for received in arr_rx {
            let squared = received * received;
            println!("Received: {}, square: {}", received, squared);
            square_tx.send(squared).unwrap();
        }
    });

    // Second thread: printing
    let print_handler = thread::spawn(move || {
        for result in square_rx {
            println!("Result: {}", result);
        }
    });

    // Sending range values
    for number in numbers {
        println!("Sending: {}", number);
        arr_tx.send(number).unwrap();
        thread::sleep(Duration::from_millis(250));
    }
    // Dropping first sender to consecutevly close all other threads and channels
    drop(arr_tx);

    print_handler
        .join()
        .expect("Error while printing square result");
}
