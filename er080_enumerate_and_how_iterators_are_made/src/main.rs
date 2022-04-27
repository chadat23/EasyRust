// https://youtu.be/5lqkQ1HFsyk?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main() {
    let num_vec = vec![10, 9, 8];
    num_vec
        .iter()
        .enumerate()
        .for_each(|(index, number)| println!("Index number {index} is the number {number}"));

    let num_vec = vec![10, 9, 8];
    let new_vec = num_vec
        .into_iter()
        .enumerate()
        .map(|(index, mut number)| {
            number += 1;
            (index, number)
        })
        .collect::<Vec<_>>();
    println!("{:?}", new_vec);
}
