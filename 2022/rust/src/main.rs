use std::env;

fn main() {
    let mut args = env::args().skip(1);
    let day = args.next();
    let example = args.any(|arg| arg == "--example" || arg == "-e" || arg == "example");

    match day.as_deref() {
        Some("01") => days::d01::run(example),
        Some("02") => days::d02::run(example),
        _ => {
            eprintln!("Usage: cargo run -- <day> [--example]");
            eprintln!("Example: cargo run -- 01 --example");
        }
    }
}

mod days;
