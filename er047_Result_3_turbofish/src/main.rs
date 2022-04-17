// https://youtu.be/L3xYMTmjnKM?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

// Turbofish::<>()
use core::num::ParseIntError;

fn return_number(input: &str) -> Result<i32, ParseIntError> {
    input.parse::<i32>()
}

fn main() {
     let my_number = return_number("5");

     let my_vec = vec!["5", "five", "9.7", "6tyfive", "789"];
     for number in my_vec {
         match return_number(number) {
             Ok(number) => println!("Got: {number}"),
             Err(e) => println!("Didn't work: {e}"),
         }
     }
}
