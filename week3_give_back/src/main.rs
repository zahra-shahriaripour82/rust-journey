fn count_vowels(text: String) -> (String, usize) {
    let mut count = 0;
    for c in text.chars() {
        if "aeiouAEIOU".contains(c) {
            count += 1;
        }
    }
    (text, count)
}

fn shout(mut text: String) -> String {
    text = text.to_uppercase();
    text.push('!');
    text
}

fn sum_and_len(numbers: Vec<i32>) -> (Vec<i32>, i32, usize) {
    let mut sum = 0;
    for i in 0..numbers.len() {
        sum += numbers[i];
    }
    let len = numbers.len();
    (numbers, sum, len)
}

fn main() {
    let name = String::from("Zahra learns Rust");
    let (name, vowels) = count_vowels(name);
    println!("'{}' has {} vowels", name, vowels);

    let loud = shout(name);
    println!("{}", loud);

    let nums = vec![4, 8, 15, 16, 23, 42];
    let (nums, sum, len) = sum_and_len(nums);
    println!("{:?}: sum = {}, count = {}", nums, sum, len);
}
