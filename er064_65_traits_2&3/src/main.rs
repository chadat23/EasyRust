// https://youtu.be/kDpqRNHIz4E?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://youtu.be/o9jZXLX9_Vw?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

struct Cat {
    name: String,
    age: u8,
}

impl std::fmt::Display for Cat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cat_type = match self.age {
            0..=2 => "a kitten",
            3..=10 => "an adult cat",
            _ => "an old cat"
        };
        write!(f, "{} is a cat who is {} so they're {}", self.name, self.age, cat_type)
    }
}

fn main() {
    let mr_mantle = Cat {
        name: "Reggie Mantle".to_string(),
        age: 4,
    };

    println!("{mr_mantle}")
    
    println!("This many characters: {}", mr_mantle.to_string().chars().count());
}
