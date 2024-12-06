#[macro_export]
macro_rules! day_main {
    ($day:literal) => {
        fn main() {
            use std::fs;
            use std::time::Instant;

            let input_path = format!("inputs/day{:02}.txt", $day);
            let input = fs::read_to_string(input_path).unwrap();

            let start = Instant::now();
            let (part_1, part_2) = solve(&input);
            let elapsed = start.elapsed();

            println!("Day {}", $day);
            match part_1 {
                Some(part_1) => println!("Part 1: {:?}", part_1),
                None => println!("Part 1: not yet implemented"),
            }
            match part_2 {
                Some(part_2) => println!("Part 2: {:?}", part_2),
                None => println!("Part 2: not yet implemented"),
            }

            println!("Elapsed: {:?}", elapsed);
        }
    };
}
