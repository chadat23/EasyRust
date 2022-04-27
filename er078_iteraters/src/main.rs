// https://youtu.be/IX8KcuZBjtk?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

#[derive(Debug, Clone)]
struct Library {
    library_type: LibraryType,
    books: Vec<String>,
}

#[derive(Debug, Clone)]
enum LibraryType {
    City,
    Country,
}

impl Library {
    fn add_book(&mut self, book: &str) {
        self. books.push(book.to_string());
    }

    fn new() -> Self{
        Self {
            library_type: LibraryType::City,
            books: Vec::new(),
        }
    }
}

impl Iterator for Library {
    type Item = String;  // This is the Associated Type
    fn next(&mut self) -> Option<Self::Item> {  // or (same as) Option<String> since that what it is in this example
        match self.books.pop() {
            Some(book) => Some(book + "is found!"),
            None => None,
        }
    }
}

fn main() {
    let mut my_library = Library::new();
    my_library.add_book("If you give a mouse a cookie");
    my_library.add_book("The giving tree");
    my_library.add_book("12 little indians");
    my_library.add_book("Snow Piercer");

    for book in my_library {
        // Note that it's returned in reverse order since we're using ".pop()"
        println!("{book}");
    }
}
