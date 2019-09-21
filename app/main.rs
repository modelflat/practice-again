use security;

struct Args(String, usize);

fn parse_args() -> Args {
    let mut args = std::env::args().into_iter().skip(1);

    Args(args.next()
             .expect("First argument should be path to a file or a string to parse"),
         args.next()
             .expect("Second argument should be a window size")
             .parse()
             .ok()
             .expect("Can't parse window size. Should be an integer!")
    )
}

fn main() {
    let Args(filename, size) = parse_args();

    let filtered_text = std::fs::read_to_string(&filename)
        .unwrap_or(filename) // assume that this is the string to parse, if can't load
        .chars()
        .filter(|c| security::is_russian_char(*c))
        .collect::<String>();

    for (el, n) in security::frequencies(&filtered_text, size) {
        println!("{} - {:.6}", el, n);
    }
}