use crate::days::DaySolution;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;

pub struct Day05;

impl DaySolution for Day05 {
    type Output1 = u32;
    type Output2 = u32;

    fn solve(&self, input: &str) -> (Option<Self::Output1>, Option<Self::Output2>) {
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
}

#[test]
fn test_day_05() {
    let example = "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47";
    assert_eq!(Day05.solve(example), (Some(143), None))
}

struct PartialOrdSort {}
