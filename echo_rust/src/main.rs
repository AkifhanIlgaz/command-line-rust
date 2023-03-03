use clap::{Arg, ArgAction, Command};
fn main() {
    let matches = Command::new("echo_rust")
        .version("0.1.0")
        .author("Mehmet Akifhan ILGAZ <akifhanilgazz@gmail.com>")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print new line")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let text: Vec<String> = matches
        .get_many("text")
        .expect("text is required")
        .cloned()
        .collect();

    let omit_newline = matches.get_flag("omit_newline");

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" })
}
