use std::io;

use clap::{ArgGroup, Parser};

#[derive(Parser)]
#[clap(name = "steno")]
#[clap(version, about, long_about = None)]
#[clap(group(
            ArgGroup::new("fileop")
                .required(true)
                .args(&["write", "read"]),
        ))]
struct Cli {
    #[clap(short, long)]
    read: bool,

    #[clap(short, long)]
    write: bool,

    /// filename input
    #[clap(group = "input")]
    filename: String,
}

mod cryptor;

fn main() {
    let cli = Cli::parse();

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
