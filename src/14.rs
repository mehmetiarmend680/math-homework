use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1..101);
    println!("The random number is: {}", random_number);
}
