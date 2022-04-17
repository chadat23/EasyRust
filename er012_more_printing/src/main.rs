// https://youtu.be/ycjZtvqyRHc?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main() {
    let father_name = "Vlad";
    let son_name = "Adrian Fahrenheit";
    let family_name = "Țepeș";
    println!("This is {1} {2}, sone of {0} {2}.", father_name, son_name, family_name);

    println!(
        "{city1} is in {country} and {city2} is also in {country},
but {city3} is not in {country}.",
        city1 = "Seoul",
        city2 = "Busan",
        city3 = "Tokyo",
        country = "Korea"
    );

    // <variable>:<padding_charicter><alignment><minimum>.<maximum>
    let letter = "a";
    println!("{letter:ㅎ^11}");
    println!("{letter:ㅎ>11}");
    println!("{letter:ㅎ<11}");

    let title = "TODAY'S NEWS";
    println!("{:-^30}", title); // no variable name, pad with -, put in centre, 30 characters long
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar); // no variable name, pad with space, 15 characters each, one to the left, one to the right
    let a = "SEOUL";
    let b = "TOKYO";
    println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b); // variable names city1 and city2, pad with -, one to the left, one to the right
}
