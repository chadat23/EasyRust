// https://youtu.be/-qg8wpJdSHY?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main() {
    let mut counter = 0;
    loop {
        counter += 1;
        println!("The counter is now {counter}");
        if counter == 5 {
            break;
            continue;
        }
    }    

    let mut counter = 0;
    let mut counter2 = 0;
    println!("Now entering the first loop");
    'first_loop: loop {
        counter += 1;
        println!("The counter is {counter}");
        if counter > 9 {
            println!("Now starting the secont loop");
            'second_loop: loop {
                println!("The second loop is now {counter2}");
                counter2 += 1;
                if counter2 == 3 {
                    break 'first_loop;
                }
            }
        }
    }
}
