const CLEAR_SCREEN: &str = "\x1b[2J";
const SET_BOLD: &str = "\x1b[1m";

pub fn hello_world() {
    println!("{}", CLEAR_SCREEN);
    println!("{}", SET_BOLD);
    println!("Hello World");
}