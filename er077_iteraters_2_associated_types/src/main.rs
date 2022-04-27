// https://youtu.be/_1AJeCnGSmo?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main() {
    let my_vec = vec!['a', 'b', 'c', 'd'];  // Just a regular vec
    let mut my_vec_iter = my_vec.iter();
    assert_eq!(my_vec_iter.next(), Some(&'a'));
    assert_eq!(my_vec_iter.next(), Some(&'b'));
    assert_eq!(my_vec_iter.next(), Some(&'c'));
    assert_eq!(my_vec_iter.next(), Some(&'d'));
    assert_eq!(my_vec_iter.next(), None);
    assert_eq!(my_vec_iter.next(), None);

    let my_vec = std::iter::repeat(6).skip(6).take(6); // Will give 6 6es after skipping the first 6;
}
