// https://youtu.be/zHt8QkqAJ-o?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main() {
    let my_number = 8;
    let my_reference = &my_number;
    // println!("{}", my_number == my_reference); // doesn't work since there's a different number of references
    // * means the value that & is pointing to
    println!("{}", my_number == *my_reference);
    
    let my_reference = &&my_number;
    println!("{}", my_number == **my_reference);
}
