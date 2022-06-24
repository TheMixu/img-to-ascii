use ascii::{output_ascii, turn_to_ascii};
use parser::parse_args;

mod ascii;
mod cli;
mod parser;

fn main() {
    let options = cli::Options::new();
    let result = parse_args(&options);
    let ascii = match result {
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(0)
        }
        Ok((img, mode)) => turn_to_ascii(img, mode),
    };
    let res = output_ascii(ascii, &options);
    if let Err(e) = res {
        eprintln!("Error: {}", e);
        std::process::exit(0)
    }
}
