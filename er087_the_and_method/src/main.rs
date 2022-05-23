// https://youtu.be/jXy4-AteA-g?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main() {
    let one = true;
    let two = false;
    let three = true;
    let four = true;
    println!("{}", one && three);
    println!("{}", one && two && three && four);

    let first_try = vec![Some("success!"), None, Some("success!"), Some("success!"), None];
    let second_try = vec![None, Some("success!"), Some("success!"), Some("success!"), Some("success!")];
    let third_try = vec![Some("success!"), Some("success!"), Some("success!"), Some("success!"), None];
    for i in 0..first_try.len() {
        println!("{:?}", first_try[i].and(second_try[i]).and(third_try[i]))
    }

}
