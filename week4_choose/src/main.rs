// WHY: & : just because read the value
fn sum(v: &Vec<i32>) -> i32 {
    // TODO: loop over the indexes and add each number to a total
    let mut total = 0;
    for i in 0..v.len() {
        total += v[i]
    }
    total
}

fn main() {
    let mut numbers = vec![2, 3, 4, 5];

    let total = sum(&numbers);
    println!("{}", total);
    double_all(&mut numbers);
    println!("{:?}", numbers);
    let largest = into_largest(numbers);
    println!("{:?}", largest);
}

// WHY: because we need to change the value of vec
fn double_all(v: &mut Vec<i32>) {
    // TODO: for i in 0..v.len() { v[i] = v[i] * 2; }   (or v[i] *= 2)

    for i in 0..v.len() {
        v[i] *= 2;
    }
}

// WHY: we just need to read a value
fn into_largest(v: Vec<i32>) -> Option<i32> {
    if v.is_empty() {
        return None;
    }
    let mut largest = v[0];
    for i in 1..v.len() {
        if v[i] > largest {
            largest = v[i];
        }
    }
    Some(largest)
}
