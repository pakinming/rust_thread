use tokio::task;
use std::time::Instant;

async fn sum_large_data(data: Vec<i64>) -> i64 {
    let chunk_size = data.len() / 4;
    let mut tasks = vec![];

    for chunk in data.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        let task = task::spawn(async move {
            chunk.iter().sum::<i64>()
        });
        tasks.push(task);
    }

    let mut result = 0;
    for task in tasks {
        result += task.await.unwrap();
    }

    result
}

#[tokio::main]
async fn main() {
    let data: Vec<i64> = (1..=1_000_000).collect();

    let start = Instant::now();
    let result = sum_large_data(data).await;
    let duration = start.elapsed();

    println!("Sum: {}", result);
    println!("Time taken with Tokio: {:?}", duration);
}
