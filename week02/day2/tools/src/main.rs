fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn total(numbers: [i32; 5]) -> i32 {
    let mut sum = 0;
    for n in numbers {
        sum += n;
        
    }
    sum
}

fn main() {
    let numbers = [3, 7, 1, 9, 4];
    let ok = is_even(4);
    if ok {
        println!("4 is even");
    }
    println!("total = {}", total(numbers));




    let t = total(numbers);
    let doubled = dbg!(t * 2);
    let pair = (t, doubled);
    println!("{:?}", pair);
    println!("{:#?}", pair);


}
