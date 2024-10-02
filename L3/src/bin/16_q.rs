fn as_chan(vs: &[i32]) -> std::sync::mpsc::Receiver<i32> {
    let (tx, rx) = std::sync::mpsc::channel();

    let handle = std::thread::spawn({
        // vs is cloned using .to_owned() so that ownership of the data can
        // be moved into the thread (due to the move closure).
        let vs = vs.to_owned();

        move || {
            for v in vs {
                tx.send(v).unwrap();
                std::thread::sleep(std::time::Duration::from_secs(1))
            }
            // Once all values are sent, drop(tx) is called to close the sender.
            // This is necessary to ensure the receiving side (rx) knows there will be no more messages.
            drop(tx);
        }
    });

    handle.join().unwrap();

    rx
}

fn merge(
    a: std::sync::mpsc::Receiver<i32>,
    b: std::sync::mpsc::Receiver<i32>,
) -> std::sync::mpsc::Receiver<i32> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut a_done = false;

    let mut b_done = false;

    // This loop attempts to receive values from a and b using try_recv (non-blocking receive).
    // If it successfully receives a value (Ok(i)), it sends it through the new channel (tx.send(i)).
    // If there are no more values to receive from a channel (Err(_)), it sets the corresponding flag (a_done or b_done) to true.
    // The loop breaks once both channels have sent all their values (a_done && b_done).
    loop {
        match a.try_recv() {
            Ok(i) => {
                tx.send(i).unwrap();
            }

            Err(_) => {
                a_done = true;
            }
        }
        match b.try_recv() {
            Ok(i) => {
                tx.send(i).unwrap();
            }

            Err(_) => {
                b_done = true;
            }
        }

        if a_done && b_done {
            break;
        }
    }

    rx
}

fn main() {
    let a = as_chan(&vec![1, 3, 5, 7]);

    let b = as_chan(&vec![2, 4, 6, 8]);

    // At this momemnt, both a and b channels
    // would contain all the values as the as_chan function joins the spawned handler.
    // This means that the merge function would return the 1..8 sequence.
    let c = merge(a, b);

    for v in c.iter() {
        println!("{v:?}");
    }
}
