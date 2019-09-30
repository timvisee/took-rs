use std::thread::sleep;
use std::time::Duration;

use took::took;

pub fn main() {
    // Time the given function
    let (took, _result) = took(|| {
        // Block for a second
        println!("Sleeping 1 second...");
        sleep(Duration::from_secs(1));
    });

    // Report elapsed time in human readable format
    println!("Done, took {}", took);
}
