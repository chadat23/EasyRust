// https://youtu.be/RAABcA6BTVg?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn in_char_vec(char_vec: &Vec<char>, check: char) {
    println!("Is {} inside? {}", check, char_vec
        .iter()
        .any(|&char| char == check)
        );
}

fn main() {
    let char_vec = ('a'..'z').collect::<Vec<_>>();
    in_char_vec(&char_vec, 'i');
    in_char_vec(&char_vec, 'K');
}
