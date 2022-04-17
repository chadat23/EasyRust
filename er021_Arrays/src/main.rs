// https://youtu.be/Iuq3Cort3Eg?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main() {
    let my_array = ["One", "Two"];
    // my_array.blkajsdlrfkj();  // would show the type in an error message if run
    let my_array = [0_u8; 40];
    println!("{:?}", my_array);
    println!("{}", my_array[0]);
    
    let array_of_ten = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let three_to_five = &array_of_ten[2..5];
    let start_at_two = &array_of_ten[1..];
    let end_at_five = &array_of_ten[..5];
    let everything = &array_of_ten[..];
    println!("Three to five: {:?},\nstart at two: {:?},\nend at five: {:?},\neverything: {:?}", three_to_five, start_at_two, end_at_five, everything);
}