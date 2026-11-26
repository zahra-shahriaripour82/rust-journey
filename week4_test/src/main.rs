fn without_zeros(v: &Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for i in 0..v.len() {
        if v[i] != 0 {
            result.push(v[i])
        }
    }

    result
}

fn remove_zeros_in_place(v: &mut Vec<i32>) {
    let mut i = 0;
    while i < v.len() {
        if v[i] == 0 {
            v.remove(i); // the next item slides into position i, so DON'T move i
        } else {
            i += 1; // only move on when we kept the item
        }
    }
}

fn main() {
    let original = vec![0, 5, 0, 0, 3, 7, 0];
    let clean = without_zeros(&original);
    println!("original: {:?}", original);
    println!("clean:    {:?}", clean);

    let mut data = vec![0, 5, 0, 0, 3, 7, 0];
    remove_zeros_in_place(&mut data);
    println!("in place: {:?}", data);

    let mut all = vec![0, 0, 0];
    remove_zeros_in_place(&mut all);
    println!("all zeros: {:?}", all);
}
