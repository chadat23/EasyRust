// https://youtu.be/oCLy_E64JTs?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main() {
    let some_number = vec![9, 6, 9, 10, 11];
    println!("{}", some_number.iter().fold(0, |total_so_far, next_number| total_so_far + next_number));

    let a_string = "I don't have any dashes in me.";
    println!("{}", a_string.chars().fold("-".to_string(), |mut string_so_far, next_char| {
        string_so_far.push(next_char);
        string_so_far.push('-');
        string_so_far
    }));

    let some_number = vec![9, 6, 9, 10, 11];
    println!("The smallest number is: {}.", some_number.iter().fold(i32::MAX, |x, y| {
        if x < *y {
            x
        } else {
            *y
        }
    }))
}
