use clap::{Arg, Command, value_parser};

fn main() {
    let matches = Command::new("echo-rs")
    .version("0.1.0")
    .author("me@codejitsu.net")
    .about("Rust echo")    
    .arg(
        Arg::new("text")
        .value_name("TEXT")
        .help("Input text")
        .required(true)
        .value_parser(value_parser!(String))
        .num_args(1..),
    )
    .arg(
        Arg::new("omit_newline")
        .short('n')
        .required(false)
        .action(clap::ArgAction::SetTrue)
        .help("Do not print newline"),
    )
    .get_matches();

    let text: Vec<&str> = matches.get_many::<String>("text").unwrap().map(|s| s.as_str().trim()).collect();
    let omit_newline = matches.get_flag("omit_newline");

    let ending = if omit_newline { "" } else { "\n" };

    println!("{}{}", text.join(" "), ending)
}
