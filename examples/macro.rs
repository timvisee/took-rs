#[macro_use]
extern crate took_macro;

pub fn main() {
    my_function();
    other_function();
}

#[took]
pub fn my_function() {
    // Simulate heavy task running 1 second
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Written to stderr on return:
    // my_function() took 1.00 s
}


#[took(description = "Render finished,")]
pub fn other_function() {
    // Simulate heavy task running 1 second
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Written to stderr on return:
    // Render finished, took 1.00 s
}
