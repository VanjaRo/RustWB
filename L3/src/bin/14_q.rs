fn main() {
    // This is used for communication between threads.
    // tx is the transmitter (sender) side of the channel.
    // rv is the receiver side of the channel.
    let (tx, rv) = std::sync::mpsc::channel::<i32>();

    // move ensures that the ownership of tx (the transmitter) is moved into the new thread,
    // allowing the spawned thread to send data through the channel
    let handle = std::thread::spawn(move || {
        for i in 0..10 {
            // For each value i in this range, the value is sent through the channel using tx.send(i).
            // unwrap() is used to handle potential errors, which could occur if the channel is closed or other transmission issues arise,
            // but it's safe in this case because the receiver is still open.
            tx.send(i).unwrap();
        }
    });

    // The handle.join().unwrap() waits for the spawned thread to complete its execution.
    // This ensures that the main thread will not continue until the child thread is finished.
    handle.join().unwrap();

    for i in rv.iter() {
        // As there is no racing during data sending the output would be 0 to 9 in order of the thread for loop
        println!("{i:?}");
    }
}
