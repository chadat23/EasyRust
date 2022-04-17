// https://youtu.be/vqTK35kw7wQ?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

#[derive(Debug)]
struct  Animal {
    age: u8,
    animal_type: AnimalType,
}

#[derive(Debug)]
enum AnimalType{
    Cat,
    Dog,
}

impl Animal {
    fn new() -> Self {
        Self {
            age: 10,
            animal_type: AnimalType::Cat,
        }
    }

    fn change_to_dog(&mut self) {
        println!("Changing animal to dog.");
        self.animal_type = AnimalType::Dog;
    }

    fn change_to_cat(&mut self) {
        println!("Changing animal to cat.");
        self.animal_type = AnimalType::Cat;
    }

    fn check_type(&self) {
        use AnimalType::*;
        match self.animal_type {
            Cat => println!("The animal is a cat."),
            Dog => println!("The animal is a dog."),
        };
    }
}

fn main() {
    let mut new_animal = Animal::new();

    println!("{:?}", new_animal);

    new_animal.change_to_dog();
    new_animal.check_type();
    new_animal.change_to_cat();
    new_animal.check_type();
}
