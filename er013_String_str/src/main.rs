// https://youtu.be/pSyaGzGg26o?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main() {
    // String: "string", more like a structure that own the data. Flexable but slow. Data's on the heap
    // &str: "str" or "string slice", really fast but not owned by you

    let my_variable = "hello, World!"; // &str

    println!("A String is always {:?} bytes. It is Sized.", std::mem::size_of::<String>()); // std::mem::size_of::<Type>() gives you the size in bytes of a type
    println!("And an i8 is always {:?} bytes. It is Sized.", std::mem::size_of::<i8>());
    println!("And an f64 is always {:?} bytes. It is Sized.", std::mem::size_of::<f64>());
    println!("But a &str? It can be anything. '서태지' is {:?} bytes. It is not Sized.", std::mem::size_of_val("서태지")); // std::mem::size_of_val() gives you the size in bytes of a variable
    println!("And 'Adrian Fahrenheit Țepeș!!!!!' is {:?} bytes. It is not Sized.", std::mem::size_of_val("Adrian Fahrenheit Țepeș!!!!!"));    

    let my_name = "Billybrobby";
    let my_country = "USA";
    let my_home = "Korea";
    let together = format!(
        "I am {} and I come from {} but I live in {}.",
        my_name, my_country, my_home
    );
    println!("{together}");
}
