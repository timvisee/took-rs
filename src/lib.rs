use std::fmt;
use std::time::{Duration, Instant};

/// Simple struct tracking elapsed time.
///
/// On creation this struct immediately starts tracking elapsed time.
#[derive(Clone, Debug)]
pub struct Timer {
    /// The instant this stopwatch was started at.
    start: Instant,
}

impl Timer {
    /// Construct a new `Timer` stopwatch, start immediately.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the elapsed time.
    pub fn took(&self) -> Took {
        Took::from_std(self.start.elapsed())
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            start: Instant::now(),
        }
    }
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.took().fmt(f)
    }
}

/// Defines elasped time.
#[derive(Clone, Debug)]
pub struct Took {
    /// Elapsed time as duration.
    elapsed: Duration,
}

impl Took {
    /// Construct `Took` from `Duration` in `std`.
    pub fn from_std(elapsed: Duration) -> Self {
        Self { elapsed }
    }

    /// Extract the inner `Duration`.
    pub fn into_std(self) -> Duration {
        self.elapsed
    }

    /// Get a reference to the inner `Duration`.
    pub fn as_std(&self) -> &Duration {
        &self.elapsed
    }

    /// Print the elapsed time with a description.
    ///
    /// Prints the following formatted text to `stdout`:
    ///
    /// ```skip
    /// description took 0 s
    /// ```
    pub fn describe(&self, description: &str) {
        println!("{} took {}", description, self);
    }
}

impl fmt::Display for Took {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let elapsed = self.elapsed;
        let secs = u128::from(elapsed.as_secs());
        let millis = elapsed.as_millis();
        let micros = elapsed.as_micros();
        let nanos = elapsed.as_nanos();

        let (major, minor, t) = if secs > 0 {
            (secs, millis, "s")
        } else if millis > 0 {
            (millis, micros, "ms")
        } else if micros > 0 {
            (micros, nanos, "μs")
        } else {
            (nanos, nanos * 1000, "ns")
        };

        let time = major as f64 + (minor - major * 1000) as f64 / 1000.0;
        f.pad(&format!("{:.2} {}", time, t))
    }
}

/// Measure run time of given function, return elapsed time.
///
/// Returns `Took` along with the function result.
#[must_use]
pub fn took<T, F>(f: F) -> (Took, T)
where
    F: FnOnce() -> T,
{
    let take = Timer::new();
    let out = f();
    (take.took(), out)
}

/// Measure run time of given function, print elapsed time.
///
/// Prints the following formatted text to `stdout`:
///
/// ```skip
/// description took 0 s
/// ```
///
/// Returns the function result.
#[must_use]
pub fn took_print<T, F>(description: &str, f: F) -> T
where
    F: FnOnce() -> T,
{
    let (took, out) = took(f);
    eprintln!("{} took {}", description, took);
    out
}
