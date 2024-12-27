// use tokio::task;
// use std::thread;
// use std::time::Instant;

// fn fibonacci(n: u64) -> u64 {
//     if n <= 1 {
//         n
//     } else {
//         fibonacci(n - 1) + fibonacci(n - 2)
//     }
// }

// #[tokio::main]
// async fn main() {
//     let inputs: Vec<u64> = vec![47, 47, 47, 47, 47, 47, 47];

//     let start = Instant::now(); // เริ่มจับเวลา

//     let tasks: Vec<_> = inputs
//         .into_iter()
//         .map(|n| {
//             task::spawn(async move {
//                 let thread_id = thread::current().id();
//                 println!("Thread {:?} is processing Fibonacci({})", thread_id, n);
//                 let result = fibonacci(n);
//                 println!("Thread {:?} finished Fibonacci({}) = {}", thread_id, n, result);
//             })
//         })
//         .collect();

//     futures::future::join_all(tasks).await;

//     let duration = start.elapsed(); // วัดระยะเวลาการทำงาน
//     println!("Time taken: {:?}", duration);
// }



// use tokio::task;
// use std::time::Instant;

// // การคำนวณ Fibonacci ด้วยการคำนวณแบบ Recursive
// fn fibonacci(n: u64) -> u64 {
//     if n <= 1 {
//         return n;
//     }
//     fibonacci(n - 1) + fibonacci(n - 2)
// }

// #[tokio::main]
// async fn main() {
//     let inputs: Vec<u64> = vec![45, 49, 42, 43, 44 ,49, 51, 42, 43, 44 ]; // ตัวเลขที่สูงขึ้นเพื่อทดสอบการประมวลผลหนัก

//     let start_tokio = Instant::now();
//     let tasks: Vec<_> = inputs
//         .into_iter()
//         .map(|n| {
//             task::spawn(async move {
//                 let result = fibonacci(n);
//                 println!("Tokio Fibonacci({}) = {}", n, result);
//             })
//         })
//         .collect();

//     // รอให้ tasks ทั้งหมดทำงานเสร็จ
//     futures::future::join_all(tasks).await;
    
//     let duration_tokio = start_tokio.elapsed();
//     println!("Tokio Time taken: {:?}", duration_tokio);
// }


use tokio::task;
use std::time::Instant;

async fn sum_large_data(data: Vec<usize>) -> usize {
    let chunk_size = data.len() / 4;
    let mut tasks = vec![];

    for chunk in data.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        let task = task::spawn(async move {
            chunk.iter().sum::<usize>()
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

    let start = Instant::now();
    let data: Vec<usize> = (1..=1000_000_000).collect();

    
    let result = sum_large_data(data).await;
    let duration = start.elapsed();

    println!("Sum: {}", result);
    println!("Time taken with Tokio: {:?}", duration);
}
