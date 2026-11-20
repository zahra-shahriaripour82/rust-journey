// fn main() {
//    let s1 = String::from("hello");
//    let s2 = s1;
//    println!("{}", s2);
//    println!("{}", s1);
// }

// my guess does not complied => correct :)

// fn main() {
//    let s1 = String::from("hello");
//    let s2 = s1.clone();
//    println!("{} {}", s1, s2);
// }

//My guess is complied => correct :)

// fn total(v: Vec<i32>) -> i32 {
//    v.iter().sum()
// }

// fn main() {
//    let numbers = vec![1, 2, 3];
//    println!("total = {}", total(numbers));
//    println!("{:?}", numbers);
// }

// my guess does not complied => correct :)

// fn main() {
//    {
//        let b = Box::new(1);
//        println!("inside: {}", b);
//    }
//    println!("outside: {}", b);
// }

//my guess does not complied => correct :)

// fn greet(name: String) {
//    println!("Hi, {}!", name);
// }

// fn main() {
//    let name = String::from("Zahra");
//    for _ in 0..2 {
//        greet(name);
//    }
// }

// my guess does not  complied . => correct :)

// fn main() {
//     let n = 7;
//     let b = Box::new(n); // L1
//     let c = b; // L2
//     println!("{} {}", n, c);
// }
// L1                              L2
// Stack            Heap           Stack            Heap
// main                            main
//  n │ 7                           n │ 7
//  b │ ● ─────────► 7              b │ ⦻
//                                  c │ ● ─────────► 7

// fn shout(mut text: String) -> String {
//     text.push('!'); // L2
//     text
// }

// fn main() {
//     let hi = String::from("hi"); // L1
//     let loud = shout(hi); // L3
//     println!("{}", loud);
// }

// L1                              L2
// Stack            Heap           Stack            Heap
// main                            main
//  hi │ ● ───────► h i             hi │ ⦻
//                                 shout
//                                  text │ ● ─────► h i !

// L3
// Stack            Heap
// main
//  hi   │ ⦻
//  loud │ ● ─────► h i !
// fn main() {
//     let v = vec![1, 2, 3];
//     let w = v.clone(); // L1
//     let x = v; // L2
//     println!("{:?} {:?}", w, x);
// }
// L1                              L2
// Stack            Heap           Stack            Heap
// main                            main
//  v │ ● ────────► 1 2 3           v │ ⦻
//  w │ ● ────────► 1 2 3           w │ ● ────────► 1 2 3
//                                  x │ ● ────────► 1 2 3  (the one v had)
