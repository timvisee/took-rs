use std::thread::sleep;
use std::time::Duration;

use took::Take;

pub fn main() {
    // Create the stopwatch, start automatically
    let took = Take::new();
    println!("Sleeping 1 second...");

    // Block for a second
    sleep(Duration::from_secs(1));

    // Report elapsed time in human readable format
    println!("Done, took {}", took.took());
}
