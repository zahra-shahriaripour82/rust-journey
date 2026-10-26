
use std::time::Instant;

fn main() {
    // println!("Hello, world!");
    let start =Instant::now();

    let mut sum: u64 = 0 ;

    for i in 1..=1_000_000_000u64 {
sum += std::hint::black_box(i)
    }



    println!("sum = {sum}");
    println!("took {:?}",start.elapsed());

}







