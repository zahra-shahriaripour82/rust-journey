fn main() {
    let my_numbers = [4, 8, 15, 16, 23, 42, 1, 7, 9, 10];
    let (min, max, avg, sum) = stats(my_numbers);
    println!("min = {min}, max = {max}, avg = {avg} sum = {sum}");
}

fn stats(numbers: [i32; 10]) -> (i32, i32, f64, i64) {
    let mut min = numbers[0]; // start with the first number
    let mut max = numbers[0];
    let mut sum = 0;

    for n in numbers {
        // look at each number, one by one
        if n < min {
            min = n; // found a smaller one → remember it
        }
        if n > max {
            max = n; // found a bigger one → remember it
        }
        sum += n as i64; // add to the total
    }

    let avg = sum as f64 / numbers.len() as f64; // decimal division

    (min, max, avg, sum) // no ; → returned as a tuple
}
