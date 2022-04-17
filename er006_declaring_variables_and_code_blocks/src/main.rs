// https://youtu.be/DTCSfBJJZb8?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main() {
    let my_number = 10; // let binding
    // declaring a variable
    // Tell Rust to make me a variable

    println!("{my_number}");

    {
        let my_second_number = 10;
    }
    // println!("{my_second_number}");  // Doesn't work since my_second_number no longer exists

    let my_second_number = {
        let second_number = 9;
        second_number
    };
    println!("{my_second_number}");

    let my_third_number = {
        let third_number = 8;
        third_number + 9
    };
    println!("My number is: {my_third_number}");

    let my_third_number = {
        let third_number = 8;
        third_number + 9;
    };
    println!("My number is: {:?}", my_third_number);
}
