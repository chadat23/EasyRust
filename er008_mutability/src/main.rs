// https://youtu.be/Nyyd6qn7dZY?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

// mutability: ability to change
// mutable vs. immutable

fn main() {
    // let my_number = 8;
    let mut my_number = 8;
    println!("My number is: {my_number}");
    // my_number = 10; // nope, not mutable
    my_number = 10; // nope, not mutable
    println!("Now my number is {my_number}");
    let my_number = "I am a string!";
    println!("Now my number is {my_number}")
}
