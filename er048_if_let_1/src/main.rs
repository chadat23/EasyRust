// https://youtu.be/smex41M4CRw?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

// if let is for when you want to match on Some(thing) and don't car about None

fn main() {
     let my_vec = vec![2, 3, 4];
     let get_one = my_vec.get(0);
     let get_two = my_vec.get(10);
     println!("{:?}, {:?}", get_one, get_two);

     let my_vec = vec![2, 3, 4];
     for index in 0..10 {
        //  match my_vec.get(index) {
        //      Some(number) => println!("The number is: {number}"),
        //      none => {}
        //  }
        if let Some(number) = my_vec.get(index) {
            println!("The number is: {number}");
        }
     }
}
