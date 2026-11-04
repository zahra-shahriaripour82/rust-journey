fn main() {
    let my_numbers = [4, 8, 15, 16, 23, 42, 1, 7, 9, 10];
    let (min, max, avg, sum) = stats(my_numbers);
    println!("min = {min}, max = {max}, avg = {avg} sum = {sum}");
}

fn stats(numbers: [i32; 10]) -> (i32, i32, f64, i64) {
    let mut min = numbers[0]; 
    let mut max = numbers[0];
    let mut sum = 0;

    for n in numbers {

        if n < min {
            min = n; 
        }
        if n > max {
            max = n;
        }
        sum += n as i64; 
    }

    let avg = sum as f64 / numbers.len() as f64; 

    (min, max, avg, sum) 
}
