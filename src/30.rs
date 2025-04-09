use std::cmp;

fn main() {
    // Example usage of the `rand` module to generate random numbers
    let mut rng = rand::thread_rng();
    let num: u32 = 10;
    
    if num > 5 {
        println!("The number is greater than or equal to 5.");
    } else {
        println!("The number is less than 5.");
    }
}
