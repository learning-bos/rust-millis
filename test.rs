use std::time::{Duration, Instant};

fn main() {
    // Start the timer
    let start = Instant::now();

    // Simulate some code execution
    std::thread::sleep(Duration::from_secs(2) + Duration::from_millis(500) + Duration::from_micros(250));

    // Calculate the elapsed duration
    let duration = start.elapsed();

    // Extract whole seconds
    let seconds = duration.as_secs();

    // Extract milliseconds and fractional milliseconds
    let millis = duration.subsec_millis(); // Whole milliseconds
    let nanos = duration.subsec_nanos(); // Total nanoseconds
    let fractional_millis = nanos as f64 / 1_000_000.0; // Convert nanoseconds to fractional milliseconds

    // Print the result
    println!(
        "Elapsed time: {} seconds, {} milliseconds, and {:.3} fractional milliseconds",
        seconds,
        millis,
        fractional_millis
    );
}
