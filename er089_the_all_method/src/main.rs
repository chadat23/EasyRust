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

    let smaller_vec: Vec<char> = ('A'..'w').collect();
    println!("all alphabetic? {}", smaller_vec
        .iter()
        .all(|&character| character.is_alphabetic())
    );
    println!("All less than the character 'z'? {}", smaller_vec
        .iter()
        .all(|&x| x < 'z')
    );

    let mut big_vec = vec![6; 1_000_000];
    big_vec.push(5);
    let mut iterator = big_vec.iter().rev();
    // println!("{:?}", iterator.next());
    // println!("{:?}", iterator.next());
    println!("{:?}", big_vec
        .iter()
        .any(|&number| number == 5)
    );
}
