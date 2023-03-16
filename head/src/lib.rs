use std::error::Error;

use clap::{Arg, ArgAction, Command};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: u64,
    bytes: Option<u64>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("head")
        .about("Rust head")
        .version("1.0.0")
        .author("Mehmet Akifhan ILGAZ")
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .help("Input file(s)")
                .num_args(1..)
                .default_value("-"),
        )
        .arg(
            Arg::new("lines")
                .short('n')
                .long("lines")
                .help("Number of lines")
                .value_name("LINES")
                .value_parser(clap::value_parser!(u64).range(1..))
                .default_value("10"),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .help("Number of bytes")
                .value_name("BYTES")
                .value_parser(clap::value_parser!(u64).range(1..))
                .num_args(0..=1)
                .conflicts_with("lines"),
        )
        .get_matches();

    Ok(Config {
        files: matches
            .get_many("files")
            .expect("file required")
            .cloned()
            .collect(),
        lines: matches.get_one("lines").cloned().unwrap(),
        bytes: matches.get_one("bytes").cloned(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

fn parse_positive_int(input: &str) -> MyResult<usize> {
    match input.parse() {
        Ok(val) if val > 0 => Ok(val),
        _ => Err(From::from(input)),
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(3, res.unwrap());

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
