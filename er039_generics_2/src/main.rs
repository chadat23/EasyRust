// https://youtu.be/ljcXsogCMSU?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk


use std::fmt::{Display, Formatter, Result};

// since we're going to display things, we need to tell rust that Display is going
// to be implimented
fn print_number<T: Display>(number: T) {
    println!("Here is your number: {number}");
}

struct Book {
    number: u8,
}

impl Display for Book {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Book: {}", self.number)
    }
}

fn main() {
    print_number(8);
    print_number(8.9);
    print_number("8");
    print_number(Book { number: 8 });
}
