// https://youtu.be/eKENnjPeCwU?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

use std::collections::HashMap;

fn main() {
    let some_numbers = vec![0, 1, 2, 3, 4, 5];
    let some_words = vec!["zero", "one", "two", "three", "four"];
    let number_word_hashmap = some_numbers
        .into_iter()
        .zip(some_words.into_iter())
        // .collect::<HashMap<i32, &str>>();
        .collect::<HashMap<_, _>>();

    println!("For key {} we get {}", 2, number_word_hashmap.get(&2).unwrap());

    for (key, value) in number_word_hashmap {
        println!("{key}, {value}");
    }
}
