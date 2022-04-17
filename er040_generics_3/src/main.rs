// https://youtu.be/IYXby69VMrU?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

use std::fmt::{Debug, Display};
use std::cmp::PartialOrd;

#[derive(Debug)]
struct Animal {
    name: String,
    age: u8
}

// Debug is a trait ~ power
fn print_item<MyGenericType: Debug>(item: MyGenericType) {
    println!("Here is your item: {:?}", item);
}

fn main1() {
    let charlie = Animal {
        name: String::from("Charlie"),
        age: 1,
    };
    let number = 55;
    print_item(charlie);
    print_item(number)
}

fn compare_and_display<T: Display, U: Display + PartialOrd>(statement: T, numb_1: U, numb_2: U) {
    println!("{}! Is {} greater than {}? {}",
        statement, numb_1, numb_2, numb_1 > numb_2
    );
}

fn main() {
    compare_and_display("Listen", 1, 8);
}
