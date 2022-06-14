// https://youtu.be/qmtow7Hojtk?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main() {
    let mut my_number = dbg!(9);
    dbg!(my_number += 10);

    let new_vec = dbg!(vec![8, 9, 10]);

    let doubele_vec = dbg!(new_vec
        .iter()
        .map(|x| x * 2)
        .collect::<Vec<i32>>());

    dbg!(doubele_vec);
}
