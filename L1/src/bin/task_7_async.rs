use tokio::time::{sleep, Duration};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let token = CancellationToken::new();
    let cloned_token = token.clone();
    // Task that will finish after cancelation received
    let task = tokio::spawn(async move {
        loop {
            // Check the cancelation token
            // Will stop the thread
            tokio::select! {
                _ = cloned_token.cancelled() => {
                    println!("Task is being cancelled");
                    break;
                }
                _ = sleep(Duration::from_secs(1)) => {
                    println!("Task is working...");
                }
            }
        }
    });

    sleep(Duration::from_secs(3)).await;
    token.cancel();

    let _ = task.await;
}
