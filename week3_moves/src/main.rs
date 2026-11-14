fn main() {
    let mut a = Box::new(1);
    let b = a;
    a = Box::new(2);
    println!("a = {}, b = {}", a, b);
}