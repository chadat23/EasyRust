// https://youtu.be/BdU9JphfBaI?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main() {
    // Note: this is print!, not println!
    print!("\t Start with a tab\nand move to the next line.");
    println!("Inside quotes
you can write over
many lines
and it will print just fine.");
    println!("If you forget to write
        on the left side, the paces
    will be added when you print.");
    println!("\\t Start with a tab\\nand move to the next line.");
    println!("He said, \"You can find the file at c:\\files\\my_documents\\file.txt.\" Then I found the file.");
    // Raw string
    println!(r#"He said, "You can find the file at c:\files\my_documents\file.txt." Then I found the file."#);
    // Add more # to enable the use of them in the text
    println!(r###"Look, it's ""##" in the line!"###);

    // putting a "b" at the beginning of a string converts it to bytes
    println!("{:?}", b"This will look like numbers");

    // :X converts to hexadecimal
    println!("{}", '행' as u32);
    println!("{:X}", '행' as u32); // Cast char as u32 to get the hexadecimal value
    println!("{}", 'H' as u32);
    println!("{:X}", 'H' as u32);
    println!("{}", '居' as u32);
    println!("{:X}", '居' as u32);
    println!("{}", 'い' as u32);
    println!("{:X}", 'い' as u32);
    println!("\u{D589}, \u{48}, \u{5C45}, \u{3044}"); // Try printing them with unicode escape \u

    // :p prints the memory address where pointers are pointing to
    let number = 9;
    let number_ref = &number;
    println!("Here's the memory address that the pointer's pointing to: {number_ref:p}");

    // :b prints binery
    // :x hex
    // :o octal
    // :p prints the memory address where pointers are pointing to
    let number = 555;
    println!("Binary: {number:b}, Hex: {number:x}, octal: {number:o}");
}
