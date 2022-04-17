// https://youtu.be/W23uQghBOFk?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://youtu.be/GSVhrjLCuNA?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

struct FileDirectory; // Unit struct

#[derive(Debug)]
struct Color(u8, u8, u8); // tuple struct

#[derive(Debug)]
struct SizeAndColor { // named struct    
    size: u32,
    color: Color,
}

#[derive(Debug)]
struct Country {
    population: u32,
    capital: String,
    leader_name: String,
}

fn main() {
    let my_directory = FileDirectory;
    let my_colors = Color(50, 60, 0);
    let size_and_color = SizeAndColor {
        size: 150,
        color: my_colors,
    };
    println!("SizeAndColor: {:?}", size_and_color);

    let population = 500_000;
    let capital = String::from("Elista");
    let leader_name = String::from("Batu Khasikov");
    let kalmykia = Country {
        population,
        capital,
        leader_name,
    };
    println!("Kalmykia: {:?}", kalmykia);

}
