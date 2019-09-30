use std::fmt;
use std::io::{Cursor, Write};
use std::time::{Duration, Instant};

/// Simple struct tracking elapsed time.
#[derive(Clone, Debug)]
pub struct Take {
    /// The instant this stopwatch was started at.
    start: Instant,
}

impl Take {
    /// Construct a new `Take` stopwatch, start immediately.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the elapsed time.
    pub fn took(&self) -> Took {
        Took::from_std(self.start.elapsed())
    }
}

impl Default for Take {
    fn default() -> Self {
        Self {
            start: Instant::now(),
        }
    }
}

impl fmt::Display for Take {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.took().fmt(f)
    }
}

/// Defines elasped time.
pub struct Took {
    elapsed: Duration,
}

impl Took {
    /// Construct `Took` from `Duration` in `std`.
    pub fn from_std(elapsed: Duration) -> Self {
        Self { elapsed }
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
        let buff: &mut [u8] = &mut [0; 128];
        f.pad({
            let mut cursor = Cursor::new(buff);
            write!(cursor, "{:.2} {}", time, t).unwrap();
            let len = cursor.position();
            let buff = cursor.into_inner();
            std::str::from_utf8(&buff[..len as usize]).unwrap()
        })
    }
}

/// Measure the run time of the given function.
///
/// Returns `Took` along with the function result.
pub fn took<T, F>(f: F) -> (Took, T)
where
    F: FnOnce() -> T,
{
    let take = Take::new();
    let out = f();
    (take.took(), out)
}
