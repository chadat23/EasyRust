// https://youtu.be/JIv1Pv4vCHU?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

use std::convert::From;

#[derive(Debug)]
struct EvenOddVec(Vec<Vec<i32>>);

// to impliment ::from(u: T) -> Self, just do below
impl From<Vec<i32>> for EvenOddVec {
    fn from(input: Vec<i32>) -> Self {
        let mut even_odd_vec: Vec<Vec<i32>> = vec![vec![], vec![]];

        for item in input {
            if item % 2 == 0 {
                even_odd_vec[0].push(item);
            } else {
                even_odd_vec[1].push(item);
            }
        }
        Self(even_odd_vec)
    }
}

fn main() {
    let bunch_of_numbers = vec![8, 7, -1, 3, 222, 9787, -47, 77, 0, 55, 7, 8];
    let new_vec = EvenOddVec::from(bunch_of_numbers);

    println!("{:?}", new_vec);
}
