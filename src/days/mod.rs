use crate::days::day02::Day02;
use crate::days::day03::Day03;
use crate::days::day04::Day04;
use crate::days::day05::Day05;
use std::fmt::Debug;

mod day02;
mod day03;
mod day04;
mod day05;

trait DaySolution {
    type Output1: Debug;
    type Output2: Debug;

    fn solve(&self, input: &str) -> (Option<Self::Output1>, Option<Self::Output2>);

    fn run(&self, input: &str) -> (Option<String>, Option<String>) {
        let (output1, output2) = self.solve(input);
        (
            output1.map(|o| format!("{:?}", o)),
            output2.map(|o| format!("{:?}", o)),
        )
    }
}

pub fn run_day(input: &str, day: u32) -> (Option<String>, Option<String>) {
    match day {
        2 => Day02.run(input),
        3 => Day03.run(input),
        4 => Day04.run(input),
        5 => Day05.run(input),
        _ => unimplemented!(),
    }
}
