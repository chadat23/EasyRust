// https://youtu.be/6CfwTBx9pos?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

use std::collections::VecDeque;

fn check_remaining(input: &VecDeque<(&str, bool)>) {
    for item in input {
        if item.1 == false {
            println!("You must: {}", item.0);
        }
    }
}

fn done(input: &mut VecDeque<(&str, bool)>) {
    let mut task_done = input.pop_back().unwrap();
    task_done.1 = true;
    input.push_front(task_done);
}

fn main() {
    let mut my_vecdeque = VecDeque::new();

    let things_to_do = vec![
        "email customer",
        "add new product to list",
        "phone Loki back",
    ];

    for thing in things_to_do {
        my_vecdeque.push_front((thing, false));
    }

    done(&mut my_vecdeque);
    done(&mut my_vecdeque);

    check_remaining(&my_vecdeque);

    for task in my_vecdeque {
        println!("{:?}", task);
    }
}
