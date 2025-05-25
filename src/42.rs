use std::cmp;

fn main() {
    let arr = vec![1, 3, 5, 7, 9];
    let mut max_value = arr[0];

    for &value in arr.iter().rev() {
        if value > *max_value {
            max_value = value;
        }
    }

    println!("The maximum value is: {}", max_value);
}
