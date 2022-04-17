// https://youtu.be/LOHVUYTc5Us?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

enum Number {
    U32(u32),
    I32(i32),
}

fn get_number(input: i32) -> Number {
    match input.is_positive() {
        true => Number::U32(input as u32),
        false => Number::I32(input)
    }
}

fn main() {
    use Number::*;

    let my_vec = vec![get_number(-800), get_number(8)];

    for item in my_vec {
        match item {
            U32(number) => println!("It is a u32 with this value: {number}"),
            I32(number) => println!("It is a i32 with this value: {number}"),
        }
    }
}
