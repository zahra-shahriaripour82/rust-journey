fn main() {
    let s = String::from("hi");
    let r = &s;
    println!("{}", r);
    println!("{}", s);
}

//s : R O
// r : R
//Complied

fn main() {
    let mut v = vec![1, 2, 3];
    let num = &v[2];
    println!("third = {}", *num);
    v.push(4);
    println!("{:?}", v);
}

// v : R O W
// num : R
// Not compiled

fn main() {
    let mut v = vec![1, 2, 3];
    let num = &v[2];
    v.push(4);
    println!("third = {}", *num);
}

//v : R O W
// num : R
// Not compiled

fn main() {
    let mut x = 5;
    let y = &x;
    println!("y = {}", y);
    x += 1;
    println!("x = {}", x);
}

// x = R O W
// y : R
// compiled
