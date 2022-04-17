// https://youtu.be/ASZnjtCUNhs?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

use std::collections::VecDeque;

fn main() {
    // let mut my_vec = vec![0; 600_000];
    // for _ in 0..600_000 {
    //     my_vec.remove(0);
    // }
    // println!("Done!")

    
    let mut my_vec = VecDeque::from(vec![0; 600_000]);
    for _ in 0..600_000 {
        my_vec.pop_front();
    }
    println!("Done!")
}
