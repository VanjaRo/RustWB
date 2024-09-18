use flume::{unbounded, Receiver};
use tokio::io::{self, AsyncReadExt};

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
    let timeout_seconds = io::stdin().read_u64().await.expect("Failed to read line");

    let worker_count = 1;
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

    // Create a thread that will fan out shutdown signal after a timeout
    let sndr_tx_cln = sender_shutdown.clone();
    let timeout_handler = tokio::spawn(async move {
        // completes after receiving
        tokio::time::sleep(tokio::time::Duration::from_secs(timeout_seconds)).await;
        println!("Shutting down");
        // Fan out worker_count shutdown messages
        for _ in 0..worker_count {
            sndr_tx_cln
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

    timeout_handler
        .await
        .expect("Timeout handler task panicked");
}
