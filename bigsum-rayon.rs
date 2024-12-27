//ใช้ Rayon:
use rayon::prelude::*;
use std::time::Instant;

fn sum_large_data(data: &[i64]) -> i64 {
    data.par_iter().sum()
}

fn main() {
    let data: Vec<i64> = (1..=1_000_000).collect();

    let start = Instant::now();
    let result = sum_large_data(&data);
    let duration = start.elapsed();

    println!("Sum: {}", result);
    println!("Time taken with Rayon: {:?}", duration);
}
