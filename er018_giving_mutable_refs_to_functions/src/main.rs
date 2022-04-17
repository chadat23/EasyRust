// https://youtu.be/kJV1wIvAbyk?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn add_and_print_hungary(country_name: &mut String) {
    country_name.push_str("-Hungary");
    println!("Now it says: {country_name}");
}

fn adds_hungary(mut country: String) {
    country.push_str("-Hungary");
    println!("Now it says: {country}");
}

fn main1() {
    let mut country = String::from("Austria");
    add_and_print_hungary(&mut country);
}

fn main() {
    let country = String::from("Austria");
    adds_hungary(country);
}
