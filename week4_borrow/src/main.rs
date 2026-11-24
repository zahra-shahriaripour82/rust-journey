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

fn longest_word(s: &String) -> usize {
    let mut longest = 0;
    for word in s.split_whitespace() {
        if word.len() > longest {
            longest = word.len();
        }
    }
    longest
}

fn shout(s: &mut String) {
    s.make_ascii_uppercase();
    s.push('!');
}

fn main() {
    let mut text = String::from("rust is wonderful");

    println!("longest word: {}", longest_word(&text));
    shout(&mut text);
    println!("after shout: {}", text);
    println!("longest word: {}", longest_word(&text));
}
