use std::io::Read;


pub fn get_key() -> u8 {
    let mut buffer = [0; 1];
    std::io::stdin().read_exact(&mut buffer).unwrap();

    return buffer[0];
}

pub fn print(charr: u8) {
    print!("{}", charr as char);
}