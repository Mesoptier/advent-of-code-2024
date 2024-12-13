macro_rules! impl_solve {
    ($($m:ident,)*) => {
        pub fn solve(day: usize) {
            match day {
                $(day if day == $m::DAY => {
                    let input_path = format!("inputs/day{:02}.txt", day);
                    let input = std::fs::read_to_string(input_path).unwrap();

                    let start = std::time::Instant::now();
                    let (part_1, part_2) = $m::solve(&input);
                    let elapsed = start.elapsed();

                    println!("Day {}", day);
                    match part_1 {
                        Some(part_1) => println!("Part 1: {:?}", part_1),
                        None => println!("Part 1: not yet implemented"),
                    }
                    match part_2 {
                        Some(part_2) => println!("Part 2: {:?}", part_2),
                        None => println!("Part 2: not yet implemented"),
                    }

                    println!("Elapsed: {:?}", elapsed);
                })*,
                _ => unimplemented!(),
            }
        }
    };
}

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;

impl_solve! {
    day01,
    day02,
    day03,
    day04,
    day05,
    day06,
    day07,
    day08,
    day09,
    day10,
    day11,
    day12,
    day13,
}
