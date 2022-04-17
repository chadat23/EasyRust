// https://youtu.be/2uh64U9JesA?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

enum Something {
    One,
    Two,
}

fn main1() {
    println!("{}", Something::One as u32);
    println!("{}", Something::Two as u8);
}

enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

fn main2() {
    use Season::*;
    let four_seasons = vec![Spring, Summer, Autumn, Winter];
    for season in four_seasons {
        println!("{}", season as u32);
    }
}

#[derive(Clone, Copy)]
enum Star {
    BrownDwarf = 10,
    RedDwarf = 50,
    YellowStar = 100,
    RedGiant = 1000,
    DeadStar,
}

fn main() {
    use Star::*;
    let starvec = vec![BrownDwarf, RedDwarf, YellowStar, RedGiant, DeadStar];
    for star in starvec {
        match star as u32 {
            size if size <= 80 => println!("Not the biggest star {}", size),
            size if size > 80 => println!("This is a good sized star {}", size),
            _ => println!("Some other sized star {}", star as u32),
        }
    }
    println!("What about DeadStar? It's the number {}", DeadStar as u32);
}
