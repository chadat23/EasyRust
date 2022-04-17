// https://youtu.be/q1D2vpy3kEI?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main() {
    // Type inference: Rust will guess the type
    let my_number = 109; // i32 is guessed
    println!("{}", my_number);

    let my_number = 109u8;
    println!("{}", my_number);

    let my_number = 109u8;
    println!("{}", my_number);

    let my_number = 109_u8;

    println!("{}", my_number);

    let my_number = 1_________________________________0_______________9_____________u8;
    println!("{}", my_number);

    let my_number: u8 = 109;
    println!("{}", my_number);

    // let my_number = 8450487364950; // causes an issue since it doesn't fit in i32, which numbers are assumed to be
    // println!("{}", my_number);

    let my_number = 8450487364950_i64;
    println!("{}", my_number);

    let my_number = 8_450_487_364_950_i64; // works and is easier to read
    println!("{}", my_number);

    // "floats": f32, f64
    let my_float = 5.; // f64 by default, notice that nothing's needed after the .

    let my_float: f64 = 5.0; // This is an f64
    let my_other_float: f32 = 8.5; // This is an f32
    // let third_float = my_float + my_other_float; // ⚠️ causes an issue since it's adding f32 + f64
    let third_float = my_float + my_other_float as f64; // casting can fix it

    let my_float = 5.0; // removing the f64 lets rust figure it out for itself
    let my_other_float: f32 = 8.5; // This is an f32
    let third_float = my_float + my_other_float;
}
