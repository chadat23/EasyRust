// https://youtu.be/L6rMIVRxwDc?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main() {
    let my_vec = (0..10)
        .skip(2)
        .take(5)
        .collect::<Vec<_>>();
    println!("{:?}", my_vec);

    let my_vec = (0..10)
        .skip_while(|&x| x < 4)
        .take_while(|&x| x < 8)
        .collect::<Vec<_>>();
    println!("{:?}", my_vec);

    let my_vec = [1, 2, 3]
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    println!("{:?}", my_vec);

    // watch the video for by_ref and sum
}
