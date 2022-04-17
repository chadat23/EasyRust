// https://youtu.be/DBdbe2QUlf8?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

// use std::collections::BTreeMap;

// struct City {
//     name: String,
//     population: BTreeMap<u32, u32>, // year, population
// }

// fn main() {
//     let mut tallinn = City {
//         name: "Tallinn".to_string(),
//         population: BTreeMap::new(),
//     };

//     tallinn.population.insert(1372, 3_250);
//     tallinn.population.insert(1851, 24_000);
//     tallinn.population.insert(2020, 427_619);

//     for (year, population) in tallinn.population {
//         println!("In the year {}, the city of {} had a population of {}",
//             year, tallinn.name, population
//         );
//     }
// }


use std::collections::HashMap;

fn main() {
    let canadian_cities = vec!["Calgary", "Vancouver", "Gimli"];
    let gearman_cities = vec!["Karsruhe", "Bad Doberan", "Bielefeld"];

    let mut city_hashmap = HashMap::new();

    for city in canadian_cities {
        city_hashmap.insert(city, "Canada");
    }

    for city in gearman_cities {
        city_hashmap.insert(city, "Germany");
    }

    println!("{:?}", city_hashmap["Bielefeld"]);
    // println!("{:?}", city_hashmap["foo"]);  // panics
    println!("{:?}", city_hashmap.get("Bielefeld"));  // panics
    println!("{:?}", city_hashmap.get("foo"));  // panics

}
