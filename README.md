# took: easily measure & report elapsed time
I always find measuring run time of code and reporting it in a human readable
format troublesome.

This crate provides a few simple interfaces to do just that.

## Examples
```rust
use took::Take;



// # Measure & report manually using Take stopwatch

let took = Take::new();
// Run heavy task
println!("Done! Took {}", took.took());

// Prints:
// Done! Took 1.00 s



// # Measure a function, report manually

let (took, result) = took(|| {
    // Run heavy task
});
println!("Done, took {}", took);

// Prints:
// Done! Took 1.00 s



// # Measure & report a function automatically using attribute

#[macro_use]
extern crate took_macro;

my_function();
other_function();

#[took]
pub fn my_function() {
    // Run heavy task
}


#[took(description = "Render finished,")]
pub fn other_function() {
    // Run heavy task
}

// Prints:
// my_function() took 1.00 s
// Render finished, took 1.00 s
```

## License
This project is released under the MIT license.
Check out the [LICENSE](LICENSE) file for more information.
