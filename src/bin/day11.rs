use advent_of_code_2024::day_main;
use itertools::Itertools;

day_main!(11);

fn solve(input: &str) -> (Option<usize>, Option<usize>) {
    let mut stones = input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();
    let mut next_stones = vec![];

    for _ in 0..25 {
        blink(&stones, &mut next_stones);
        (stones, next_stones) = (next_stones, stones);
    }

    (Some(stones.len()), None)
}

fn blink(stones: &Vec<usize>, next_stones: &mut Vec<usize>) {
    next_stones.clear();

    for &stone in stones {
        match stone {
            0 => next_stones.push(1),
            n if (n.ilog10() + 1) % 2 == 0 => {
                let f = 10usize.pow((n.ilog10() + 1) / 2);
                next_stones.push(n / f);
                next_stones.push(n % f);
            }
            n => next_stones.push(n * 2024),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{blink, solve};

    #[test]
    fn test_solve() {
        let example_input = "125 17";
        assert_eq!(solve(example_input), (Some(55312), None));
    }

    #[test]
    fn test_blink() {
        let mut stones = vec![125, 17];
        let mut next_stones = vec![];

        blink(&stones, &mut next_stones);
        (next_stones, stones) = (stones, next_stones);
        assert_eq!(stones, vec![253000, 1, 7]);

        blink(&stones, &mut next_stones);
        (next_stones, stones) = (stones, next_stones);
        assert_eq!(stones, vec![253, 0, 2024, 14168]);

        blink(&stones, &mut next_stones);
        (next_stones, stones) = (stones, next_stones);
        assert_eq!(stones, vec![512072, 1, 20, 24, 28676032]);

        blink(&stones, &mut next_stones);
        (next_stones, stones) = (stones, next_stones);
        assert_eq!(stones, vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]);

        blink(&stones, &mut next_stones);
        (next_stones, stones) = (stones, next_stones);
        assert_eq!(
            stones,
            vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]
        );

        blink(&stones, &mut next_stones);
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
