// https://youtu.be/K3EbxHmVByM?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

/// GENERICS (in functions)
/// generic = can be one type, can be another type
/// constant = is one type
/// 
/// T, U, V - one capitcal letter hints as being a generic (is convention), being in brackets makes it so

fn return_number(number: i32) -> i32 {
    println!("Here is your number.");
    number
}

fn return_thing<T>(number: T) -> T {
    println!("Here is your number.");
    number
}

fn main() {
    let number = return_number(5);
    let my_thing = return_thing(String::from("I am a string"));
    println!("{my_thing}");
    let my_thing = return_thing(5);
    println!("{my_thing}");
}
