[![Build status on GitLab CI][gitlab-ci-master-badge]][gitlab-ci-link]
[![Newest release on crates.io][crate-version-badge]][crate-link]
[![Documentation][docs-badge]][docs]
[![Number of downloads on crates.io][crate-download-badge]][crate-link]
[![Project license][crate-license-badge]](LICENSE)

[crate-download-badge]: https://img.shields.io/crates/d/took-rs.svg
[crate-license-badge]: https://img.shields.io/crates/l/took-rs.svg
[crate-link]: https://crates.io/crates/took-rs
[crate-version-badge]: https://img.shields.io/crates/v/took-rs.svg
[docs-badge]: https://docs.rs/took-rs/badge.svg
[docs]: https://docs.rs/took-rs
[gitlab-ci-link]: https://gitlab.com/timvisee/took-rs/pipelines
[gitlab-ci-master-badge]: https://gitlab.com/timvisee/took-rs/badges/master/pipeline.svg

# `took`: easily measure & report elapsed time
I always find measuring run time of code and reporting it in a human readable
format troublesome.

This crate provides a few simple interfaces to do just that.

## Examples
- Measure & report manually using Timer stopwatch:

  ```rust
  use took::Timer;

  let timer = Timer::new();
  // Run heavy task
  println!("Done! Took {}", timer.took());

  // Prints:
  // Done! Took 1.00 s
  ```

- Measure a function, report manually:

  ```rust
  use took::took;

  let (took, result) = took(|| {
      // Run heavy task
  });
  println!("Done, took {}", took);

  // Prints:
  // Done! Took 1.00 s
  ```

- Measure & report a function automatically using attribute:

  ```rust
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

## TODO
- Support `#[took]` attribute for almost anything
  (function call, blocks, if-statements, ...)
- Time formatting configurability
- Use more precise timers
- Print elapsed time to more than just `stderr`

## License
This project is released under the MIT license.
Check out the [LICENSE](LICENSE) file for more information.
