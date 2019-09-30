use took::took;

pub fn main() {
    // Time the given function
    let (took, _result) = took(|| {
        // Simulate heavy task running 1 second
        std::thread::sleep(std::time::Duration::from_secs(1));
    });

    // Report elapsed time in human readable format
    println!("Done, took {}", took);
}
