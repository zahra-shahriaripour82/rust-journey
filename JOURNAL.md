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