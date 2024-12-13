use advent_of_code_2024::day_main;
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::digit1;
use nom::combinator::{map, map_res, value};
use nom::multi::{fold_many0, many_till};
use nom::sequence::{delimited, separated_pair};
use nom::IResult;

day_main!(3);

fn solve(input: &str) -> (Option<u32>, Option<u32>) {
    struct State {
        sum1: u32,
        sum2: u32,
        enabled: bool,
    }

    let result: IResult<_, _> = fold_many0(
        many_till(value((), take(1usize)), parse_instruction),
        || State {
            sum1: 0,
            sum2: 0,
            enabled: true,
        },
        |state, (_, instruction)| match instruction {
            Instruction::Mul(x, y) => State {
                sum1: state.sum1 + x * y,
                sum2: state.sum2 + if state.enabled { x * y } else { 0 },
                ..state
            },
            Instruction::Enable => State {
                enabled: true,
                ..state
            },
            Instruction::Disable => State {
                enabled: false,
                ..state
            },
        },
    )(input);
    let (_, result_state) = result.unwrap();

    (Some(result_state.sum1), Some(result_state.sum2))
}

#[derive(Clone, Copy)]
enum Instruction {
    Mul(u32, u32),
    Enable,
    Disable,
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        map(
            delimited(
                tag("mul("),
                separated_pair(
                    map_res(digit1, |s: &str| s.parse::<u32>()),
                    tag(","),
                    map_res(digit1, |s: &str| s.parse::<u32>()),
                ),
                tag(")"),
            ),
            |(x, y)| Instruction::Mul(x, y),
        ),
        value(Instruction::Enable, tag("do()")),
        value(Instruction::Disable, tag("don't()")),
    ))(input)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_solve() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(solve(input), (Some(161), Some(161)));

        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(solve(input), (Some(161), Some(48)));
    }
}
