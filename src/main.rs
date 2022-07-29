use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut handles = Vec::new();

    for i in 0..3 {
        let handle = tokio::spawn(async move {
            task(i).await;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.expect("Error retrieving task.");
    }
}

async fn task(i: u64) {
    println!("Hello from task {i}, beginning computation...");
    tokio::time::sleep(Duration::from_secs(i)).await;
    println!("Task {i} signing off.");
}