use crate::days::DaySolution;
use nom::bytes::complete::{tag, take};
use nom::character::complete::digit1;
use nom::combinator::{map_res, value};
use nom::multi::{fold_many0, many_till};
use nom::sequence::{delimited, separated_pair};
use nom::IResult;

pub struct Day03;

impl DaySolution for Day03 {
    type Output1 = u32;
    type Output2 = ();

    fn solve(&self, input: &str) -> (Option<Self::Output1>, Option<Self::Output2>) {
        let result: IResult<_, _> = fold_many0(
            many_till(
                value((), take(1usize)),
                delimited(
                    tag("mul("),
                    separated_pair(
                        map_res(digit1, |s: &str| s.parse::<u32>()),
                        tag(","),
                        map_res(digit1, |s: &str| s.parse::<u32>()),
                    ),
                    tag(")"),
                ),
            ),
            || 0,
            |sum, (_, (a, b))| sum + a * b,
        )(input);
        let (_, output1) = result.unwrap();

        (Some(output1), None)
    }
}

#[test]
fn test_day3() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    println!("{:?}", Day03.solve(input));
}
