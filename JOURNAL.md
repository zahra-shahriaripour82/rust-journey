## Week 1 Day 1

- Learned: how to install rust and run first program
- Confused by: rustc
- Open question:---




## Week 1 Day 2
- Learned: `cargo check` only checks code and makes no program; `cargo build` makes it in
  target/debug; `--release` is much faster because of optimizations.
  `=` replaces a value, `+=` adds to it (my challenge bug!).
- Confused by: why I can't run cargo in the rust-journey folder (needs Cargo.toml).
- Question: why did the optimizer remove my loop without black_box?





## Week 1 Day 4: Guessing game
- Learned: built the guessing game with the rand crate (rand@0.8.5, because newer
  versions renamed the functions). `mut` = I can change my own variable;
  `&mut` = I lend my variable to another function so it can change it (read_line).
  Warnings can show real bugs ("never read" = I counted but never printed).
- Confused by: why `&counter += 1` failed (& makes a reference, not the number).
- Question: why does Rust need references at all? Why not just give the variable?


## Week 1 Day 5: Functions, control flow, tuples, arrays
- Learned: a function returns its last expression (no `;`). Adding `;` throws the
  value away and returns `()`. Function signatures always need types (a contract).
  A function can return several values as a tuple: (min, max, avg, sum).
  Started Rustlings: shadowing (`let` again) can change a variable's type; `mut` can't.
- Confused by: statements vs expressions at first; how to start Rustlings.
- Question: how do I return a list whose length isn't fixed (not always 10)?





## Week 2 Day 1: Stack, heap, memory bugs

### The 5 memory bugs (one sentence each)
1. Memory leak: the program never frees memory, so memory use keeps growing.
2. Double free: the program frees the same memory twice, which can corrupt
   memory and crash the program.
3. Use after free: the program frees memory but then still uses it.
4. Dangling pointer: a pointer that points to memory that is already freed.
5. Buffer overflow: the program reads or writes past the end of its memory.

### How my previous language (JavaScript) prevents them
1. Leak: the garbage collector frees memory that nobody uses.
2. Double free: I never free memory myself, the garbage collector does it.
3. Use after free: memory is only freed when nobody uses it anymore.
4. Dangling pointer: same reason as 3.
5. Buffer overflow: it checks every index (arr[10] gives `undefined`,
   Rust stops the program with "index out of bounds").

### What the garbage collector costs
- Cost 1: pauses: the GC sometimes stops or slows the program.
- Cost 2: extra memory: the GC needs spare room to work.
- Cost 3: no control over WHEN memory is cleaned up.

### Learned / Confused by / Question
- Learned: every function call adds a stack frame, and the stack grows down
  (addresses get smaller). The stack is small (~8 MB): too many calls or a
  10 MB array = stack overflow. A Box value on the heap survives after
  its function ends.
- Confused by: recursion in Step 4 (a function calling itself).
- Question: without a garbage collector, how does Rust know when to free
  heap memory?




  ## Week 2 Day 2: Tools (fmt, Clippy, rust-analyzer, dbg!, debugger)
- Learned: `cargo clippy` suggests better Rust: no `return` at the end,
  `sum += n` instead of `sum = sum + n`, `if ok` instead of `if ok == true`.
  In `for n in numbers`, n is the VALUE, not the index, so `sum += n`, not `numbers[n]`.
  The debugger: breakpoint (red dot), F5 = continue, Variables panel, Call Stack
  panel shows the stack frames live (sum on top of main).
  Bug found with the debugger: `while i < numbers.len() - 1` stops one step
  too early, so i = 4 never appears and the last number (50) is never added.
  Fix: `while i < numbers.len()`.
- Confused by: VS Code setup: format on save, rust-analyzer needs the folder
  with Cargo.toml, and F5 while launch.json is open just reopens launch.json.
- Question: why is `for n in numbers` safer than an index loop?


## Week 2 Day 3: Unit converter
- Learned: built a 6-option unit converter with a menu loop and small functions
  (one job each). `stdin` = the keyboard input channel; read_line writes into a
  String (including the Enter key "\n"), .trim() removes it and gives a &str view,
  .to_string() makes an owned copy that can leave the function.
  match needs the same type on both sides, so `match choice.as_str()`.
  Variables in different functions are separate (different stack frames),
  so it's not shadowing.
  Found a copy-paste bug: miles → km printed "miles". The compiler can't catch
  wrong text; only testing can.
- Confused by: String vs &str, and when to use each.
- Question: how do I test my conversion functions automatically?




## Week 2 Day 4: Tests for the unit converter
- Learned: a test is a function marked #[test] inside `#[cfg(test)] mod tests`,
  with `use super::*;` so it can see my functions. `cargo test` runs them all.
  assert_eq!(a, b) checks a == b; on failure it shows "left" (what I got) and
  "right" (what I expected). 0.1 + 0.2 is NOT 0.3 in binary, so for f64 I use
  approx_eq (difference < 1e-9) with assert!.
  Tests make clean-up safe: change code, run cargo test, all green = nothing broke.
- Confused by: my tests had errors: a test with the same name as the function
  (it called itself!), and `1` instead of `1.0` (integer vs f64).
- Question: can tests also check what the menu prints, not just the functions?





## Week 2 Day 5: Practice (Exercism + Rustlings)
- Done: 2 Exercism exercises (Raindrops and ___). Rustlings: not finished,
  quiz1 and primitive_types still to do.
- Learned: a function with `-> String` must RETURN text; println! only prints.
  `match` runs only ONE branch, so when several things can be true at the same
  time (15 → "PlingPlang"), use separate `if`s. `match` compares patterns,
  `if` checks conditions.
  Build text step by step: String::new() + push_str(...), check with is_empty().
  `break` only works inside a loop.
  An if/else as the last expression is the return value: both branches must
  have the same type (n.to_string() and result are both String).
- Confused by: when to use match vs if.



## Week 3 · Day 1: Ownership and moves

**What I learned**
- Variables live on the stack. `Box::new(5)` puts the 5 on the heap,
  and the variable only holds a pointer to it.
- `let b = a;` with a Box is a **move**: a gives ownership to b.
  Only the pointer is copied, not the heap data. After that, a can't be used.
- There are 3 ways to move a Box: `let b = a;`, passing it to a function,
  and returning it from a function.
- Simple values like `i32` are copied, not moved, so both variables still work.
- A moved-from `mut` variable can get a new value: `a = Box::new(2);`
- I can't free memory myself in Rust. The owner frees it automatically,
  exactly once.

**Why Rust does this**
If a and b both owned the same Box, both would free it at the end.
That's a double free (or a use after free). One owner means one free.

**My predictions**
- First try, Programs 1 to 5: I got all of them wrong, because I didn't know
  that `let b = a;` moves ownership.
- After learning the rule, I got 3/3 on the new programs (A, B, C).

**Error to remember**
E0382 "borrow of moved value": I used a variable after its value moved away.