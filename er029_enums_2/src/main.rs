// https://youtu.be/F_EcbWM63lk?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

enum ThingsInTheSky {
    Sun(String),
    Stars(String),
}

fn create_sky_state(time: i32) -> ThingsInTheSky {
    let string2 = String::from("The stars are out and I can see them!");
    match time {
        6..=18 => ThingsInTheSky::Sun(String::from("I can see the sun from up here!")),
        _ => ThingsInTheSky::Stars(string2),
    }
}

fn check_skystate(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun(description) => println!("{description}"),
        ThingsInTheSky::Stars(description) => println!("{description}"),
    }
}

fn main1() {
    let time = 18;
    let skystate = create_sky_state(time);
    check_skystate(&skystate);
}

enum Mood {
    Happy,
    Sleepy,
    NotBad,
    Angry,
}

fn match_mood(mood: &Mood) -> i32 {
    use Mood::*;
    
    match mood {
        Happy => 10,
        Sleepy => 6,
        NotBad => 7,
        Angry => 2,
    }
}

fn main() {
    let my_mood = Mood::NotBad;
    let happiness_level = match_mood(&my_mood);
    println!("Out of 1 - 10, my happiness level is a {happiness_level}");
}
