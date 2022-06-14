// https://youtu.be/M43XCULOAbA?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main() {
    let new_vec = vec![8, 9, 10];
    let doubele_vec = new_vec
        .iter()
        .inspect(|first_item| println!("The item is: {}", first_item))
        .map(|x| x * 2)
        .inspect(|next_item| {
            dbg!(next_item);
        })
        .collect::<Vec<i32>>();

    let double_vec = new_vec
        .iter()
        .inspect(|first_item| {
            println!("The item is: {}", first_item);
            match **first_item % 2 {
                0 => println!("It is even."),
                _ => println!("It is odd.")
            }
            println!("In binary it is: {:b}", first_item);
        })
        .map(|x| x * 2)
        .collect::<Vec<i32>>();
}
