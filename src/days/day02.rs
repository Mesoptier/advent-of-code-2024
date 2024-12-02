use crate::days::DaySolution;
use itertools::Itertools;

pub struct Day02;

impl DaySolution for Day02 {
    type Output1 = usize;
    type Output2 = ();

    fn solve(&self, input: &str) -> (Option<Self::Output1>, Option<Self::Output2>) {
        let reports = Self::parse(input);

        // Part 1
        let output1 = reports
            .iter()
            .filter(|levels| {
                let mut windows = levels.iter().tuple_windows::<(_, _)>().peekable();

                let (first, second) = windows.peek().unwrap();
                let target_ordering = first.cmp(second);

                windows.all(|(a, b)| {
                    a.cmp(b) == target_ordering && (1 <= a.abs_diff(*b) && a.abs_diff(*b) <= 3)
                })
            })
            .count();

        (Some(output1), None)
    }
}

impl Day02 {
    fn parse(input: &str) -> Vec<Vec<u32>> {
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|level| level.parse().unwrap())
                    .collect()
            })
            .collect()
    }
}
