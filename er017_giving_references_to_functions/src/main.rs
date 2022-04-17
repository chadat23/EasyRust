// https://youtu.be/mKWXt9YTavc?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn print_country(country_name: &String) {
    println!("{country_name}");
}

fn main() {
    let country = String::from("Austria");
    // print_country(country); // This doesn't work since the first print_country consumes country
    // print_country(country);
    print_country(&country);
    print_country(&country);
}
