use advent_of_code_2024::day_main;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;

day_main!(5);

fn solve(input: &str) -> (Option<u32>, Option<u32>) {
    let mut lines = input.lines();

    // Parse rules
    let mut rules = HashMap::<u32, Vec<u32>>::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let (page_a, page_b) = line
            .split('|')
            .map(|s| s.parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap();
        rules.entry(page_a).or_default().push(page_b);
    }

    let cmp = |a: u32, b: u32| -> Ordering {
        debug_assert_ne!(a, b);
        if rules.get(&a).map(|x| x.contains(&b)).unwrap_or(false) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    };

    let mut output1 = 0;
    let mut output2 = 0;

    for line in lines {
        let pages = line
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect_vec();

        if pages
            .iter()
            .tuple_combinations()
            .all(|(a, b)| cmp(*a, *b) == Ordering::Less)
        {
            output1 += pages[pages.len() / 2];
        } else {
            let mut pages = pages;
            pages.sort_by(|a, b| cmp(*a, *b));
            output2 += pages[pages.len() / 2];
        }
    }

    (Some(output1), Some(output2))
}

#[cfg(test)]
mod tests {
    use crate::solve;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let example = indoc! {"
            47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13
            
            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47
        "};
        assert_eq!(solve(example), (Some(143), Some(123)))
    }
}
