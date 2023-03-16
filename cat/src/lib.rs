use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{Arg, ArgAction, Command};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_non_blank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("cat")
        .version("1.0.0")
        .author("Mehmet Akifhan ILGAZ")
        .about("Rust cat")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .num_args(1..)
                .default_value("-"),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .help("Number lines")
                .action(ArgAction::SetTrue)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::new("number_nonblank")
                .short('b')
                .long("number-nonblank")
                .help("Number nonblank lines")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    Ok(Config {
        files: matches
            .get_many("files")
            .expect("Files required")
            .cloned()
            .collect(),
        number_lines: matches.get_flag("number"),
        number_non_blank_lines: matches.get_flag("number_nonblank"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut index = 1;
    for filename in &config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(buff) => {
                let mut print_func: fn(usize, &str) = print_line;

                if config.number_lines || config.number_non_blank_lines {
                    print_func = print_line_with_index;
                }

                for line in buff.lines() {
                    let text = line?;

                    if text.is_empty() && config.number_non_blank_lines {
                        println!();
                        continue;
                    }
                    print_func(index, &text);

                    index += 1;
                }
            }
        }
    }
    Ok(())
}

fn open(file_name: &str) -> MyResult<Box<dyn BufRead>> {
    match file_name {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file_name)?))),
    }
}

fn print_line_with_index(index: usize, text: &str) {
    println!("{:6}\t{}", index, text);
}

fn print_line(_: usize, text: &str) {
    println!("{}", text)
}
