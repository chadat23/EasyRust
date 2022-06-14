// https://youtu.be/AX9FZ1MJOOo?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

use core::num;

fn main() {
    let numbs = [1, 2, 3, 4, 5];
    // let mut chunks = numbs.chunks(3);
    let mut windows = numbs.windows(3);
    for chunk in numbs.chunks(3) {
        println!("{:?}", chunk)
    }
    println!("****************");
    while let Some(w) = windows.next() {
        println!("{:?}", w)
    }

    let rules = "Rule number1: No fighting.
Rule number 2: Go to bed at 8 pm.
Rule number 3: Wake up at 6 am.";
    let rule_locations = rules.match_indices("Rule").collect::<Vec<(_, _)>>();
    println!("{:?}", rule_locations);
}
