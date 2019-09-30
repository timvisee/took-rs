use took::Timer;

pub fn main() {
    // Create the stopwatch, start automatically
    let took = Timer::new();

    // Simulate heavy task running 1 second
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Report elapsed time in human readable format
    println!("Done! Took {}", took.took());
}
