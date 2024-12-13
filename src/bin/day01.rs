use advent_of_code_2024::day_main;
use itertools::Itertools;
use std::cmp::Ordering;

day_main!(1);

fn solve(input: &str) -> (Option<usize>, Option<usize>) {
    let (mut left_list, mut right_list) = parse(input);

    left_list.sort();
    right_list.sort();

    // Part 1
    let output1 = left_list
        .iter()
        .zip(&right_list)
        .map(|(&l, &r)| l.abs_diff(r))
        .sum();

    // Part 2
    let mut left_it = left_list.into_iter().peekable();
    let mut right_it = right_list.into_iter().dedup_with_count().peekable();

    let mut output2 = 0;

    while let (Some(l), Some((r_count, r))) = (left_it.peek(), right_it.peek()) {
        match l.cmp(r) {
            Ordering::Less => {
                left_it.next();
            }
            Ordering::Equal => {
                output2 += l * r_count;
                left_it.next();
            }
            Ordering::Greater => {
                right_it.next();
            }
        }
    }

    (Some(output1), Some(output2))
}

fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut left_list = vec![];
    let mut right_list = vec![];

    input.lines().for_each(|line| {
        let mut it = line
            .split_whitespace()
            .take(2)
            .map(|x| x.parse::<usize>().unwrap());
        left_list.push(it.next().unwrap());
        right_list.push(it.next().unwrap());
    });

    (left_list, right_list)
}

#[cfg(test)]
mod tests {
    use crate::solve;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let example_input = indoc! {"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        "};
        assert_eq!(solve(example_input), (Some(11), Some(31)));
    }
}
