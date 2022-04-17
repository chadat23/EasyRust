// https://youtu.be/InULHyRGw7g?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main1() {
    let my_number = 8; // This is an i32
    println!("My number is: {my_number}");
    let my_number = 9.2; // This is an f64 with the same name. But it's not the first number but it's completely different!
    println!("Now my number is {my_number}");
}

fn main2() {
    let my_number = 8;
    println!("{my_number}");
    {
        let my_number = 9.2;
        println!("{my_number}");
    }
    println!("{my_number}");
}

fn times_two(number: i32) -> i32 {
    number * 2
}

fn main() {
    let final_number = {
        let y = 10;
        let x = 9;
        let x = times_two(x);
        let x = x + y;
        x
    };
    println!("The number is now: {final_number}")
}
