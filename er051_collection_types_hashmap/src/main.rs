// https://youtu.be/Ovlt82Jz0GQ?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

use std::collections::HashMap;

struct City {
    name: String,
    population: HashMap<u32, u32>, // year, population
}

fn main() {
    let mut tallinn = City {
        name: "Tallinn".to_string(),
        population: HashMap::new(),
    };

    tallinn.population.insert(1372, 3_250);
    tallinn.population.insert(1851, 24_000);
    tallinn.population.insert(2020, 427_619);

    for (year, population) in tallinn.population {
        println!("In the year {}, the city of {} had a population of {}",
            year, tallinn.name, population
        );
    }
}
