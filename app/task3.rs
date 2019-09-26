/// Does NOT filter input text to avoid introducing OperationMode into args.

use security::*;

struct Args(String, String, Option<String>);

fn parse_args() -> Args {
    let usage = "USAGE: <input file or text> <key> [output file]";
    let mut args = std::env::args().into_iter().skip(1);

    Args(
        args.next().expect(usage),
        args.next().expect(usage),
        args.next(),
    )
}

fn main() {
    let Args(input, key, output) = parse_args();

    let mut text = std::fs::read(&input)
        .unwrap_or(input.into_bytes());

    xor_inplace(&mut text, key.as_bytes());

    if let Some(filename) = output {
        std::fs::write(filename, &text).expect("Can't write output file");
    } else {
        println!("{}", String::from_utf8_lossy(&text));
    }
}
