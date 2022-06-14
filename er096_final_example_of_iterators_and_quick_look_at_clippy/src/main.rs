// https://youtu.be/OgcrRt84bUY?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

#[derive(Debug)]
struct Names {
    one_word: Vec<String>,
    two_word: Vec<String>,
    three_words: Vec<String>
}

fn main() {
    let vec_of_names = vec![
        "Ceaser",
        "Frodo Baggins",
        "Bilbo Baggins",
        "Jean-Luc Picard",
        "Data",
        "Rand Al'Thor",
        "Paul Atreides",
        "Barach Hussein Obama",
        "Bill Jefferson Clinton",
    ];

    let mut iter_of_names = vec_of_names.iter().peekable();

    let mut all_names = Names {
        one_word: vec![],
        two_word: vec![],
        three_words: vec![],
    };

    while iter_of_names.peek().is_some() {
        let next_item = iter_of_names.next().unwrap();
        // match next_item.match_indices(' ').collect::<Vec<_>>().len() {
        match next_item.match_indices(' ').count() {
            0 => all_names.one_word.push(next_item.to_string()),
            1 => all_names.two_word.push(next_item.to_string()),
            _ => all_names.three_words.push(next_item.to_string()),
        }
    }

    println!("{:?}", all_names);
}
