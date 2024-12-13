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
    let mut count2 = 0;

    for line in input.lines() {
        let (_, (test_value, numbers)) = parse_line(line).unwrap();

        fn test(test_value: isize, numbers: &[isize], accumulator: isize) -> (bool, bool) {
            if numbers.is_empty() {
                let result = accumulator == test_value;
                (result, result)
            } else {
                let result_add = test(test_value, &numbers[1..], accumulator + numbers[0]);
                if result_add.0 {
                    return (true, true);
                }
                if result_add.1 {
                    return (false, true);
                }

                let result_mul = test(test_value, &numbers[1..], accumulator * numbers[0]);
                if result_mul.0 {
                    return (true, true);
                }
                if result_mul.1 {
                    return (false, true);
                }

                let result_concat =
                    test(test_value, &numbers[1..], concat(accumulator, numbers[0]));
                (false, result_concat.1)
            }
        }

        fn concat(a: isize, b: isize) -> isize {
            a * 10isize.pow(b.ilog10() + 1) + b
        }

        let (result1, result2) = test(test_value, &numbers, 0);
        if result1 {
            count1 += test_value;
        }
        if result2 {
            count2 += test_value;
        }
    }

    (Some(count1), Some(count2))
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
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let input = indoc! {"
            190: 10 19
            3267: 81 40 27
            83: 17 5
            156: 15 6
            7290: 6 8 6 15
            161011: 16 10 13
            192: 17 8 14
            21037: 9 7 18 13
            292: 11 6 16 20
        "};
        assert_eq!(solve(input), (Some(3749), Some(11387)));
    }
}
