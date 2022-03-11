use std::io;

use clap::{command, Arg, ArgGroup};

mod cryptor;

fn main() {
    let matches = command!()
        .arg(
            Arg::new("from_file")
                .short('f')
                .long("from-file")
                .takes_value(true)
                .value_name("FROM_FILE")
                .help("Extracts the data from a file into the PNG."),
        )
        .arg(
            Arg::new("read")
                .short('r')
                .long("read")
                .takes_value(true)
                .value_name("TARGET_PNG")
                .help("Reads the PNG hidden message."),
        )
        .arg(
            Arg::new("write")
                .short('w')
                .long("write")
                .takes_value(true)
                .value_name("TARGET_PNG")
                .help("Write mode."),
        )
        .group(
            ArgGroup::new("file_op")
                .required(true)
                .args(&["read", "write"]),
        )
        .get_matches();

    let (read, write) = (matches.is_present("read"), matches.is_present("write"));

    match (read, write) {
        (true, _) => {
            let target_filename = matches.value_of("read").unwrap();
            let cry = cryptor::Cryptor::new(String::from(target_filename));
            let msg = cry.read_file_message();
            print!("Secret message:\n{}", msg);
        }
        (_, true) => {
            let target_filename = matches.value_of("write").unwrap();
            let mut cry = cryptor::Cryptor::new(String::from(target_filename));

            if matches.is_present("from_file") {
                let from_file = matches.value_of("from_file").unwrap();
                cry.write_from_file(from_file);
            } else {
                println!("Write your message, press Enter to finish.\n");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Incorrect string input!");
                cry.write_message(input);
            }
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn write_message_to_image_and_read() {
        let mut cry = cryptor::Cryptor::new(String::from("./tests/examples/beetle.png"));

        let msg = "!!!TEST MESSAGE!!!";
        cry.write_message(String::from(msg));

        assert_eq!(msg, &cry.read_file_message()[..]);
    }

    #[test]
    fn write_message_from_file_to_image_and_read() -> Result<(), Box<dyn std::error::Error>> {
        let mut cry = cryptor::Cryptor::new(String::from("./tests/examples/beetle.png"));

        let secret_file = "./tests/examples/secret.txt";
        cry.write_from_file(secret_file);

        let msg = fs::read_to_string(secret_file)?;
        assert_eq!(&msg[..], &cry.read_file_message()[..]);
        Ok(())
    }
}
