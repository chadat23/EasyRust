// https://youtu.be/9jHr72qeAh0?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

use std::fmt::{Debug, Display};
use std::cmp::PartialOrd;

#[derive(Debug)]
struct Animal {
    name: String,
    age: u8
}

// fn compare_and_display<T: Display, U: Display + PartialOrd, MyGenericType: Debug>(statement: T, numb_1: U, numb_2: U, animal: MyGenericType) 
fn compare_and_display<T, U, V>(statement: T, numb_1: U, numb_2: U, animal: V) 
where T: Display,
      U: Display + PartialOrd,
      V: Debug
{
    println!("{}! Is {} greater than {}? {}",
        statement, numb_1, numb_2, numb_1 > numb_2
    );
    println!("By the way, I have an animal: {:?}", animal);
}

fn main() {
    let charlie = Animal {
        name: String::from("Charlie"),
        age: 7,
    };
    compare_and_display("Listen", 1, 8, charlie);
    compare_and_display(8, 9, 10, 11);
}
