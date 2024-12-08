use advent_of_code_2024::day_main;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::IResult;

day_main!(7);

fn solve(input: &str) -> (Option<isize>, Option<isize>) {
    let mut count1 = 0;

    for line in input.lines() {
        let (_, (test_value, numbers)) = parse_line(line).unwrap();

        fn test(test_value: isize, numbers: &[isize], accumulator: isize) -> bool {
            if numbers.is_empty() {
                accumulator == test_value
            } else {
                test(test_value, &numbers[1..], accumulator + numbers[0])
                    || test(test_value, &numbers[1..], accumulator * numbers[0])
            }
        }

        if test(test_value, &numbers, 0) {
            count1 += test_value;
        }
    }

    (Some(count1), None)
}

fn parse_line(line: &str) -> IResult<&str, (isize, Vec<isize>)> {
    separated_pair(
        map(digit1, |x: &str| x.parse::<isize>().unwrap()),
        tag(": "),
        separated_list0(tag(" "), map(digit1, |x: &str| x.parse::<isize>().unwrap())),
    )(line)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_solve() {
        let input = "190: 10 19\n3267: 81 40 27\n83: 17 5\n156: 15 6\n7290: 6 8 6 15\n161011: 16 10 13\n192: 17 8 14\n21037: 9 7 18 13\n292: 11 6 16 20\n";
        assert_eq!(solve(input), (Some(3749), None));
    }
}
