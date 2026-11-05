fn sum(numbers: [i32; 5]) -> i32 {
    let mut total = 0;
    let mut i = 0;
    while i < numbers.len() - 1 {
      total += numbers[i];
        i += 1;
    }
    total
}

fn main() {
    let numbers = [10, 20, 30, 40, 50];
     let result = sum(numbers);
    println!("sum = {result}"); // should be 150
}