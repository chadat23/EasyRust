// https://youtu.be/Q6LZ4KzwZfw?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn prints_three_things(vector: Vec<i32>) {
    if vector.len() != 3 {
        panic!("my_vec must ALWAYS have EXACTLY three items!")
    }
    println!("{}, {}, {}", vector[0], vector[1], vector[2]);
}

fn main() {
    // panic!();
    // panic!("Something has gone terribly wrong!");

    // let my_vec = vec![8, 9, 10];
    let my_vec = vec![8, 9, 10, 10, 10, 55]; // oops, too many things
    prints_three_things(my_vec);

    
}
