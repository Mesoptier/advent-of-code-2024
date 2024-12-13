use divrem::DivRem;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::combinator::{map_res, opt};
use nom::sequence::{delimited, preceded, separated_pair};
use nom::IResult;

pub const DAY: usize = 13;

pub fn solve(input: &str) -> (Option<isize>, Option<isize>) {
    let mut input = input;

    let mut count1 = 0;
    let mut count2 = 0;

    while let Ok((next_input, mut claw_machine)) = parse_claw_machine(input) {
        input = next_input;

        // Part 1
        if let Some(cost) = solve_claw_machine(claw_machine) {
            count1 += cost;
        }

        // Part 2
        claw_machine.prize.0 += 10000000000000;
        claw_machine.prize.1 += 10000000000000;
        if let Some(cost) = solve_claw_machine(claw_machine) {
            count2 += cost;
        }
    }

    (Some(count1), Some(count2))
}

#[derive(Debug, Copy, Clone)]
struct ClawMachine {
    button_a: (isize, isize),
    button_b: (isize, isize),
    prize: (isize, isize),
}

fn parse_claw_machine(input: &str) -> IResult<&str, ClawMachine> {
    fn parse_xy_pair(input: &str) -> IResult<&str, (isize, isize)> {
        separated_pair(
            preceded(
                alt((tag("X+"), tag("X="))),
                map_res(digit1, |s: &str| s.parse()),
            ),
            tag(", "),
            preceded(
                alt((tag("Y+"), tag("Y="))),
                map_res(digit1, |s: &str| s.parse()),
            ),
        )(input)
    }

    let (input, button_a) = delimited(tag("Button A: "), parse_xy_pair, newline)(input)?;
    let (input, button_b) = delimited(tag("Button B: "), parse_xy_pair, newline)(input)?;
    let (input, prize) = delimited(tag("Prize: "), parse_xy_pair, newline)(input)?;
    let (input, _) = opt(newline)(input)?;
    Ok((
        input,
        ClawMachine {
            button_a,
            button_b,
            prize,
        },
    ))
}

fn solve_claw_machine(claw_machine: ClawMachine) -> Option<isize> {
    // Solve for a, b:
    // a0 * a + b0 * b = c0
    // a1 * a + b1 * b = c1

    // Solve for a in terms of b:
    // a0 * a + b0 * b = c0
    // a0 * a = c0 - b0 * b
    // a = (c0 - b0 * b) / a0
    // a = c0 / a0 - (b0 * b) / a0

    // Substitute for a:
    // a1 * a + b1 * b = c1
    // a1 * (c0 / a0 - (b0 * b) / a0) + b1 * b = c1
    // b = (a1 * c0 - a0 * c1) / (a1 * b0 - a0 * b1)    iff a1 * b0 != a0 * b1 && a0 != 0

    let (a0, a1) = claw_machine.button_a;
    let (b0, b1) = claw_machine.button_b;
    let (c0, c1) = claw_machine.prize;

    if a0 == 0 || a1 * b0 == a0 * b1 {
        // No solution.
        return None;
    }

    let (b, b_rem) = (a1 * c0 - a0 * c1).div_rem(a1 * b0 - a0 * b1);
    if b_rem != 0 {
        // No integer solution.
        return None;
    }

    let (a, a_rem) = (c0 - b0 * b).div_rem(a0);
    if a_rem != 0 {
        // No integer solution.
        return None;
    }

    Some(a * 3 + b)
}

#[cfg(test)]
mod tests {
    use super::solve;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let example_input = indoc! {"
            Button A: X+94, Y+34
            Button B: X+22, Y+67
            Prize: X=8400, Y=5400
            
            Button A: X+26, Y+66
            Button B: X+67, Y+21
            Prize: X=12748, Y=12176
            
            Button A: X+17, Y+86
            Button B: X+84, Y+37
            Prize: X=7870, Y=6450
            
            Button A: X+69, Y+23
            Button B: X+27, Y+71
            Prize: X=18641, Y=10279
        "};
        assert_eq!(solve(example_input), (Some(480), Some(875318608908)));
    }
}
