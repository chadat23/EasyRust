// https://youtu.be/cxTP5gPuiu4?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

struct Book {
    number: u32
}

impl Book {
    fn get_number(&self) -> u32 {
        self.number
    }

    fn change_number(&mut self, new_number: u32) {
        self.number = new_number;
    }

    // Associated functions / static function
    fn new(number: u32) -> Self {
        Self { number }
    }

    fn helper_function(number: u32) -> u32 {
        number * 2
    }
}

fn main() {
    let mut my_book = Book {
        number: 50
    };
    println!("My book's number is {}.", my_book.get_number());
    my_book.change_number(60);
    println!("Now my book's number is {}.", my_book.get_number());

    let my_book2 = Book::new(70);
    println!("I don't know how this is currently helping but... {}", Book::helper_function(4));
}
