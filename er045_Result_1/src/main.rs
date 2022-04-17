// https://youtu.be/hyPbjVRSu4Y?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn check_error() -> Result<(), ()> {
    Ok(()) // returning nothing inside the Ok
}

fn give_result(input: i32) -> Result<(), ()> {
    if input % 2 == 0 {
        return Ok(())
    } else {
        return Err(())
    }
}

fn main() {
     check_error().is_err();

     if give_result(5).is_ok() {
         println!("The number is ok")
     } else {
         println!("It's an error")
     }
}
