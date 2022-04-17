// https://youtu.be/R13sQ8SNoEQ?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

// (mine) Everyone can look at the art, but once one person can draw on it (change it), and that person is (by definition) the owner

fn return_str() -> &'static str {
    let country = String::from("Austria");
    let country_ref = &country;
    country_ref

    // cannot return value referencing local variable `country`
    // returns a value referencing data owned by the current function

    // People are externally being given permission to look at the art,
    // but then the owner dies (when the scope of the owner is left) 
    // their stuff is discarded so it's not
    // possible (doesn't make sense) for anyone to be granted permission to look at it
}

fn return_str_two() -> String {
    let country = String::from("Austria");
    country

    // Since the String owns the data, and ownership can be passed

    // The owner of the art gives it away as they die so the
    // art still exists and can still be viewed by others
}

fn main() {
    // ownership: who own it
    // rust wants exactly one owner per piece of data
    // (mine) Everyone can look at the art, but once one person can draw on it (change it), and that person is (by definition) the owner

    // if you're just reading data, you can have infinite references
    let country = String::from("Austria");
    let ref_one = &country;
    let ref_two = &country;
    println!("{country}, {ref_one}, {ref_two}");

    let country_name = return_str();
}
