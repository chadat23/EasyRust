// https://youtu.be/sjq_0qCCQm0?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

// .into_iter() // iterator that owns its values = takes self and consumes it
// .iter() // iterator that references &self
// .iter_mut() // iterator of mutable references &mut self

// .map - does something to each item, then passes it on
// .for_each - does something to each item

fn main() {
    let vector1 = vec![1, 2, 3];
    let vector1_a = vector1.iter().map(|x| x + 1).collect::<Vec<i32>>();
    println!("{:?} {:?}", vector1, vector1_a);


    let vector1 = vec![1, 2, 3];
    let vector1_a = vector1.into_iter().map(|x| x * 10).collect::<Vec<i32>>();
    // println!("{:?} {:?}", vector1, vector1_a); // vector1 no longer exists
    println!("{:?}", vector1_a);

    let mut vector1 = vec![10, 20, 30];
    vector1.iter_mut().for_each(|x| *x += 10);
    println!("{:?}", vector1);
}
