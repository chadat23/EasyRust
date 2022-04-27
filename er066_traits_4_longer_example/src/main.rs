// https://youtu.be/dmPKGL6Gl0I?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

struct Monster {
    health: i32
}
struct Wizard {}
struct Ranger{}

trait FightClose {
    fn attach_with_sword(&self, opponent: &mut Monster) {
        opponent.health -= 10;
        println!("You attacked with your sword. Your opponent has {} health left", opponent.health)
    }

    fn attach_with_hand(&self, opponent: &mut Monster) {
        opponent.health -= 2;
        println!("You attacked with your hand. Your opponent has {} health left", opponent.health)
    }
}

impl FightClose for Wizard {}
impl FightClose for Ranger {}

trait FightFromDistance {
    fn attach_with_bow(&self, opponent: &mut Monster, distance: u32) {
        if distance < 10 {
            opponent.health -= 10;
            println!("You attacked with your bow. Your opponent has {} health left", opponent.health)
        }
    }

    fn attach_with_rock(&self, opponent: &mut Monster, distance: u32) {
        if distance < 3 {
            opponent.health -= 4;
            println!("You attacked with a rock. Your opponent has {} health left", opponent.health)
        }
    }
}

impl FightFromDistance for Ranger {}

fn main() {
    let radagast = Wizard{};
    let aragorn = Ranger {};

    let mut uruk_hai = Monster {health: 40};

    let distance = 8;
    radagast.attach_with_sword(&mut uruk_hai);
    aragorn.attach_with_bow(&mut uruk_hai, distance)

}
