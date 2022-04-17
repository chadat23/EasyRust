// https://youtu.be/yYlPHRl2geQ?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn number() -> i32 { // skinny arrow = what the fucntion is returning
    8
}

fn multiply(number_one: i32, number_two: i32) {
    let result = number_one * number_two;
    println!("{number_one} times {number_two} is {}", result)
}

fn main() {
    println!("Hello, world number {}!", 8);
    println!("Hello, world number {}!", number());

    multiply(8, 16);
    let some_number = 10;
    let other_number = 2;
    multiply(some_number, other_number);
}
