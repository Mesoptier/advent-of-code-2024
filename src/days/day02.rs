use crate::days::DaySolution;
use std::cmp::Ordering;

pub struct Day02;

impl DaySolution for Day02 {
    type Output1 = usize;
    type Output2 = usize;

    fn solve(&self, input: &str) -> (Option<Self::Output1>, Option<Self::Output2>) {
        let mut count1 = 0;
        let mut count2 = 0;

        let mut levels = vec![];
        for line in input.lines() {
            levels.clear();
            levels.extend(
                line.split_whitespace()
                    .map(|level| level.parse::<u32>().unwrap()),
            );

            match (
                check_safety_with_dampener(&levels, Ordering::Less),
                check_safety_with_dampener(&levels, Ordering::Greater),
            ) {
                (Ok(false), _) | (_, Ok(false)) => {
                    count1 += 1;
                    count2 += 1;
                }
                (Ok(true), _) | (_, Ok(true)) => {
                    count2 += 1;
                }
                _ => {}
            }
        }

        (Some(count1), Some(count2))
    }
}

fn check_safety_with_dampener(levels: &[u32], target_ordering: Ordering) -> Result<bool, ()> {
    match check_safety(levels, target_ordering) {
        Ok(_) => Ok(false),
        Err(index) => {
            // The step from `index` to `index + 1` failed. Either one of those could be removed by
            // the Problem Dampener. So we try to recover locally until `index + 2` and resume normal
            // checking from there. Any further problems are then fatal.

            #[allow(clippy::collapsible_if)]
            #[allow(unused_parens)]
            if (
                // Whether level at `index` can be removed.
                ((index == 0 || is_safe_step(levels[index - 1], levels[index + 1], target_ordering))
                && (index + 2 == levels.len()
                    || is_safe_step(levels[index + 1], levels[index + 2], target_ordering)))
                // Whether level at `index + 1` can be removed.
                || (index + 2 == levels.len()
                    || is_safe_step(levels[index], levels[index + 2], target_ordering))
            ) {
                if check_safety(&levels[index + 2..], target_ordering).is_ok() {
                    return Ok(true);
                }
            }

            Err(())
        }
    }
}

fn check_safety(levels: &[u32], target_ordering: Ordering) -> Result<(), usize> {
    for index in 1..levels.len() {
        if !is_safe_step(levels[index - 1], levels[index], target_ordering) {
            return Err(index - 1);
        }
    }

    Ok(())
}

fn is_safe_step(level: u32, next_level: u32, target_ordering: Ordering) -> bool {
    level.cmp(&next_level) == target_ordering && (0..=3).contains(&level.abs_diff(next_level))
}
