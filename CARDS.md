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


     ## Week 1 Day 3 
- Q: What does 250u8 + 10 give with checked_add, wrapping_add and saturating_add?
A:checked_add gives None (close!). Rust has no null. “No value” is written None, and you’ll see why that’s safer in Week 7.
wrapping_add gives 4, not 0. Count on the u8 “clock” of 256 values: 250 → 251, 252, 253, 254, 255 (5 steps), then wrap to 0, 1, 2, 3, 4 (5 more steps). The quick way: 260 − 256 = 4.
saturating_add gives 255 ✅
 - Q:What does 300 as u8 give, and why?
 A:A u8 only has room for 8 bits (0 to 255). as doesn’t check, it just keeps the lowest 8 bits and drops the rest, which is the same as 300 − 256 = 44. No error, no warning. That’s why as is dangerous when the value might not fit.
 - Q:Stack or heap: where does the 5 in Box::new(5) live? Which one is cleaned up automatically?
 A:The 5 lives on the heap ✅
The Box itself (the pointer, the address of the 5) lives on the stack. So a Box is like a note on the stack saying “your 5 is over there in the heap”.
The stack is cleaned up automatically





## Week 1 Day 4
- Q: `mut` vs `&mut`: when do I use each?
  A: mut = I change my own variable (counter += 1).
     &mut = another function must change my variable (read_line(&mut guess)).

- Q: `rand = "0.8.5"` in Cargo.toml: which versions can Cargo use?
  A: >= 0.8.5 and < 0.9.0. Before 1.0, the middle number is the "breaking" one.

- Q: Cargo.toml vs Cargo.lock?
  A: Cargo.toml = what I asked for (a range). Cargo.lock = exact versions used.
     Commit Cargo.lock for programs.

- Q: Why `.trim()` before `.parse()`?
  A: read_line keeps the Enter key ("50\n"), and "50\n" is not a valid number.




  ## Week 1 Day 5
- Q: Expression vs statement? What does `;` do?
  A: Expression = gives a value (a + b). Statement = does something, no value (let x = 5;).
     `;` throws the value away. No `;` on the last line = the return value.

- Q: Why must function signatures have types, when `let` doesn't need them?
  A: The signature is a contract: you can understand and check a function by
     reading one line, without reading its body or its callers.

- Q: Why is `135 / 10` equal to 13, and how do I get 13.5?
  A: i32 / i32 = integer division (drops decimals). Use `sum as f64 / 10.0`.

- Q: Tuple vs array?
  A: Tuple = different types, fixed size, t.0 / t.1. Array = same type,
     fixed size [i32; 10], a[0]. Both live on the stack.