use security::*;

struct Args(OperationMode, String, String, char, Option<String>);

fn parse_args() -> Args {
    let usage = "USAGE: <operation mode> <container file> <input file or text> [secret char] [output file]";
    let mut args = std::env::args().into_iter().skip(1);

    Args(
        args.next().expect(usage).parse().expect("Can't parse OperationMode"),
        args.next().expect(usage),
        args.next().expect(usage),
        args.next().map_or(' ', |s| s.chars().next().unwrap()),
        args.next(),
    )
}

fn main() {
    let Args(mode, container_file, input, secret_char, output_file) = parse_args();

    let mut container = std::fs::read_to_string(container_file)
        .expect("Cannot read container")
        .split_terminator('\n')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let text = std::fs::read_to_string(&input).unwrap_or(input);

    match
        match mode {
            OperationMode::Encrypt => lines::hide(&text, &mut container, secret_char),
            OperationMode::Decrypt => lines::reveal(&container, secret_char),
        }
        {
            Ok(output) => match output_file {
                Some(filename) => { std::fs::write(filename, output.as_bytes()).expect("Can't write output file"); },
                None => println!("{}", output)
            },
            Err(AlgorithmError { what }) => eprintln!("AlgorithmError: {}", what),
        };
}
