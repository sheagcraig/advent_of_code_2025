use anyhow::Result;
use clap::Parser;
use reqwest::{blocking::Client, header::COOKIE};

/// Download puzzle input for a day and save to that module.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Day number to get
    day: String,
}

fn main() -> Result<()> {
    let session = std::env::var("SESSION").expect("should have a session token set");
    let args = Args::parse();
    let client = Client::new();
    let puzzle_input = client
        .get(format!(
            "https://adventofcode.com/2025/day/{}/input",
            args.day
        ))
        .header(COOKIE, format!("session={session}"))
        .send()?
        .text()?;
    print!("{}", puzzle_input);
    Ok(())
}
