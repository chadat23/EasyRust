// https://www.youtube.com/watch?v=OxTPU5UGMhs&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk&index=4

fn main() {
    // maybe consider usize and isize if you don't know the destination system

    // " ", double quotes for strings, ' ', single quotes for chars

    let first_letter = 'A';
    let space = ' '; // A space inside ' ' is also a char
    let other_language_char = 'Ꮔ'; // Thanks to Unicode, other languages like Cherokee display just fine too
    let cat_face = '😺'; // Emojis are chars too

    let my_number = 100; // We didn't write a type of integer,
    // so Rust chooses i32. Rust always
    // chooses i32 for integers if you don't
    // tell it to use a different type

    println!("{}", my_number); // ⚠️
    // println!("{}", my_number as char); // ⚠️ Doesn't work since i32 has too large of a bounds to be able to garentee that there won't be truncation
    println!("{}", my_number as u8 as char); // ⚠️ 'd' is the 100th char
    println!("{}", 'd' as u8); // ⚠️ 'd' is the 100th char
    
    let my_number: u8 = 100;
    println!("{}", my_number as u8 as char); // This works since my_number started as a u8

    println!("Size of a char: {}", std::mem::size_of::<char>()); // 4 bytes
    println!("Size of string containing 'a': {}", "a".len()); // .len() gives the size of the string in bytes
    println!("Size of string containing 'ß': {}", "ß".len());
    println!("Size of string containing '国': {}", "国".len());
    println!("Size of string containing '𓅱': {}", "𓅱".len());

    let slice = "Hello!";
    println!("Slice is {} bytes.", slice.len());
    let slice2 = "안녕!"; // Korean for "hi"
    println!("Slice2 is {} bytes.", slice2.len());

    let slice = "Hello!";
    println!("Slice is {} bytes and also {} characters.", slice.len(), slice.chars().count());
    let slice2 = "안녕!";
    println!("Slice2 is {} bytes but only {} characters.", slice2.len(), slice2.chars().count());

}
