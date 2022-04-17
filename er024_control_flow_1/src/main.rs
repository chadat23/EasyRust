// https://youtu.be/UAymDOpv_us?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main() {
    let my_number = 5;
    if my_number == 7 {
        println!("It's seven");
    }
    if my_number == 5 {
        println!("It's five");
    }
    if my_number == 9 {
        println!("It's nine");
    } else if my_number == 6 {        
        println!("It's nine");
    } else {
        println!("It's something else!");
    }

    let my_number = 5;
    if my_number % 2 == 1 && my_number > 0 { // % 2 means the number that remains after diving by two
        println!("It's a positive odd number");
    } else if my_number == 6 {
        println!("It's six")
    } else {
        println!("It's a different number")
    }

    let my_number: u8 = 5;
    match my_number {
        8 => println!("It's an eight"),
        10 => println!("It's a ten"),
        _ => println!("It's something else"),
    }

    let my_number: u8 = 5;
    let something = match my_number {
        8 => 10,
        10 => 200,
        _ => 55,
    };
    println!("{something}");
    
    let sky = "cloudy";
    let temperature = "warm";
    match (sky, temperature) {
        ("cloudy", "cold") => println!("It's dark and unpleasant today"),
        ("clear", "warm") => println!("It's a nice day"),
        ("cloudy", "warm") => println!("It's dark but not bad"),
        _ => println!("Not sure what the weather is."),
    };

    let children = 5;
    let married = true;
    match (children, married) {
        (children, married) if married == false => println!("Not married with {} children", children),
        (children, married) if children == 0 && married == true => println!("Married but no children"),
        _ => println!("Married? {}. Number of children: {}.", married, children),
    }
}