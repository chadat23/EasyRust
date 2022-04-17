// https://youtu.be/SRnqNTJUgjs?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://youtu.be/F_EcbWM63lk?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

/// struct = something AND something AND something
/// enum = something OR something OR something

enum ThingsInTheSky {
    Sun,
    Stars,
}

fn create_sky_state(time: i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun,
        _ => ThingsInTheSky::Stars,
    }
}

fn check_skystate(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun => println!("I can see the sun"),
        ThingsInTheSky::Stars => println!("I can see the stars"),
    }
}

fn main() {
    let time = 18;
    let skystate = create_sky_state(time);
    check_skystate(&skystate);
}
