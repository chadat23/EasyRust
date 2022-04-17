// https://youtu.be/ph9CdWb9zXk?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main() {
    let mut book_hashmap = std::collections::HashMap::new(); // fully qualified path

    book_hashmap.insert(1, "L'allemagne Moderne");
    book_hashmap.insert(1, "Le Petite Prince");
    book_hashmap.insert(1, "If you give a mounse a cookie");
    book_hashmap.insert(1, "Eye of the world");

    println!("{:?}", book_hashmap.get(&1));    
}
