// use rayon::prelude::*;
// use std::thread;
// use std::time::Instant;
// use core_affinity;

// fn fibonacci(n: u64) -> u64 {
//     if n <= 1 {
//         n
//     } else {
//         fibonacci(n - 1) + fibonacci(n - 2)
//     }
// }

// fn main() {
//     // add value 45 x 10times


//     let inputs: Vec<u64> = vec![47, 47, 47, 47, 47, 47, 47]; // ขนาดงานหนักพอที่จะเห็นผล

//     let start = Instant::now(); // เริ่มจับเวลา

//     inputs.par_iter().for_each(|&n| {
//         let thread_id = thread::current().id();
//         let core_id = core_affinity::get_core_ids()
//             .and_then(|mut ids| ids.get(0).cloned()); // ดึง Core ID

//         if let Some(core) = core_id {
//             println!(
//                 "Thread {:?} on Core {} is processing Fibonacci({})",
//                 thread_id, core.id, n
//             );
//         } else {
//             println!("Thread {:?} (Core unknown) is processing Fibonacci({})", thread_id, n);
//         }

//         let result = fibonacci(n);
//         println!(
//             "Thread {:?} finished Fibonacci({}) = {}",
//             thread_id, n, result
//         );
//     });

//     let duration = start.elapsed(); // วัดระยะเวลาการทำงาน
//     println!("Time taken: {:?}", duration);
// }


// use rayon::prelude::*;
// use std::time::Instant;

// // การคำนวณ Fibonacci ด้วยการคำนวณแบบ Recursive
// fn fibonacci(n: u64) -> u64 {
//     if n <= 1 {
//         return n;
//     }
//     fibonacci(n - 1) + fibonacci(n - 2)
// }

// fn main() {
//     let inputs: Vec<u64> = vec![45, 49, 42, 43, 44 ,49, 51, 42, 43, 44 ]; // ตัวเลขที่สูงขึ้นเพื่อทดสอบการประมวลผลหนัก

//     let start_ray = Instant::now();
//     let results: Vec<u64> = inputs
//         .par_iter() // ใช้ par_iter สำหรับการทำงานขนาน
//         .map(|&n| fibonacci(n))
//         .collect();
//     let duration_ray = start_ray.elapsed();
    
//     for (n, result) in inputs.iter().zip(results.iter()) {
//         println!("Rayon Fibonacci({}) = {}", n, result);
//     }
    
//     println!("Rayon Time taken: {:?}", duration_ray);
// }



use rayon::prelude::*;
use std::time::Instant;
// ฟังก์ชันคำนวณหาผลรวมของเลขยกกำลัง
fn heavy_computation(n: u64) -> u64 {
    (1..=n).fold(0, |acc, x| acc + (x * x))
}

// fn main() {
//     let inputs: Vec<u64> = vec![100_000, 200_000, 300_000, 400_000, 500_000]; // ข้อมูลขนาดใหญ่

//     let start = Instant::now();

//     // ใช้ Rayon เพื่อทำงานขนาน
//     let results: Vec<u64> = inputs
//         .par_iter() // ใช้ par_iter แทน iter เพื่อให้ทำงานแบบขนาน
//         .map(|&n| heavy_computation(n)) // ใช้ map เพื่อทำการคำนวณ
//         .collect(); // เก็บผลลัพธ์ทั้งหมดใน Vec

//     let duration = start.elapsed();
    
//     println!("Results: {:?}", results);
//     println!("Time taken with Rayon: {:?}", duration);
// }

fn main() {
    let inputs: Vec<u64> = vec![100_000, 200_000, 300_000, 400_000, 500_000]; // ข้อมูลขนาดใหญ่

    let start = Instant::now();

    // ใช้ iter() แทน par_iter() เพื่อทำงานแบบ sequential
    let results: Vec<u64> = inputs
        .iter()
        .map(|&n| heavy_computation(n))
        .collect();

    let duration = start.elapsed();
    
    println!("Results: {:?}", results);
    println!("Time taken sequentially: {:?}", duration);
}
