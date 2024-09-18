use flume::{unbounded, Receiver};
use tokio::io;
use tokio::io::AsyncReadExt;
use tokio::signal;

async fn worker(id: u64, load_rx: Receiver<usize>, shutdown_rx: Receiver<()>) {
    loop {
        tokio::select! {
             data_res = load_rx.recv_async() => {
                match data_res  {
                    Ok(data) =>  {
                        println!("worker with id: {}; processing: {}", id, data);
                        // imitating load
                        tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;
                    },
                    Err(e) => eprintln!("worker with id: {}; failed with err: {}", id, e)
                }

             },
             _ = shutdown_rx.recv_async() => {
                 println!("worker with id: {} shutting down.", id);
                 break;
             }
        };
    }
}

#[tokio::main]
async fn main() {
    let worker_count = io::stdin().read_u64().await.expect("Failed to read line");

    let (sender_load, receiver_load) = unbounded();
    let (sender_shutdown, receiver_shutdown) = unbounded();
    let worker_handlers = (0..worker_count)
        .map(|id| {
            let load_rx_cln = receiver_load.clone();
            let shutdown_rx_cln = receiver_shutdown.clone();
            tokio::spawn(async move {
                worker(id, load_rx_cln, shutdown_rx_cln).await;
            })
        })
        .collect::<Vec<_>>();

    // Create CTRL+C signal handler
    let shutdown_tx_cln = sender_shutdown.clone();
    let ctrl_c_handler = tokio::spawn(async move {
        // completes after receiving
        signal::ctrl_c()
            .await
            .expect("Got an error while receiving a CTRL+C signal.");
        println!("Shutting down");
        // Fan out worker_count shutdown messages
        for _ in 0..worker_count {
            shutdown_tx_cln
                .send_async(())
                .await
                .expect("Failed to send Msg::Shutdown");
        }
    });

    for i in 0..42 {
        sender_load
            .send_async(i)
            .await
            .expect("Error sending Load to the channel.")
    }

    for handle in worker_handlers {
        handle.await.expect("Worker task panicked");
    }

    ctrl_c_handler.await.expect("CTRL+C handler task panicked");
}
