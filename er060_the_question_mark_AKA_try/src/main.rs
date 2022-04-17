// https://youtu.be/XvXlrcESzjY?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

use std::num::ParseIntError;

// fn parse_str(input: &str) -> Result<i32, ParseIntError> {
//     // input.parse::<i32>()
//     let parsed_number = input.parse::<i32>()?;
//     Ok(parsed_number)
// }

// Notice how the "try"s are strung together
fn parse_str(input: &str) -> Result<u8, ParseIntError> {
    // input.parse::<i32>()
    let parsed_number = input.parse::<u16>()?
        .to_string()
        .parse::<u32>()?
        .to_string()
        .parse::<u8>()?;
    Ok(parsed_number)
}

fn main() {
    let str_vec = vec!["Seven", "8", "9.0", "nice", "6060"];
    for item in str_vec {
        let parsed = parse_str(item);
        println!("{:?}", parsed);
    }
}
