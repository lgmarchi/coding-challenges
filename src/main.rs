use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Starting...");
    let result;

    let timeout_duration = Duration::from_secs(3);
    let timeout = sleep(timeout_duration);

    tokio::pin!(timeout);

    tokio::select! {
        res = fast_task() => {
            println!("Fast task finished first!");
            result = res;
        }
        res = slow_task() => {
            println!("Slow task finished first!");
            result = res;
        }
        _ = &mut timeout => {
            println!("Timeout reached!");
            result = "timeout".to_string();
        }
    }

    println!("{result:?}");
}

async fn fast_task() -> String {
    println!("Fast task starting...");
    sleep(Duration::from_secs(2)).await;
    "Fast task finished.".to_string()
}

async fn slow_task() -> String {
    println!("Slow task starting...");
    sleep(Duration::from_secs(5)).await;
    "Slow task finished.".to_string()
}
