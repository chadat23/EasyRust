// https://youtu.be/4YivPkdw53M?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

// Fn - takes by reference, (&self, ...
// FnMut - Takes by mututable reference, (&mut self, ...
// FnOnce - takes by value, (self, ...

fn main() {
    let months = vec!["January", "February", "March", "April", "May", "June", 
    "July", "August", "September", "October", "November", "December"];
    let filtered_months: Vec<&str> = months
        .into_iter()
        .filter(|month| month.len() < 5 && month.len() > 3)
        .filter(|month| month.contains("u"))
        .collect();
    println!("{:?}", filtered_months);

    let mut my_vec = vec![1, 2, 3, 4, 5];
    my_vec.retain(|item| item % 2 == 0);
    println!("{:?}", my_vec); 
}
