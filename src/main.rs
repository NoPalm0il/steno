use std::fs::{File};
use std::error::Error;
use std::result::Result;
use std::io::{Read, stdin};

fn main() {
    // prompt user for file path
    let file_content = match file_read(String::from("image.png")) {
        Ok(content) => content,
        Err(_) => panic!("failed to read file"),
    };
    
    print_chunks(&file_content);

    //let mut message = String::new();
    //println!("message to hide shhhhh:");
    //stdin().read_line(&mut message).expect("failed to read user input");
    let message = "using steganography to hide this message".to_string();
    write_message(message, file_content);
}

fn file_read(path: String) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut f = File::open(path)?;
    let mut contents: Vec<u8> = Vec::new();
    f.read_to_end(&mut contents)?;

    Ok(contents)
}

fn print_chunks(file_content: &Vec<u8>) {
    let content_len: usize = file_content.len();
    // 4 magic bytes file type and + 4 for "signature", 8 total
    // println!("FILE: {:02X?} - total 8 bytes", &file_content[..8]);
    // 25 bytes IHDR / image header
    // 8+25 = 33
    println!("IHDR CHUNK: {:02X?} - total 25 bytes", &file_content[8..33]);
    // IDAT chunk has 24 bytes, 33+24 = 57
    println!("IDAT CHUNK: {:02X?} - total 24 bytes", &file_content[33..57]);
    // data
    println!("DATA: {:?}", &file_content[57..content_len-12]);
    println!("DATA bytes: {} bytes", &file_content[57..content_len-12].len());
    // end file
    //println!("IEND: {:02X?}", &file_content[content_len-12..]);

    println!("Total bytes: {} bytes", content_len);
}

fn write_message(message: String, file_content: Vec<u8>) {
    let message_bytes = message.as_bytes();
    println!("message bytes: {:02X?}\n", message_bytes);
    // one char from the message is 1 byte / 8 bits, the last 2 bits from
    // each byte from the image will be used to save 2 bits from each char
    // foreach char it will be needed 4 image bytes
    for byte in message_bytes {
        let mut temp_byte = byte.clone();
        for _ in 0..4 {
            // get the 2 bits value
            let bits = temp_byte % 4;
            temp_byte >>= 2;
            print!("{:02b}", bits);
        }
        print!("  ");
    }
}
