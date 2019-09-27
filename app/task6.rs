use security::checksum;

use std::path::PathBuf;


struct Args(PathBuf, String);

fn parse_args() -> Args {
    let usage = "USAGE: <file path> <algorithm name>";

    let mut args = std::env::args().into_iter().skip(1);

    Args(
        args.next().expect(usage).into(),
        args.next().expect(usage),
    )
}

fn checksum(algo: &str, data: Vec<u8>) -> String {
    match algo.to_lowercase().as_str() {
        "xor" => format!("{:X}", checksum::parity_bit(&data)),
        "sha256" => checksum::sha::sha256(data),
        _ => "no such algorithm".to_string(),
    }
}

fn main() {
    let Args(path, algo) = parse_args();

    let data = std::fs::read(path).expect("Can't open file!");
    println!("{} = {}", algo, checksum(&algo, data));
}
