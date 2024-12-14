use bmp::{px, Image, Pixel};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map_res, opt, recognize};
use nom::sequence::{preceded, separated_pair, tuple};
use nom::IResult;
use std::cmp::Ordering;

pub const DAY: usize = 14;

pub fn solve(input: &str) -> (Option<usize>, Option<usize>) {
    solve_with_dimensions(input, [101, 103])
}

fn solve_with_dimensions(input: &str, dimensions: Vec2) -> (Option<usize>, Option<usize>) {
    let particles = input
        .lines()
        .map(|line| parse_particle(line).unwrap().1)
        .collect_vec();

    let get_quadrant = |particle: Particle| -> Option<usize> {
        match (
            particle.pos[0].cmp(&(dimensions[0] / 2)),
            particle.pos[1].cmp(&(dimensions[1] / 2)),
        ) {
            (Ordering::Less, Ordering::Less) => Some(0),
            (Ordering::Greater, Ordering::Less) => Some(1),
            (Ordering::Less, Ordering::Greater) => Some(2),
            (Ordering::Greater, Ordering::Greater) => Some(3),
            _ => None,
        }
    };

    // Part 1
    let mut counts1 = [0; 4];
    for mut particle in particles.iter().copied() {
        for d in 0..2 {
            particle.pos[d] = (particle.pos[d] + particle.vel[d] * 100).rem_euclid(dimensions[d]);
        }

        if let Some(quadrant) = get_quadrant(particle) {
            counts1[quadrant] += 1;
        }
    }

    // Part 2
    for time in 0..10000 {
        let mut img = Image::new(dimensions[0] as u32, dimensions[1] as u32);

        for Particle { mut pos, vel } in particles.iter().copied() {
            for d in 0..2 {
                pos[d] = (pos[d] + vel[d] * time).rem_euclid(dimensions[d]);
            }

            img.set_pixel(pos[0] as u32, pos[1] as u32, px!(255, 255, 255));
        }

        let img_path = format!("data/day14/{}.bmp", time);
        img.save(img_path).unwrap();
    }

    (Some(counts1.iter().product()), None)
}

type Vec2 = [isize; 2];

#[derive(Debug, Copy, Clone)]
struct Particle {
    pos: Vec2,
    vel: Vec2,
}

fn parse_particle(input: &str) -> IResult<&str, Particle> {
    let (input, pos) = preceded(tag("p="), parse_vec2)(input)?;
    let (input, vel) = preceded(tag(" v="), parse_vec2)(input)?;

    Ok((input, Particle { pos, vel }))
}

fn parse_vec2(input: &str) -> IResult<&str, Vec2> {
    let (input, (x, y)) = separated_pair(parse_isize, tag(","), parse_isize)(input)?;
    Ok((input, [x, y]))
}

fn parse_isize(input: &str) -> IResult<&str, isize> {
    map_res(recognize(tuple((opt(tag("-")), digit1))), |s: &str| {
        s.parse::<isize>()
    })(input)
}

#[cfg(test)]
mod tests {
    use super::solve_with_dimensions;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let example_input = indoc! {"
            p=0,4 v=3,-3
            p=6,3 v=-1,-3
            p=10,3 v=-1,2
            p=2,0 v=2,-1
            p=0,0 v=1,3
            p=3,0 v=-2,-2
            p=7,6 v=-1,-3
            p=3,0 v=-1,-2
            p=9,3 v=2,3
            p=7,3 v=-1,2
            p=2,4 v=2,-3
            p=9,5 v=-3,-3
        "};
        assert_eq!(
            solve_with_dimensions(example_input, [11, 7]),
            (Some(12), None)
        );
    }
}
