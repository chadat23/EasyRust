// https://youtu.be/Ky3HqkWUcI0?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

const MY_NUMBER: i8 = 9; // always immutable, type must be declared, no shadowing
static MY_NUMBER_2: i8 = 8; // always immutable, type must be declared, no shadowing, static memory location

static SEASONS: [&str; 4] = ["Spring", "Summer", "Fall", "Winter"]; // seasons are fixed so might make sense to be static

fn main() {
    // const is immutable
}
