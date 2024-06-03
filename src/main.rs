mod util;


extern crate rosrust;

fn main() {
    // This is a sample code that can act either as a publisher or subscriber based on input argument.
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- [pub|sub]");
        return;
    }

    match args[1].as_str() {
        "pub" => util::publisher::run_publisher(),
        "sub" => util::subscriber::run_subscriber(),
        _ => eprintln!("Unknown argument: {}", args[1]),
    }
}