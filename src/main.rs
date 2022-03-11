use std::{io, path::PathBuf};

use clap::{ArgGroup, Parser, Command, Arg};

mod cryptor;

fn main() {
    let matches = Command::new("steno")
        .about("package manager utility")
        .version("5.2.1")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Pacman Development Team")
        // Query subcommand
        //
        // Only a few of its arguments are implemented below.
        .arg(
            Arg::new("read")
                .short('r')
                .long("read")
                .help("Reads the PNG hidden message.")
        )
        // Sync subcommand
        //
        // Only a few of its arguments are implemented below.
        .subcommand(
            Command::new("write")
                .short_flag('w')
                .long_flag("write")
                .about("Writes content to PNG file.")
                .arg(
                    Arg::new("search")
                        .short('s')
                        .long("search")
                        .conflicts_with("info")
                        .takes_value(true)
                        .multiple_values(false)
                        .help("search remote repositories for matching strings"),
                )
                .arg(
                    Arg::new("package")
                        .help("packages")
                        .required_unless_present("search")
                        .takes_value(true)
                        .multiple_values(true),
                ),
        )
        .get_matches();

    let (read, write) = (cli.read, cli.write);
    match (read, write) {
        (true, _) => {
            let cry = cryptor::Cryptor::new(cli.filename);
            let msg = cry.read_file_message();
            print!("Secret message:\n{}", msg);
        }
        (_, true) => {
            let mut cry = cryptor::Cryptor::new(cli.filename);
            println!("Write your message, press Enter to finish.\n");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Incorrect string input!");
            cry.write_message(input);
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn write_message_to_image_and_read() {
        let mut cry = cryptor::Cryptor::new(String::from("./tests/examples/beetle.png"));

        let msg = "!!!TEST MESSAGE!!!";
        cry.write_message(String::from(msg));

        assert_eq!(msg, &cry.read_file_message()[..]);
    }
}
