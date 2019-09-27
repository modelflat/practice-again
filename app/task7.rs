#![feature(duration_float)]

use security::{sig_search, utils};

use std::path::PathBuf;
use std::time::Instant;

struct Args(PathBuf, String);


fn parse_args() -> Args {
    let usage = "USAGE: <directory> <signature file or string to search for>";

    let mut args = std::env::args().into_iter().skip(1);

    Args(
        args.next().expect(usage).into(),
        args.next().expect(usage),
    )
}


fn main() {
    let Args(path, signature) = parse_args();

    let signature = std::fs::read(&signature).unwrap_or(signature.into_bytes());

    let mut all_files = utils::files_in_tree(&path).expect("Error listing target directory");

    all_files.sort();

    let t = Instant::now();
    let mut total_size = 0;
    for (i, file) in all_files.iter().rev().enumerate() {
        // println!("[{:3}/{:3}] Looking at {:?}...", i + 1, all_files.len(), file);

        if let Some(res) = sig_search::search_file(&file, &signature)
            .expect(format!("Error searching file: {:?}", file).as_str())
        {
            println!("{:?}: found at byte position {}", res.path, res.start);
        }

        total_size += file.metadata().unwrap().len();
    }
    let t = (Instant::now() - t).as_secs_f64();

    println!(
        "Searched {} files ({} MB in total) in {:.3} seconds.\nAverage search speed: {:.1} MB/s",
        all_files.len(), total_size / (1 << 20), t, (total_size / (1 << 20)) as f64 / t
    )
}