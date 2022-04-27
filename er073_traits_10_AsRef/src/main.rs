// https://youtu.be/iKFljZP6JD0?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

use std::convert::AsRef;
use std::fmt::Display;

fn print_it<T: AsRef<str> + Display>(input: T) {
    println!("{}", input);
}

fn main() {
    print_it("Please print this");
}
