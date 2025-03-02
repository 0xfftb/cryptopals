mod challenge_1;
mod challenge_2;
mod challenge_3;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("1") => challenge_1::run(),
        Some("2") => challenge_2::run(),
        Some("3") => challenge_3::run(),
        _ => print!("Challenge not found\n"),
    }
}
