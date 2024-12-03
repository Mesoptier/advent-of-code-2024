use crate::days::day01::Day01;
use crate::days::day02::Day02;
use crate::days::day03::Day03;
use std::fmt::Debug;

mod day01;
mod day02;
mod day03;

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
        1 => Day01.run(input),
        2 => Day02.run(input),
        3 => Day03.run(input),
        _ => unimplemented!(),
    }
}
