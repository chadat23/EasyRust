// https://youtu.be/Eh-DsRnDKmw?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main() {
    let name1 = String::from("Windy");
    let name2 = String::from("Gomesy");
    let name3 = String::from("MB");
    let mut my_vec = Vec::new();
    my_vec.push(name1);
    my_vec.push(name2);
    my_vec.push(name3);
    println!("{:?}", my_vec);

    let my_vec: Vec<String> = Vec::new();

    let my_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let my_slice = &my_vec[2..5];

    let mut num_vec: Vec<char> = Vec::new();
    println!("{}", num_vec.capacity()); // 0 elements: prints 0
    num_vec.push('a');
    println!("{}", num_vec.capacity()); // 4 elements: prints 4
    num_vec.push('a');
    println!("{}", num_vec.capacity()); // 4 elements: prints 4
    num_vec.push('a');
    println!("{}", num_vec.capacity()); // 4 elements: prints 4
    num_vec.push('a');
    println!("{}", num_vec.capacity()); // 4 elements: prints 4
    num_vec.push('a');
    println!("{}", num_vec.capacity()); // 4 elements: prints 4
    num_vec.push('a');
    println!("{}", num_vec.capacity()); // 4 elements: prints 4

    let mut num_vec: Vec<char> = Vec::with_capacity(5);
    println!("{}", num_vec.capacity()); // 0 elements: prints 0
    num_vec.push('a');
    println!("{}", num_vec.capacity()); // 4 elements: prints 4
    num_vec.push('a');
    println!("{}", num_vec.capacity()); // 4 elements: prints 4
    num_vec.push('a');
    println!("{}", num_vec.capacity()); // 4 elements: prints 4
    num_vec.push('a');
    println!("{}", num_vec.capacity()); // 4 elements: prints 4
    num_vec.push('a');
    println!("{}", num_vec.capacity()); // 4 elements: prints 4
    num_vec.push('a');
    println!("{}", num_vec.capacity()); // 4 elements: prints 4

    let my_vec: Vec<u8> = [1, 2, 3, 4, 5, 6, 7, 8, 9].into();
    let my_vec: Vec<_> = [1, 2, 3, 4, 5, 6, 7, 8, 9].into(); // This lets rust choose the type, assuming it can, so in this case it'd be i32
}