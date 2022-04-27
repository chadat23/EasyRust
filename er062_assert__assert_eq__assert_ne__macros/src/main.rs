// https://youtu.be/LfpILBEN6fE?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

const REAL_NAME: &str = "Loki Laufeyson";

fn main() {
    let my_name = "Loki Laufeyson";
    assert!(my_name == REAL_NAME, "Name must be {}", REAL_NAME);
    assert_eq!(my_name, REAL_NAME, "Name must be {}", REAL_NAME);
    assert_ne!(my_name, "Something Else", "Name must not be Something Else");

    let mut my_vec = vec![8, 9, 10];
    // immagine this being in docs
    assert_eq!(my_vec.pop(), Some(10));
    assert_eq!(my_vec.pop(), Some(9));
    assert_eq!(my_vec.pop(), Some(8));
    assert_eq!(my_vec.pop(), None);
    assert_eq!(my_vec.pop(), None);
}
