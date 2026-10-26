- Q: What does `rustc` produce? A: a native exectue prgram
- Q: Why does `println!` have a `!`? A: it's a macro
- Q: Why can a Rust binary run on a machine without Rust installed? A: ...
- Q: Why can a Rust binary run without Rust installed?
  A: Compiled ahead of time to machine code + std library copied into the file + no runtime/GC.
     Needs only: same OS, same CPU, system libc.

## Week 1 Day 2
- Q: cargo check vs cargo build vs cargo build --release?
  A: check = only finds errors, no program (fastest). build = debug program in target/debug.
     --release = optimized, much faster program in target/release.

- Q: Why is a release build so much faster than debug?
  A: The optimizer rewrites the code (registers, loop unrolling, SIMD) and removes
     overflow checks. Debug keeps code simple for debugging.

- Q: Shadowing vs mut: what's the difference?
  A: mut = change the SAME variable, type can't change.
     Shadowing (let again) = a NEW variable with the same name, type CAN change.