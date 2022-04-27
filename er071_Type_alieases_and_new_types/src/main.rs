// https://youtu.be/xStMBBnfKyA?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

// type alias = a different name for the existing type

type MyString = String;
type MyNumber = Vec<Vec<Vec<Vec<[i32; 4]>>>>;

fn return_something() -> MyNumber {
    vec![vec![vec![vec![[7, 8, 9, 10]]]]]
}

// you could inpliment stuff on this that couldn't be implimented on String
#[derive(Debug)]
struct MyString2(String);

fn main() {
    let x = String::from("This is some writing");
    let y = MyString::from("This is some writing");
    println!("{}", x == y);

    let x = MyString2(String::from("Put the string inside here"));
    println!("{:?}", x);
    println!("{}", x.0); // Here you can easily access the String
}
