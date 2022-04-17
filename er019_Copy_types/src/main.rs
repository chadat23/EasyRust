// https://youtu.be/g0QM2wM1X5o?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn print_country(country_name: String) {
    println!("{country_name}")
}

fn main() {
    let country = String::from("Kiribati");
    print_country(country.clone());
    print_country(country);
}