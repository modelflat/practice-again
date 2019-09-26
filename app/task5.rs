use security::*;
use security::ru_en_similarity::MappingDirection;

struct Args(OperationMode, String, String, MappingDirection, Option<String>);

fn parse_args() -> Args {
    let usage = "USAGE: <operation mode> <container file> <input file or text> [direction] [output file]";
    let mut args = std::env::args().into_iter().skip(1);

    Args(
        args.next().expect(usage).parse().expect("Can't parse OperationMode"),
        args.next().expect(usage),
        args.next().expect(usage),
        args.next().map_or(MappingDirection::EnRu, |s| s.parse().expect("Can't parse direction")),
        args.next(),
    )
}

fn main() {
    let Args(mode, container_file, input, direction, output_file) = parse_args();

    let container = std::fs::read_to_string(container_file).expect("Cannot read container");
    let text = std::fs::read_to_string(&input).unwrap_or(input);

    match
        match mode {
            OperationMode::Encrypt => ru_en_similarity::hide(&text, &container, direction),
            OperationMode::Decrypt => ru_en_similarity::reveal(&container, direction),
        }
        {
            Ok(output) => match output_file {
                Some(filename) => { std::fs::write(filename, output.as_bytes()).expect("Can't write output file"); },
                None => println!("{}", output)
            },
            Err(AlgorithmError { what }) => eprintln!("AlgorithmError: {}", what),
        };
}
