// fn main() {
//     let s = String::from("hi");
//     let r = &s;
//     println!("{}", r);
//     println!("{}", s);
// }

// //s : R O
// // r : R
// //Complied

// fn main() {
//     let mut v = vec![1, 2, 3];
//     let num = &v[2];
//     println!("third = {}", *num);
//     v.push(4);
//     println!("{:?}", v);
// }

// // v : R O W
// // num : R
// // Not compiled

// fn main() {
//     let mut v = vec![1, 2, 3];
//     let num = &v[2];
//     v.push(4);
//     println!("third = {}", *num);
// }

// //v : R O W
// // num : R
// // Not compiled

// fn main() {
//     let mut x = 5;
//     let y = &x;
//     println!("y = {}", y);
//     x += 1;
//     println!("x = {}", x);
// }

// // x = R O W
// // y : R
// compiled

// fn main() {
//     let mut text = String::from("hi");
//     let r = &text;
//     let m = &mut text;
//     m.push('!');
//     println!("{}", r);
// }

// not compiled

// fn main() {
//     let mut text = String::from("hi");
//     let m1 = &mut text;
//     let m2 = &mut text;
//     m1.push('!');
//     m2.push('?');
// }

// not compiled

// fn main() {
//     let mut text = String::from("hi");
//     let m = &mut text;
//     m.push('!');
//     println!("{}", text);
// }

// compiled

// fn main() {
//     let mut text = String::from("hi");
//     let m = &mut text;
//     println!("{}", text);
//     m.push('!');
// }
// not compiled

// fn longest_word(s: &String) -> usize {
//     let mut longest = 0;
//     for word in s.split_whitespace() {
//         if word.len() > longest {
//             longest = word.len();
//         }
//     }
//     longest
// }

// fn shout(s: &mut String) {
//     s.make_ascii_uppercase();
//     s.push('!');
// }

// fn main() {
//     let mut text = String::from("rust is wonderful");

//     println!("longest word: {}", longest_word(&text));
//     shout(&mut text);
//     println!("after shout: {}", text);
//     println!("longest word: {}", longest_word(&text));
// }

// fn main() {
//     let mut v = vec![1, 2, 3];
//     let neighbor = vec![0; 3]; // another list, placed right after v on the heap
//     println!(
//         "start:   len={} cap={} data at {:p}",
//         v.len(),
//         v.capacity(),
//         v.as_ptr()
//     );

//     for n in 4..=8 {
//         v.push(n);
//         println!(
//             "push {}: len={} cap={} data at {:p}",
//             n,
//             v.len(),
//             v.capacity(),
//             v.as_ptr()
//         );
//     }
//     println!("{:?}", neighbor);
// }

// fn default_symbols() -> Vec<char> {
//     let symbols = vec!['!', '@', '#'];
//     symbols
// }

// fn main() {
//     let s = default_symbols();
//     println!("{:?}", s);
// }

// fn add_bang(password: &mut String) {
//     password.push('!');
// }

// fn main() {
//     let mut p = String::from("abc123");
//     add_bang(&mut p);
//     println!("{}", p);
// }

// fn main() {
//     let mut history = vec![String::from("Qx7!"), String::from("m9@Z")];
//     let last = &history[history.len() - 1];
//     println!("last before push: {}", last); // ← moved up: last use of `last`
//     history.push(String::from("Tt5#")); // ✅ the borrow is already over
// }

// fn main() {
//     let history = vec![String::from("Qx7!"), String::from("m9@Z")];
//     let first: &String = &history[0];
//     println!("first: {}", first);
//     println!("all: {:?}", history);
// }

fn show(password: &String) {
    println!("Your password: {}", password);
}

fn main() {
    let password = String::from("Qx7!m9@Z");
    show(&password);
    println!("Length: {}", password.len());
}
