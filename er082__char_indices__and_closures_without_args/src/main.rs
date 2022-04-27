// https://youtu.be/cgQNUCXTHEU?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main() {
    println!("{:?}", "Hello there".chars());
    println!("{:?}", "Hello there".chars().collect::<String>());

    let numbers_together = "6784521985132546874635124556876513";
    numbers_together
        .char_indices()
        .for_each(|(index, number)| {
            match (index % 3, number) {
                (0..=1, number) => print!("{number}"),
                _ => print!("{number}\t")
            }
        });

    println!();
    let my_vec = vec![8, 9, 10];
    print!("{:?}", my_vec.iter().for_each(|_| println!("We're not doing anything with these numbers")));
}
