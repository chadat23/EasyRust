// https://youtu.be/hHTzhNci4VE?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

use std::collections::HashMap;

fn main() {
    let book_collection = vec!["Book one", 
                                         "Book two",                                     
                                         "Book two", 
                                         "Book 3"];

    let mut book_hashmap = HashMap::new();

    for book in book_collection {
        let return_value = book_hashmap.entry(book).or_insert(0);
        *return_value += 1;
    }

    for (book, number) in book_hashmap {
        println!("{}: {}", book, number);
    }
}
