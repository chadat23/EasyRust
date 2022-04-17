// https://youtu.be/9EOtSysFI-s?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

// a work queue = which task is the most important

use std::collections::BinaryHeap;

fn show_remainder(input: &BinaryHeap<i32>) -> Vec<i32> {
    let mut remainder_vec = vec![];
    for number in input {
        remainder_vec.push(*number);
    }
    remainder_vec
}

fn main() {
    let mut my_heap = BinaryHeap::new();
    my_heap.push(100);
    my_heap.push(10000);
    my_heap.push(-5);
    my_heap.push(-500);
    my_heap.push(57);
    while let Some(number) = my_heap.pop() {
        println!("{}", number);
    }

    let many_numbers = vec![0, 5, 10, 15, 20, 25, 30]; // These numbers are in order
    let mut my_heap = BinaryHeap::new();
    for number in many_numbers {
        my_heap.push(number);
    }
    while let Some(number) = my_heap.pop() {
        println!("Popped off {number}. The remaining numbers are: {:?}", show_remainder(&my_heap));
    }
}
