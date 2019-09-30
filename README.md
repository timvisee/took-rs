# `took`: easily measure & report elapsed time
I always find measuring run time of code and reporting it in a human readable
format troublesome.

This crate provides a few simple interfaces to do just that.

## Examples
```rust
use took::Timer;



// # Measure & report manually using Timer stopwatch

let timer = Timer::new();
// Run heavy task
println!("Done! Took {}", timer.took());

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

## Usage
Add the dependencies in your `Cargo.toml`. The `took-macro` dependency is only
required if you'll be using the `#[took]` attribute macro.

```Cargo.toml
[dependencies]
took = "0.1"
took-macro = "0.1" # if using macros
```

Import and start using:

```rust
use took::{Timer, took};

let timer = Timer::new();
println!("Done! Took {}", timer.took());

let (took, result) = took(|| {
    // Run heavy task
});
println!("Done, took {}", took);
```

If you'll be using `#[took]` attribute macro, explicitly import it:

```rust
#[macro_use]
extern crate took_macro;

#[took]
pub fn function_a() {}

#[took(description = "Some function")]
pub fn function_a() {}
```

## License
This project is released under the MIT license.
Check out the [LICENSE](LICENSE) file for more information.
