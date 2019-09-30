use security::checksum;

use std::path::PathBuf;

struct Args(String, PathBuf);

fn parse_args() -> Args {
    let usage = "USAGE: <algorithm name> <file path>";

    let mut args = std::env::args().into_iter().skip(1);

    Args(
        args.next().expect(usage),
        args.next().expect(usage).into(),
    )
}

fn checksum(algo: &str, data: Vec<u8>) -> String {
    match algo.to_lowercase().as_str() {
        "xor" => checksum::parity_bit(data),
        "sha256" => checksum::sha::sha256(data),
        _ => "unknown algorithm".to_string(),
    }
}

fn main() {
    let Args(algo, path) = parse_args();

    let data = std::fs::read(path).expect("Can't open file!");
    println!("{} = {}", algo, checksum(&algo, data));
}
