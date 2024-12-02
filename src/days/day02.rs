use crate::days::DaySolution;
use itertools::Itertools;
use std::cmp::Ordering;

pub struct Day02;

impl DaySolution for Day02 {
    type Output1 = usize;
    type Output2 = usize;

    fn solve(&self, input: &str) -> (Option<Self::Output1>, Option<Self::Output2>) {
        let reports = Self::parse(input);

        // Part 1
        let output1 = reports
            .iter()
            .filter(|levels| Self::is_safe(levels.iter().copied()))
            .count();

        // Part 2
        let output2 = reports
            .iter()
            .filter(|levels| {
                for drop_index in 0..=levels.len() {
                    if Self::is_safe(
                        levels
                            .iter()
                            .enumerate()
                            .filter_map(|(index, level)| {
                                if index == drop_index {
                                    None
                                } else {
                                    Some(level)
                                }
                            })
                            .copied(),
                    ) {
                        return true;
                    }
                }

                false
            })
            .count();

        (Some(output1), Some(output2))
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

    fn is_safe(levels: impl Iterator<Item = u32> + Clone) -> bool {
        Self::is_safe_with_ordering(levels.clone(), Ordering::Less)
            || Self::is_safe_with_ordering(levels.clone(), Ordering::Greater)
    }

    fn is_safe_with_ordering(levels: impl Iterator<Item = u32>, target_ordering: Ordering) -> bool {
        levels.tuple_windows().peekable().all(|(a, b)| {
            a.cmp(&b) == target_ordering && (1 <= a.abs_diff(b) && a.abs_diff(b) <= 3)
        })
    }
}
