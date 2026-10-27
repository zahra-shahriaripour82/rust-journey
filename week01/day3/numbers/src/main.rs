// use std::mem::size_of;

fn main() {
    //    let x: u8 = 250;

    // println!("checked:    {:?}", x.checked_add(10));
    // println!("checked ok: {:?}", x.checked_add(5));
    // println!("wrapping:   {}", x.wrapping_add(10));
    // println!("saturating: {}", x.saturating_add(10));

    // println!("u8 range:  {} to {}", u8::MIN, u8::MAX);
    // println!("i8 range:  {} to {}", i8::MIN, i8::MAX);
    // println!("i32 range: {} to {}", i32::MIN, i32::MAX);

    //       let x: u8 = std::hint::black_box(250);
    // let y: u8 = x + 10;
    // println!("y = {y}");


let a: i32 = 5;                 // lives on the STACK
    let b: i32 = 7;                 // also on the stack
    let boxed = Box::new(5);        // the 5 lives on the HEAP; `boxed` (a pointer) is on the stack
    let text: &str = "hello";       // the letters live in STATIC data

    println!("a      (stack):  {:p}", &a);
    println!("b      (stack):  {:p}", &b);
    println!("boxed  (heap):   {:p}", boxed);
    println!("text   (static): {:p}", text.as_ptr());


    // println!("i32:   {} bytes", size_of::<i32>());
    // println!("i64:   {} bytes", size_of::<i64>());
    // println!("u8:    {} bytes", size_of::<u8>());
    // println!("f64:   {} bytes", size_of::<f64>());
    // println!("bool:  {} bytes", size_of::<bool>());
    // println!("char:  {} bytes", size_of::<char>());
    // println!("usize: {} bytes", size_of::<usize>());


}
