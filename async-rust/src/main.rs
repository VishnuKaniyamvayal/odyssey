use std::time::Instant;
use tokio::task::{self, JoinHandle};

/// Number of parallel jobs to run.
const JOB_COUNT: usize = 10;

/// Upper bound for the summation loop. Using a named constant makes
/// it easy to tweak the workload size in one place.
const ITERATIONS: u64 = 1_000_000;

/// A CPU-bound workload: sums integers in `0..ITERATIONS`.
///
/// `wrapping_add` is used so the loop cannot be optimized away and
/// won't panic on overflow in debug builds.
fn large_computation() -> u64 {
    (0..ITERATIONS).fold(0u64, |acc, i| acc.wrapping_add(i))
}

#[tokio::main]
async fn main() {
    let start = Instant::now();

    // `spawn_blocking` moves CPU-heavy work off the async worker threads
    // and onto Tokio's dedicated blocking thread pool, so the runtime
    // stays responsive and the jobs can actually run in parallel.
    let handles: Vec<JoinHandle<u64>> = (0..JOB_COUNT)
        .map(|_| task::spawn_blocking(large_computation))
        .collect();

    // Await every handle and collect the results. `unwrap` here surfaces
    // any panic that occurred inside a spawned task.
    let mut results = Vec::with_capacity(JOB_COUNT);
    for handle in handles {
        results.push(handle.await.unwrap());
    }

    let elapsed = start.elapsed();

    println!("Completed {JOB_COUNT} jobs in {elapsed:?}");
    println!("First result: {}", results[0]);
}