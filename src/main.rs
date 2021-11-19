use std::str::FromStr;

mod cryptor;

fn main() {
    let file_name = String::from_str("image.png").unwrap();
    let cry = cryptor::Cryptor::new(file_name);
    cry.write_message(String::new());
}
