use std::{env, io, process};

mod cryptor;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        print_help_and_exit();
    }

    let fileop = &args[1];
    let filename = &args[2];

    let mut cry = cryptor::Cryptor::new(filename.to_string());
    match &fileop[..] {
        "r" => cry.read_message(),
        "w" => {
            println!("Write your message, press Enter to finish.\n");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Incorrect string input!");
            cry.write_message(input);
        }
        _ => print_help_and_exit(),
    }
}

fn print_help_and_exit() {
    let help = "usage: steno <r|w> <image.png>";
    println!("{}", help);
    process::exit(0);
}
