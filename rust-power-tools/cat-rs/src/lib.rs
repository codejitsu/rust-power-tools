use std::{error::Error, fs::File, io::{self, BufRead, BufReader}};
use clap::{value_parser, Arg, Command};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for file in config.files {
        match open(&file) {
            Ok(reader) => {
                let mut start_index = 0;
                reader.lines().map(|line_result| {
                    let line = line_result.unwrap_or_default();
                    let is_empty = line.trim().is_empty();

                    if config.number_nonblank_lines && is_empty {
                        (None, line)
                    } else if config.number_lines || config.number_nonblank_lines && !is_empty {
                        start_index = start_index + 1;
                        (Some(start_index), line)
                    } else {
                        (None, line)
                    }
                }).for_each(|(num_opt, line)| {
                    match num_opt {
                        Some(num) => println!("{} {}", num, line),
                        None => println!("{}", line)
                    }
                });
            },
            Err(e) => eprintln!("Failed to open {} ({})", file, e),
        }
    }

    Ok(())
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("cat-rs")
        .version("0.1.0")
        .author("me@codejitsu.net")
        .about("Rust cat")
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .help("Input files")
                .required(false)
                .value_parser(value_parser!(String))
                .default_value("-")
                .num_args(0..),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .required(false)
                .action(clap::ArgAction::SetTrue)
                .conflicts_with("number-nonblank")
                .help("Number all output lines"),
        )
        .arg(
            Arg::new("number-nonblank")
                .short('b')
                .long("number-nonblank")
                .required(false)
                .action(clap::ArgAction::SetTrue)
                .help("Number nonempty output lines, overrides -n"),
        )
        .get_matches();

    let files: Vec<String> = matches
        .get_many::<String>("files")
        .unwrap()
        .map(|s| String::from(s.as_str().trim()))
        .collect();
    let number = matches.get_flag("number");
    let number_nonblank = matches.get_flag("number-nonblank");

    Ok(Config { 
        files: files, 
        number_lines: number, 
        number_nonblank_lines: number_nonblank 
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    if filename == "-" { 
        Ok(Box::new(BufReader::new(io::stdin())))
    } else {
        Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}