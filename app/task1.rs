use std::io::{stdout, Write};

use security::*;

struct Args(String, usize);

fn parse_args() -> Args {
    let usage = "USAGE: <input file or text> <window size>";
    let mut args = std::env::args().into_iter().skip(1);

    Args(
        args.next().expect(usage),
        args.next().expect(usage).parse().expect("Can't parse window size"),
    )
}

fn main() {
    let Args(filename, size) = parse_args();

    let filtered_text = std::fs::read_to_string(&filename)
        .unwrap_or(filename) // assume that this is the string to parse, if can't load
        .chars()
        .filter(utils::is_russian_char)
        .collect::<String>();

    for (el, n) in security::frequencies(&filtered_text, size) {
        if let Err(_) = writeln!(stdout(), "{} - {:.6}", el, n) {
            break;
        }
    }
}