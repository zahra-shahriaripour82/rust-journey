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

- Done: 2 Exercism exercises (Raindrops and \_\_\_). Rustlings: not finished,
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

  The goal of ownership is to prevent undefined behavior, like double free and use after free, at compile time.

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

## Week 3 · Day 2: Drop, String/Vec moves, clone

**What I learned**

- A value is **dropped** (freed) when its owner reaches the end of its scope `}`.
  Only the owner frees it, exactly once.
- Pointing to a box is not enough. Only the **owner** frees it.
  That's why `let b = a;` doesn't cause a double free: only b owns it.
- `String`, `Vec` and `HashMap` have a box inside (their data is on the heap),
  so they **move** just like `Box`.
- After a move, the old variable may point to freed memory
  (like `first` after `push_str` in the Ferris example). Rust forbids using it,
  so there's no use after free.
- `.clone()` is a **deep copy**: it copies the heap data too. `let b = a;` is
  a **shallow copy** of the pointer, plus a move.
- Clone costs time and memory, so Rust never does it automatically.
- Moving a value inside a loop fails the 2nd time, because it's already gone.

**My predictions**

- Step 3: 5/5 correct! (Yesterday 0/5.)

**Rustlings**

- move_semantics 1 to 3: \_\_\_ (what I fixed: `mut`, `.clone()`, `mut` parameter)

**Errors to remember**

- E0382: used a value after it moved
- E0425: cannot find value, because the variable is out of its scope

## Week 3 · Day 3: Copy vs move, and Drop

**Copy vs move**

- Copy types live only on the stack and own nothing on the heap:
  integers, floats, `bool`, `char`, and tuples/arrays made only of those.
  Rust copies them automatically, so both variables still work.
- Types that own heap memory (`String`, `Vec`, `Box`) are NOT Copy. They move.
- My easy test: "Does it own something on the heap?" Yes means it moves.
- One non-Copy part makes the whole thing move: `(i32, String)` moves.

**Why String can't be Copy**
A String is a small card on the stack (address, length, capacity).
The letters are on the heap. A Copy would copy only the card, so there would be
2 cards with the same address and 2 owners of 1 heap buffer.
At the end, both would free it, which is a double free (undefined behavior).
So Rust gives only 2 options: move (1 owner) or clone (2 separate copies).

**Watching drops with `impl Drop`**

- `impl Drop` runs my code automatically when a value is freed.
- A value is dropped at the `}` of its owner's scope, even inside an inner block.
- At the end of a scope, drops happen in REVERSE order (last made, first dropped).
- Moved into a function: dropped when THAT function ends.
- Moved in and returned: not dropped in the function. The new owner drops it later.
- `let b = a;`: only ONE drop. Ownership prevents a double free, and now I saw it!
- `a = new_value;`: the old value is dropped immediately, so there's no leak.

## Week 3 · Day 4: Ownership through functions

**What I learned**

- Passing a value to a function works like `let parameter = argument;`:
  a `String`/`Vec` MOVES in, and an `i32` is COPIED.
- If a function takes ownership and doesn't give it back, the value is
  dropped when that function ends.
- Returning a value MOVES it out to the caller, so it is NOT dropped
  when the function ends.
- To compute something AND keep my String, I return both in a tuple:
  `fn count_vowels(text: String) -> (String, usize)` and then
  `let (name, vowels) = count_vowels(name);`
- A function that changes a value can just return it:
  `fn shout(mut text: String) -> String` (same pattern as `fill_vec`).

**What I built** (week3_give_back)

- `count_vowels`: returns (String, usize)
- `shout`: uppercase + "!", returns the String
- `sum_and_len`: returns (Vec<i32>, i32, usize)

**Why this is clumsy**
For every function I had to make the return type bigger, unpack a tuple,
and remember to give the value back, just to count vowels!
Next week, borrowing (`&`) will let a function LOOK at a value
without owning it, so nothing needs to be given back.

**New tools**

- `.chars()`: goes through a String one char at a time
- `"aeiou".contains(c)`: checks if a char is in a string
- `.to_uppercase()`, `.push('!')`

## Week 4 · Day 1: References and borrowing (part 1)

**What a reference is**

- `&x` creates a reference: a pointer that does NOT own the data.
  This is called borrowing. It's like lending a book instead of giving it away.
- `fn count_vowels(text: &String)` only looks at the String. When the
  function ends, nothing is freed, so there's no need to give it back
  with tuples like last week.

**Dereferencing (`*`)**

- A Box is like a locker key. `*x` means "open the locker":
  `*x` reads the value, and `*x += 1` changes it.
- I rarely write `*` myself. With the dot (`x.abs()`, `s.len()`) and
  `println!`, Rust adds `*` and `&` automatically.

**The big rule: aliasing XOR mutation**

- Data can be reached by many names (aliasing) OR changed (mutation),
  but not both at the same time.
- Why: `let num = &v[2]; v.push(4);`: push may move v's data to a
  new place and free the old one, so num would point to freed memory
  (use after free).

**Permissions R / W / O**

- R = read, W = write, O = own (move or drop).
- `let x` → R O, and `let mut x` → R W O.
- While `&x` is in use, x loses W and O (it keeps R).
- A reference lives only until its LAST USE, not until the end of main.
  After that, the owner gets its permissions back.
- My 2-step check: (1) find where the borrow starts and its last use,
  (2) is the data changed between them? If yes → error E0502.

**My predictions**

- 3/4 correct. I got Program 2 wrong: it compiles because `num` was
  last used BEFORE `push`. Moving one line can fix a borrow error!
- Note for next time: write the permissions after EACH line, because
  they change.

## Week 4 · Day 2: Mutable references + passgen step 1

**`&mut`: borrow to change**

- `fn shout(s: &mut String)` can change the String without owning it.
- I write `&mut` in both places: the parameter type and the call `shout(&mut text)`.
- The variable must be `let mut`.

**The rule: many readers OR one writer**

- Many `&` can exist at the same time (read only).
- Only ONE `&mut` at a time, and no `&` while it's in use.
- While `&mut x` is in use, x loses R, W and O. Everything comes back
  after the reference's last use.
- Errors: E0499 = two `&mut` at once. E0502 = `&` and `&mut` mixed.

**Lifetime = from birth to last use**

- A reference is born where it's created, and dies at its last use.
- With if/else, each branch can have a different last use, so I check
  each branch separately.

**Data must outlive its references**

- A reference can never live longer than the data it points to.
- I can't return `&` to a local variable (it's freed at the end of
  the function). Fix: return the value itself.
- "missing lifetime specifier" = Rust doesn't know where a returned
  reference comes from. Lifetimes (`'a`) come later.

**Passgen step 1** 🔐 (own repo: ~/Documents/Rust_tut/passgen)

- `add_chars(&mut Vec<char>, &str)`: pushes each char into the list
  (`&mut` because it changes the list).
- `build_charset(...)`: lowercase always, plus upper/digits/symbols
  if true → 75 chars.
- `generate(&Vec<char>, length)`: a loop runs `length` times, and each
  round picks a random index with `rng.gen_range(0..len)` and pushes
  one char (`&` because it only reads).
- `rand::thread_rng()` is a secure "dice machine". It needs `mut`
  because it changes its state with every roll.

**What I learned from my mistakes**

- I forgot the loop at first, so the password had only 1 character.
- `push` returns nothing, so there's no need for `let x = ...push()`.
- Use the weakest borrow that works: `&` to read, `&mut` to change.

## Week 4 · Day 3: WHY aliasing XOR mutation

**The forbidden program**
let first = &v[0]; v.push(4); println!("{}", first); → error E0502.
It looks harmless, but it can be a real use after free.

**What I saw (real addresses)**

- A Vec has pointer, len and cap on the stack.
- When len == cap and I push, the Vec gets a BIGGER space, copies the
  items, and FREES the old space.
- In my output, the address stayed the same at push 4 (it grew in place),
  but CHANGED at push 7. The data really moved!
- push only SOMETIMES moves the data. Rust can't know when, so it
  forbids the combination always.

**Why the rule exists**

- `first` is an alias (a second way to reach v's data).
- `push` is a mutation, and it may move the data.
- After the move, v knows the new address, but `first` still points to
  the OLD, freed place → use after free 💥 (a common security hole in C/C++).
- So Rust says: while an alias is alive, no mutation. That's aliasing XOR mutation.

**Two fixes for E0502**

1. Copy the value instead of borrowing: `let first = v[0];` (i32 is Copy)
2. Finish using the reference before the change (move the line up)

**Extra things I learned**

- Vec doubles its capacity (3 → 6 → 12) so it doesn't have to move often.
  `Vec::with_capacity(n)` reserves space in advance.
- `is_ascii_uppercase()` = only A–Z. `is_uppercase()` = capitals in any
  language. For passgen I use the ascii versions.

**Passgen step 2** 🔐

- `count_kinds(&String) -> (usize, usize, usize, usize)` counts lower,
  upper, digits and symbols. The 4 numbers always add up to the length.
- It takes `&` because it only reads, so `password` is still usable
  after the call.
- This is the first piece of the strength checker.

## Week 4 · Day 4: Fixing ownership errors

**My 3-question method**

1. What does the error say? (borrow starts → change/move → later use)
2. What would go wrong in memory? (double free, use after free, dangling reference)
3. What do I actually want? → then pick the fix.

**The standard fixes**

- Return the owned value (`String`, `Vec`) instead of `&` to a local.
- Use `&mut` when I need to change through a reference.
- Reorder: finish reading before changing.
- Borrow (`&`) instead of moving, when I only need to look.
- `.clone()` when I really need my own copy (it's fine for small data,
  but it shouldn't be a lazy fix).

**My 5 fixes**

1. `default_symbols() -> &Vec<char>`: failed because the local Vec dies
   at the end (dangling). Fixed by returning `Vec<char>`. (E0106)
2. `add_bang(&String)`: failed because `&` is read only. Fixed by
   `&mut String` + `&mut p`. (E0596)
3. password history: failed because `last` points into the Vec and push
   may move the data. Fixed by reordering (print first) OR `.clone()`. (E0502)
4. `let first: String = history[0];`: failed because taking it out would
   leave 2 owners (double free). Fixed by `&history[0]`. (E0507)
5. `show(password: String)`: failed because show took ownership and freed
   it. Fixed by `show(&String)`, since it only reads. (E0382)

**4 ways to return a string from a function**

- Return `String` (move it out): the most common.
- Return `&'static str` for fixed text, which lives forever.
- `Rc<String>` for shared ownership (Week 15).
- The caller passes `&mut String` and the function fills it.
- The real questions: how long should it live? Who should free it?

**Good to know**
Sometimes Rust rejects programs that are actually safe (like borrowing two
different array elements). Rust is careful, and there are special tools for that.

## Week 4 · Day 5: Choosing &, &mut, or ownership

**My rule of thumb**

- Start with `&` (read only).
- Use `&mut` if the function must change the caller's value.
- Take ownership (no `&`) only if the function must KEEP the value or use it up.
- A function named `into_...` usually takes ownership, so my value is gone after.

**My 3 functions**

- `sum(v: &Vec<i32>)`: WHY: it only reads the numbers.
- `double_all(v: &mut Vec<i32>)`: WHY: it changes the numbers in place.
- `into_largest(v: Vec<i32>) -> Option<i32>`: WHY: it takes the list
  (I'm done with it). It returns `Some(largest)`, or `None` for an empty list.

**My mistakes (and what I learned)**

- It's `v.len()` (a method with `()`), not `v.length`.
- A loop over positions needs a range: `for i in 0..v.len()`.
- `i` is the position and `v[i]` is the number. Add `v[i]`, not `i`.
- I need my own variable (`let mut total = 0;`). `sum` is the function's name.
- Lines that change something (`v[i] *= 2`, `x += 1`, `push`) don't need `let`.

**Rustlings move_semantics 1–5**

- #2 without clone: `fill_vec(&Vec<i32>)` borrows and builds a NEW Vec.
  Two different lists always need a copy somewhere. The only question is
  who makes it (the caller with `.clone()`, or the function).
- #4: reorder, so the first `&mut` is finished before the second one is created.
- #5: `mut data: String` in a parameter is OK: once the String is moved in,
  the function is the new owner and can decide to change it.
  `mut` belongs to the variable, not the value.

**Passgen step 3** 🔐

- `add_to_history(&mut Vec<String>, String)`: &mut because it adds to the
  history, and ownership because the history must keep the password.
- `print_history(&Vec<String>)`: only reads.
- The history lives only in memory (never save passwords as plain text!).
  Later, in the web UI, it can become a "recently generated" list.

## Week 4 · Day 6: Practical test + knowledge check

**The practical: removing zeros in 2 ways**

- `without_zeros(&Vec<i32>) -> Vec<i32>`: builds a NEW list and keeps the
  original. Use it when the caller still needs the old list ("before/after").
- `remove_zeros_in_place(&mut Vec<i32>)`: changes the list itself. Use it
  when the old version isn't needed anymore (it saves memory).

**The trap in remove_zeros_in_place**

- `v.remove(i)` makes the rest of the list slide LEFT, and the list gets shorter.
- `for i in 0..v.len()` calculates the length once at the start, so later
  the index goes out of bounds → panic "index out of bounds".
- `i += 1` after every round skips the item that slid into position i
  (with `0, 0` side by side, one zero stays).
- Fix: a `while i < v.len()` loop, and only `i += 1` when I KEEP the item.
- Analogy: people in a line. If #2 leaves, the next person steps into #2,
  so I check #2 again.
- It was hard, and I used the answer to understand it. That's OK.

**Knowledge check: 1 ✅, 3 🟡**

- Rules: (1) many & OR one &mut, never both at once.
  (2) A reference must not outlive its data.
- &v[0] + push → USE AFTER FREE (the reference reads freed memory),
  not double free.
- Dangling reference: points to freed memory. Rust checks at compile time
  that data outlives all its references.
- Fixes for E0502: reorder, copy/clone, restructure (e.g. keep an index).
  Adding `mut` does NOT fix it.

**Coming soon**

- `&[i32]` (a slice) is more flexible than `&Vec<i32>` → Week 5.
- `v.retain(|&x| x != 0)` does the same thing in one line (a closure,
  like a TS arrow function) → Week 14.

**Week 4 in one sentence**
Borrowing lets me lend a value (& to read, &mut to change) without giving
it away, and Rust makes sure no reference ever points to freed memory.
