// https://youtu.be/EbtvILqrUcg?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

 fn check_if_five(number: i32) -> Result<i32, String> {
     match number {
         5 => Ok(number),
         _ => Err("The number wasn't 5".to_string()),
     }
 }

fn main() {
     let mut result_vec = Vec::new();

     for number in 2..=7 {
         result_vec.push(check_if_five(number));
     }

     println!("{:#?}", result_vec);

     for item in result_vec {
         if item.is_ok() {
             println!("{}", item.unwrap())
         } else {
             println!("{:?}", item)
         }
     }
}
