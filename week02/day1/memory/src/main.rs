fn main() {
   a();
}


fn c() {
    let z : i32 = 3;
    println!("      c:z at {:p}",&z) ;
}

fn b() {
    let y : i32 = 2;
    println!("      b:y at {:p}",&y) ;

    c();

}


fn a() {
    let x: i32 = 1;
    println!("a: x at {:p}", &x);
    b();
}

