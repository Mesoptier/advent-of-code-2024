use clap::{Parser, Subcommand};
use homedir::my_home;
use std::fs;
use std::time::Instant;

const YEAR: usize = 2024;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Download { day: usize },
    Solve { day: usize },
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
            let input = fs::read_to_string(&input_path)
                .unwrap_or_else(|_| panic!("Input file not found: {}", input_path));
            let solve = advent_of_code_2024::days::solver(day)
                .unwrap_or_else(|| panic!("Day {}: not yet implemented", day));

            let start = Instant::now();
            let (part_1, part_2) = solve(&input);
            let elapsed = start.elapsed();

            println!("Day {}", day);
            match part_1 {
                Some(part_1) => println!("Part 1: {}", part_1),
                None => println!("Part 1: not yet implemented"),
            }
            match part_2 {
                Some(part_2) => println!("Part 2: {}", part_2),
                None => println!("Part 2: not yet implemented"),
            }

            println!("Elapsed: {:?}", elapsed);
        }
    }
}
