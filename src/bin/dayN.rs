use advent_of_code_2024::day_main;

day_main!(0);

fn solve(input: &str) -> (Option<usize>, Option<usize>) {
    (None, None)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_solve() {
        let example_input = "";
        assert_eq!(solve(example_input), (None, None));
    }
}
