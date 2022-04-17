// https://youtu.be/G48z6Rv76vc?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

// Mutable reference: giving someone else permission to draw on your art

// Rule 1: if you only have immutable references then you can have infinite refs
// Rule 2: if you have a mutable ref, then you can only have 1 and you can't have any immutable

// Only one person can draw on a painting at a time (immagin infinite people trying to draw at onece)
// And you don't want people looking at a mid process edit so no immutable refs if a mutable exists

// https://dhghomon.github.io/easy_rust/Chapter_17.html Read Powerpoint example

fn main() {
    let mut my_number = 8;  // mut: if the art can't be changed, then how can permission to change it be given?
    let num_ref = &mut my_number;
    *num_ref += 10;
    println!("{num_ref}");

    // Here the compiler's smart enough to see that the &mut stops being used
    // before the & is created so that works
    let mut number = 10;
    let number_change = &mut number; // create a mutable reference
    *number_change += 10; // use mutable reference to add 10
    let number_ref = &number; // create an immutable reference
    // *number_change += 10; // this breaks since is undoes the above assertion about ordering
    println!("{}", number_ref); // print the immutable reference

    let country = String::from("Austria");
    let country_ref = &country;
    let country = 8;
    println!("{}, {}", country_ref, country);
}
