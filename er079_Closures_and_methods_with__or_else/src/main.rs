// https://youtu.be/f71I1XhLgqs?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

// officially, a "closure" encloses something
// an "anonymous function" is like a closure but doesn't enclose, capture, anything

// _or() returns a something, _or_else() returns a closure

fn main() {
    let x = 7;
    let my_closure = || println!("{}", x + 10);
    my_closure();

    let x: Option<String> = None;
    println!("{:?}", x.unwrap_or("nothing inside".to_string()));
    let x: Option<String> = None;
    println!(
        "{:?}",
        x.unwrap_or_else(|| {
            let mut my_string = "I am but a string".to_string();
            my_string.push('!');
            my_string
        })
    );
}
