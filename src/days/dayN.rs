pub const DAY: usize = 0;

pub fn solve(input: &str) -> (Option<usize>, Option<usize>) {
    (None, None)
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_solve() {
        let example_input = "";
        assert_eq!(solve(example_input), (None, None));
    }
}
