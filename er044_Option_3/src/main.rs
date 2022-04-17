// https://youtu.be/LxjLR3zJQ0o?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4])
    }
}

fn main() {
     let new_vec = vec![1, 2];
     lelt bigger_vec = vec![1, 2, 3, 4, 5];
     let vec_of_vec = vec![new_vec, bigger_vec];
     for vec in vec_of_vec {
         let inside_number = take_fifth(vec);
         if inside_number.is_some() {
             println!("We got: {:?}", inside_number.unwrap());
         } else {
             println!("You get nothing!");
         }
     }

     for vec in vec_of_vec {
        let inside_number = take_fifth(vec);
        println!("We got: {:?}", inside_number.unwrap_or(0));
    }
}
