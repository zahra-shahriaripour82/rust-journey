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

- Q : What does rustc produce, and why can it run on a computer without Rust installed?
  A:it was compiled ahead of time (nothing is left to translate),
the standard library parts you use are copied into the file,
there’s no runtime or garbage collector needed.




## Week 1 Day 6: quiz review (🟡 and ❌)

- Q: Why can a Rust binary run without Rust installed? (3 reasons)
  A: 1) Compiled ahead of time to machine code. 2) The std library parts are copied
     into the file. 3) No runtime or garbage collector needed.

- Q: cargo build vs cargo build --release?
  A: build = DEBUG program: compiles fast, runs slow, for development.
     --release = OPTIMIZED program: compiles slow, runs fast, for production.

- Q: Why is release so much faster?
  A: The optimizer rewrites the code (registers, loop unrolling, SIMD)
     and release removes the overflow checks on every +.

- Q: 250u8.saturating_add(10)?
  A: 255. Saturating = stops at the limit (a full glass of water).

- Q: Integer overflow in debug vs release?
  A: Debug = panic (crash with an error). Release = silent wrap-around (wrong number).
     Debug checks every + to find bugs; release skips the checks for speed.

- Q: 300 as u8? Why?
  A: 44. `as` keeps only the lowest 8 bits = 300 - 256. No error, no warning!

- Q: The 4 memory regions? Where do a local i32 and Box::new(5) live?
  A: Code, Static, Stack, Heap. Local i32 → stack. The 5 in a Box → heap
     (the Box pointer itself → stack).

- Q: mut vs shadowing: the difference?
  A: mut = same variable, type can't change.
     Shadowing (let again) = new variable, same name, type CAN change.

- Q: rand = "0.8.5": which versions can Cargo use?
  A: >= 0.8.5 and < 0.9.0. "Same middle number = safe." Before 1.0, the middle
     number means breaking changes (rand 0.10 broke the Book's code).

- Q: Why are indexes usize, not i32?
  A: usize = the size of a memory address, so it can index any position.
     And an index can never be negative.

- Q: arr[10] on a 5-element array: Rust vs C?
  A: Rust checks and panics ("index out of bounds"). C silently reads other
     memory → buffer overflow, a big security hole.



     ## Week 2 Day 1
- Q: What is a stack frame? When is it created and removed?
  A: A block of stack memory for one function call's local variables.
     Created when the function is called, removed when it returns.

- Q: Stack vs heap: speed, size, and how long data lives?
  A: Stack = very fast, small (~8 MB), data dies when the function returns.
     Heap = slower, huge, data lives until someone frees it.

- Q: 3 reasons data goes on the heap?
  A: 1) Size unknown at compile time (user input, growing list).
     2) Data must outlive the function. 3) Data is too big for the stack.

- Q: The 5 classic memory bugs?
  A: Leak (never freed), double free (freed twice), use after free (used after
     freeing), dangling pointer (points to freed memory), buffer overflow
     (reads/writes past the end). Hotel 🏨: no check-out, check out twice,
     re-enter the room after checking out, key to a torn-down room, walking
     into the wrong rooms.

- Q: What does a garbage collector cost?
  A: Pauses, extra memory, and no control over WHEN memory is freed.

  - Q: Why does Rust refuse `fn dangle() -> &i32 { let x = 5; &x }`?
  A: x lives in dangle's stack frame. When dangle returns, the frame is removed,
     so &x would point to memory that doesn't exist (a dangling pointer).
     Rust catches this at COMPILE time; C doesn't.




     ## Week 2 Day 2
- Q: Compiler vs Clippy: what does each check?
  A: Compiler = is the code CORRECT? Clippy = is it GOOD, idiomatic Rust?
     Run `cargo clippy` before every commit.

- Q: What does dbg!(x * 2) print and return?
  A: Prints file:line, the code, and its value (to stderr).
     Returns the value, so `let y = dbg!(x * 2);` still works.

- Q: for i in 0..numbers.len() vs for n in numbers?
  A: First: i = positions (0, 1, 2...), use numbers[i]. Second: n = the values
     directly. The second can't go out of bounds or be off by one.

- Q: Step Over vs Step Into vs Step Out?
  A: Over (F10) = run this line. Into (F11) = go inside the called function.
     Out (Shift+F11) = finish this function, back to the caller.