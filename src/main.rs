use std::io;

use clap::{ArgGroup, Parser};

#[derive(Parser)]
#[clap(name = "steno")]
#[clap(author = "Joao R. <jpedrocr2000@hotmail.com>")]
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
