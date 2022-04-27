// https://youtu.be/YEx1ABiNeBc?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

// https://youtu.be/grU-4u0Okto

struct Animal {
    name: String
}

// usually verbs or adjectives
trait Canine {
    fn bark(&self) {
        println!("Woof woof");
    }

    // would mean that "run" wouldn't need to be implimented for each struct that impliments "Canine"
    // fn run(&self) {
    //     println!("The dog is running");
    // }

    fn run(&self);
}

impl Canine for Animal {
    fn run(&self) {
        let dog_number = 5;
        println!("The dig {dog_number} is running fast")
    }
}

fn main() {
    let rover = Animal {
        name: "Rover".to_string()
    };

    rover.bark();
    rover.run();
}
