mod challenge_1;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("1") => challenge_1::run(),
        _ => print!("Challenge not found\n"),
    }
}
