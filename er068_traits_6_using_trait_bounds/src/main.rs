// https://youtu.be/ld8UV-AiMTQ?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

use std::fmt::Debug;

struct Monster {
    health: i32
}

#[derive(Debug)]
struct Wizard {
    health: i32,
}

#[derive(Debug)]
struct Ranger {
    health: i32
}

trait Magic {}
trait FightClose {}
trait FightFromDistance {}

impl FightClose for Ranger {}
impl FightClose for Wizard {}
impl FightFromDistance for Ranger {}
impl Magic for Wizard {}

fn attack_with_sword<T: FightClose + Debug>(character: &T, opponent: &mut Monster) {
    opponent.health -= 10;
    println!("You attack with your sword. 
Your opponent now has {} health left. You are now at: {:?}",
    opponent.health, character);
}

fn attack_with_fireball<T>(character: &T, opponent: &mut Monster, distance: u32)
    where
        T: Magic + Debug
{
    if distance < 15 {
        opponent.health -= 20;
        println!("You attack with a fireball. 
Your opponent now has {} health left. You are now at: {:?}",
        opponent.health, character);
    }
}

fn attack_with_bow<T>(character: &T, opponent: &mut Monster, distance: u32)
    where
        T: FightFromDistance + Debug
{
    if distance < 10 {
        opponent.health -= 10;
        println!("You attack with your bow. 
Your opponent now has {} health left. You are now at: {:?}",
        opponent.health, character);
    }
}

fn main() {
    let radagast = Wizard {health: 60};
    let aragorn = Ranger {health: 80};

    let mut uruk_hai = Monster { health: 40};

    attack_with_bow(&aragorn, &mut uruk_hai, 8);
    attack_with_sword(&radagast, &mut uruk_hai);
    attack_with_fireball(&radagast, &mut uruk_hai, 12);
}

