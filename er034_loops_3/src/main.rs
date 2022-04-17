// https://youtu.be/gX53Qr-hQ28?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main1() {
    let mut counter = 5;
    let my_number = loop {
        counter += 1;
        if counter % 53 == 3 {
            break counter;  // break while returning the associated variable
        }
    };
    println!("{my_number}");
}

fn match_colors(rgb: (u8, u8, u8)) {
    println!("Compareing a color with {} red, {} green, and {} blue", rgb.0, rgb.1, rgb.2);
    let new_vec = vec![(rgb.0, "red"), (rgb.1, "green"), (rgb.2, "blue")];

    let mut all_have_at_least_10 = true;

    for item in new_vec {
        if item.0 < 10 {
            all_have_at_least_10 = false;
            println!("Not much {}", item.1);
        }
    }
    if all_have_at_least_10 {
        println!("Every color has at least 10");
    }
}

fn main() {
    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let thrid = (200, 50, 0);

    match_colors(first);
    match_colors(second);
    match_colors(thrid);
}
