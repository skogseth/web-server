use color_eyre::eyre::{eyre, WrapErr, Result};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let mut handles = Vec::new();

    for i in 0..3 {
        let handle = tokio::spawn(async move {
            task(i).await;
        });
        handles.push(handle);
    }

    


    for handle in handles {
        handle.await.wrap_err_with(|| format!("Failed to gather task"))?;
    }

    Ok(())
}

async fn task(i: u64) {
    println!("Hello from task {i}, beginning computation...");
    tokio::time::sleep(Duration::from_secs(i)).await;
    println!("Task {i} signing off.");
}