use crate::days::DaySolution;
use itertools::Itertools;
use std::cmp::Ordering;

const MAX_REPORT_SIZE: usize = 8;

pub struct Day02;

impl DaySolution for Day02 {
    type Output1 = usize;
    type Output2 = usize;

    fn solve(&self, input: &str) -> (Option<Self::Output1>, Option<Self::Output2>) {
        let reports = input.lines().map(|line| {
            line.split_whitespace()
                .map(|level| level.parse().unwrap())
                .collect::<Vec<_>>()
        });

        let mut count1 = 0;
        let mut count2 = 0;

        for levels in reports {
            if Self::is_safe(levels.iter().copied()) {
                count1 += 1;
                count2 += 1;
            } else {
                for drop_index in 0..levels.len() {
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
                        count2 += 1;
                        break;
                    }
                }
            }
        }

        (Some(count1), Some(count2))
    }
}

impl Day02 {
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
