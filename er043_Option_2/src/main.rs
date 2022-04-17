// https://youtu.be/GKnbGUX7OB4?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4])
    }
}

fn handle_option(my_options: Vec<Option<i32>>) {
    for item in my_options {
        match item {
            Some(number) => println!("Got a {number}"),
            None => println!("The vec is too short"),
        }
    }
}

fn main() {
    let new_vec = vec![1, 2];
    let bigger_vec = vec![1, 2, 3, 4, 5];
    // println!("{:?}, {:?}", 
    //     take_fifth(new_vec).expect("This absolutely needs 5 items to work."), 
    //     take_fifth(bigger_vec).unwrap()
    // );
    let mut option_vec = Vec::new();
    option_vec.push(take_fifth(new_vec));
    option_vec.push(take_fifth(bigger_vec));
    handle_option(option_vec);
}
