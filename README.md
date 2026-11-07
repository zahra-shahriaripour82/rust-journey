# rust-journey

My daily Rust learning journey: exercises, projects, and notes from a 22-week plan, from ownership to async.



## Week 2 Day 4
- Q: How do I write and run a test?
  A: #[cfg(test)] mod tests { use super::*; #[test] fn name() { assert_eq!(..) } }
     Run with `cargo test`.

- Q: In a failing assert_eq!, what are "left" and "right"?
  A: left = the first value (what my function returned),
     right = the second value (what I expected).

- Q: Why never compare f64 with ==? What instead?
  A: Decimals like 0.1 can't be stored exactly in binary → 0.1 + 0.2 = 0.30000000000000004.
     Use: (a - b).abs() < 1e-9

- Q: Why must a test's name differ from the function it tests?
  A: Inside mod tests, the test's name hides the real function,
     so calling it calls the test itself.

- Q: 1 vs 1.0?
  A: 1 = integer, 1.0 = f64. Rust NEVER converts between them automatically.