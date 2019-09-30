use took::Take;

pub fn main() {
    // Create the stopwatch, start automatically
    let took = Take::new();

    // Simulate heavy task running 1 second
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Report elapsed time in human readable format
    println!("Done! Took {}", took.took());
}
