use advent_of_code_2024::day_main;
use std::cmp::Ordering;

day_main!(2);

fn solve(input: &str) -> (Option<usize>, Option<usize>) {
    let mut count1 = 0;
    let mut count2 = 0;

    let mut levels = vec![];
    for line in input.lines() {
        levels.clear();
        levels.extend(
            line.split_whitespace()
                .map(|level| level.parse::<u32>().unwrap()),
        );

        match check_safety_with_dampener(&levels) {
            Ok(false) => {
                // Safe without any removed levels.
                count1 += 1;
                count2 += 1;
            }
            Ok(true) => {
                // Safe with a removed level.
                count2 += 1;
            }
            _ => {
                // Unsafe.
            }
        }
    }

    (Some(count1), Some(count2))
}

fn check_safety_with_dampener(levels: &[u32]) -> Result<bool, ()> {
    match check_safety_with_dampener_ordered(levels, Ordering::Less) {
        Ok(false) => Ok(false),
        result => match (
            result,
            check_safety_with_dampener_ordered(levels, Ordering::Greater),
        ) {
            (_, Ok(false)) => Ok(false),
            (Ok(true), _) | (_, Ok(true)) => Ok(true),
            _ => Err(()),
        },
    }
}

fn check_safety_with_dampener_ordered(
    levels: &[u32],
    target_ordering: Ordering,
) -> Result<bool, ()> {
    match check_safety_ordered(levels, target_ordering) {
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
                if check_safety_ordered(&levels[index + 2..], target_ordering).is_ok() {
                    return Ok(true);
                }
            }

            Err(())
        }
    }
}

fn check_safety_ordered(levels: &[u32], target_ordering: Ordering) -> Result<(), usize> {
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

#[cfg(test)]
mod tests {
    use crate::solve;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let example_input = indoc! {"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        "};
        assert_eq!(solve(example_input), (Some(2), Some(4)));
    }
}
