// https://youtu.be/5kxpSr2p_ao?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main() {
    let mut counter = 0;
    while counter < 5 {
        counter += 1;
        println!("The counter is now: {counter}");
    }

    for number in 0..3 {
        println!("The exclusive range number is: {number}");
    }

    for number in 0..=3 {
        println!("The inclusive range number is: {number}");
    }

    for _ in 0..3 {
        println!("I never used the output of the range!");
    }
}
