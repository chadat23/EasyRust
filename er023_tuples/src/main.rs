// https://youtu.be/U67Diy6SlTg?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main() {
    let random_tuple = (7, 8, "a str", [7, 8, 9], vec![8, 9], 7.8);
    println!("{:?}", random_tuple);
    // random_tuple.lkjlkjlkj(); // To see the type
    let random_tuple_2 = (7, 8);
    println!("{:?}", random_tuple.0 + random_tuple_2.1);

    let (a, b, _) = (8, 9, 10);
    println!("{a} {b}");

}