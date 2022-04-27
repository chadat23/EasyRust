// https://youtu.be/uTWRaYfSvvM?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main() {
    let mut first_vec = vec![];
    for _ in 0..5 {
        let mut second_vec = vec![];
        for _ in 0..5 {
            let mut third_vec = vec![];
            third_vec.push(8);
            third_vec.push(9);
            third_vec.push(10);
            third_vec.push(11);
            third_vec.push(12);
            second_vec.push(third_vec);
        }
        first_vec.push(second_vec);
    }
    println!("{:?}", first_vec[0][0][2]);
    println!("{:?}", first_vec.get(2)
        .and_then(|v| v.get(2))
        .and_then(|v| v.get(4)));
    println!("{:?}", first_vec.get(2)
        .and_then(|v| v.get(12))
        .and_then(|v| v.get(4)));
}
