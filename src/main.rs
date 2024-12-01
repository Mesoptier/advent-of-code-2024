use crate::days::run_day;
use clap::{Parser, Subcommand};
use homedir::my_home;
use std::fs;

mod days;

const YEAR: u32 = 2024;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Download { day: u32 },
    Solve { day: u32 },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Download { day } => {
            let input_url = format!("https://adventofcode.com/{}/day/{}/input", YEAR, day);

            let session_path = {
                let mut path = my_home().unwrap().unwrap();
                path.push(".adventofcode.session");
                path
            };
            let session = fs::read_to_string(session_path).unwrap().trim().to_string();

            let client = reqwest::blocking::Client::new();
            let response = client
                .get(input_url)
                .header("cookie", format!("session={}", session))
                .send()
                .unwrap();

            if response.status() == reqwest::StatusCode::OK {
                let input_path = format!("inputs/day{:02}.txt", day);
                let input = response.text().unwrap();
                fs::write(&input_path, input).unwrap();
            } else {
                println!("couldn't fetch adventofcode input");
            }
        }
        Commands::Solve { day } => {
            let input_path = format!("inputs/day{:02}.txt", day);
            let input = fs::read_to_string(input_path).unwrap();

            let (output1, output2) = run_day(&input, day);
            println!("Day {}", day);
            println!(
                "Part 1: {}",
                output1.unwrap_or("not yet implemented".to_owned())
            );
            println!(
                "Part 2: {}",
                output2.unwrap_or("not yet implemented".to_owned())
            );
        }
    }
}
