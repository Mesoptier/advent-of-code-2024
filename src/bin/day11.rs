use advent_of_code_2024::day_main;
use itertools::Itertools;
use std::collections::HashMap;

day_main!(11);

fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let stones = input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec();

    let mut cache = HashMap::default();
    let count1 = stones
        .iter()
        .map(|&stone| blink(stone, 25, &mut cache))
        .sum();
    let count2 = stones
        .iter()
        .map(|&stone| blink(stone, 75, &mut cache))
        .sum();

    (Some(count1), Some(count2))
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct CacheKey(u64);

impl CacheKey {
    const STONE_BITS: u64 = 56;
    const TIMES_BITS: u64 = 8;

    fn new(stone: u64, times: u64) -> Self {
        assert!(stone < (1 << Self::STONE_BITS));
        assert!(times < (1 << Self::TIMES_BITS));

        Self((stone << Self::TIMES_BITS) | times)
    }

    fn times(&self) -> u64 {
        self.0 & ((1 << Self::TIMES_BITS) - 1)
    }
    fn stone(&self) -> u64 {
        self.0 >> Self::TIMES_BITS
    }
}

fn blink(stone: u64, times: u64, cache: &mut HashMap<CacheKey, u64>) -> u64 {
    if times == 0 {
        1
    } else {
        let cache_key = CacheKey::new(stone, times);
        if let Some(&result) = cache.get(&cache_key) {
            return result;
        }

        let result = match stone {
            0 => blink(1, times - 1, cache),
            n if (n.ilog10() + 1) % 2 == 0 => {
                let f = 10u64.pow((n.ilog10() + 1) / 2);
                blink(n / f, times - 1, cache) + blink(n % f, times - 1, cache)
            }
            n => blink(n * 2024, times - 1, cache),
        };
        cache.insert(cache_key, result);
        result
    }
}

fn blink_naive(stones: &Vec<u64>, next_stones: &mut Vec<u64>) {
    next_stones.clear();

    for &stone in stones {
        match stone {
            0 => next_stones.push(1),
            n if (n.ilog10() + 1) % 2 == 0 => {
                let f = 10u64.pow((n.ilog10() + 1) / 2);
                next_stones.push(n / f);
                next_stones.push(n % f);
            }
            n => next_stones.push(n * 2024),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{blink_naive, solve};

    #[test]
    fn test_solve() {
        let example_input = "125 17";
        assert_eq!(solve(example_input), (Some(55312), Some(65601038650482)));
    }

    #[test]
    fn test_blink() {
        let mut stones = vec![125, 17];
        let mut next_stones = vec![];

        blink_naive(&stones, &mut next_stones);
        (next_stones, stones) = (stones, next_stones);
        assert_eq!(stones, vec![253000, 1, 7]);

        blink_naive(&stones, &mut next_stones);
        (next_stones, stones) = (stones, next_stones);
        assert_eq!(stones, vec![253, 0, 2024, 14168]);

        blink_naive(&stones, &mut next_stones);
        (next_stones, stones) = (stones, next_stones);
        assert_eq!(stones, vec![512072, 1, 20, 24, 28676032]);

        blink_naive(&stones, &mut next_stones);
        (next_stones, stones) = (stones, next_stones);
        assert_eq!(stones, vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]);

        blink_naive(&stones, &mut next_stones);
        (next_stones, stones) = (stones, next_stones);
        assert_eq!(
            stones,
            vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]
        );

        blink_naive(&stones, &mut next_stones);
        (_, stones) = (stones, next_stones);
        assert_eq!(
            stones,
            vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2
            ]
        );
    }
}
