// https://youtu.be/eqysTfiiQZs?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn match_colors(rgb: (u8, u8, u8)) {
    // match rgb {
    //     (r, g, b) if r < 10 => println!("Not much red"),
    //     (r, g, b) if g < 10 => println!("Not much green"),
    //     (r, g, b) if b < 10 => println!("Not much blue"),
    //     _ => println!("Each color has at least ten"),
    // }
    match rgb {
        (r, _, _) if r < 10 => println!("Not much red"),
        (_, g, _) if g < 10 => println!("Not much green"),
        (_, _, b) if b < 10 => println!("Not much blue"),
        _ => println!("Each color has at least ten"),
    }
}

fn lucky_numbers(input: i32) {
    match input {
        number @ 4 => println!("{} is 4 * {}", number * 4, number),
        number @ 10 => println!("{} is 10 * {}", number * 10, number),
        number => println!("Nothing to see here, {number}")
    }
}

fn main() {
    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);
    match_colors(first);
    match_colors(second);
    match_colors(third);

    lucky_numbers(55);
    lucky_numbers(10);
    lucky_numbers(4);
}