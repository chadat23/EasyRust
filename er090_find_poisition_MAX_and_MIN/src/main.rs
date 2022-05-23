// https://youtu.be/4prgsm70Hrc?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

// .find()  - I will try to find it for you
// .position()  - I will try to find ift for you, and tell you where it is

fn main() {
    let num_vec = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    println!("{:?}", num_vec.iter().find(|&number| number % 3 == 0));

    println!("{:?}", num_vec.iter().find(|&number| number * 2 == 30));

    println!("{:?}", num_vec.iter().position(|&number| number % 3 == 0));

    println!("{:?}", num_vec.iter().position(|&number| number * 2 == 30));
}
