// https://youtu.be/bLsGpFTrubo?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main() {
    let x = 9;
    let y = 10;
    let my_closure = |input: i32| println!("x + y + input= {}", x + y + input);
    my_closure(10);

    let first_number = 100;
    let my_closure = |input: i32| {
        let number = 7;
        let other_number = 10;
        println!("All together, the numbers are: {}", first_number + number + other_number + input);
    };
    my_closure(18);
}
