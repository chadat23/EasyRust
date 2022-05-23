// https://youtu.be/w91X8GUBx-k?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main() {
    let even_odd = vec!["even", "odd"];
    let even_odd_vec = (0..6)
        .zip(even_odd.into_iter().cycle())
        .collect::<Vec<(i32, &str)>>();
    println!("{:?}", even_odd_vec);

    let ten_chars = ('a'..).take(10).collect::<Vec<char>>();
    let skip_then_ten_chars = ('a'..).skip(1300).take(10).collect::<Vec<char>>();
    println!("{:?}", ten_chars);
    println!("{:?}", skip_then_ten_chars);
}
