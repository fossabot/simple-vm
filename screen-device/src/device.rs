const CLEAR_SCREEN: &str = "\x1b[2J";
const SET_BOLD: &str = "\x1b[1m";
const SET_REGULAR: &str = "\x1b[0m";

pub fn set_memory_u16(address: u16, data: u16) {
    let command = (data & 0xff00) >> 8 as u8;
    let character_value = (data & 0x00ff) as u8 as char;

    match command {
        0xff => erase_screen(),
        0x01 => set_bold(),
        0x02 => set_regular(),
        _ => ()
    }

    let x = (address % 16) + 1;
    let y = (address / 16) + 1;

    move_to(x, y);
    println!("{}", character_value);
}

pub fn erase_screen() {
    println!("{}", CLEAR_SCREEN);
}

pub fn move_to(x: u16, y: u16) {
    println!("\x1b[${};${}H", y, x);
}

pub fn set_bold() {
    println!("{}", SET_BOLD);
}

pub fn set_regular() {
    println!("{}", SET_REGULAR);
}