// https://youtu.be/vJSb7-YcrHc?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

#[derive(Debug)]
struct Person {
    name: String,
    real_name: String,
    height: u8,
    happy: bool,
}

fn main() {
    let papa_doc = Person {
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170,
        happy: false,
    };
    println!("{:?}", papa_doc);

    // let Person { 
    //     name: they_call_me, 
    //     real_name: b, 
    //     height: c, 
    //     happy: happyness 
    // } = papa_doc;
    // println!("{they_call_me}, {b}, {c}, {happyness}");

    let Person { 
        name: they_call_me, 
        real_name: b, 
        .. // I don't care about the rest
    } = papa_doc;
    println!("{they_call_me}, {b}");
}
