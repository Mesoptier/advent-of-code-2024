use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::ops::RangeBounds;

pub const DAY: usize = 8;

pub fn solve(input: &str) -> (Option<usize>, Option<usize>) {
    let (antennas_map, x_range, y_range) = {
        let mut antennas_map = HashMap::<char, Vec<(isize, isize)>>::new();
        let mut width = 0;
        let mut height = 0;

        for (y, line) in input.lines().enumerate() {
            width = line.len();
            height += 1;

            for (x, c) in line.chars().enumerate() {
                if c == '.' {
                    continue;
                }

                antennas_map
                    .entry(c)
                    .or_default()
                    .push((x as isize, y as isize));
            }
        }
        (antennas_map, 0..width as isize, 0..height as isize)
    };

    let mut antinodes_1 = HashSet::<(isize, isize)>::new();
    let mut antinodes_2 = HashSet::<(isize, isize)>::new();
    for antennas in antennas_map.values() {
        antennas
            .iter()
            .tuple_combinations()
            .for_each(|(&(x0, y0), &(x1, y1))| {
                let dx = x0 - x1;
                let dy = y0 - y1;
                let d = gcd::euclid_usize(dx.unsigned_abs(), dy.unsigned_abs()) as isize;

                for (x, y, dx, dy) in [(x0, y0, dx, dy), (x1, y1, -dx, -dy)] {
                    // Part 1
                    let xi = x + dx;
                    let yi = y + dy;
                    if x_range.contains(&xi) && y_range.contains(&yi) {
                        antinodes_1.insert((xi, yi));
                    }

                    // Part 2
                    let dx = dx / d;
                    let dy = dy / d;
                    for i in 0.. {
                        let xi = x + dx * i;
                        let yi = y + dy * i;
                        if x_range.contains(&xi) && y_range.contains(&yi) {
                            antinodes_2.insert((xi, yi));
                        } else {
                            break;
                        }
                    }
                }
            })
    }

    (Some(antinodes_1.len()), Some(antinodes_2.len()))
}

#[cfg(test)]
mod tests {
    use super::solve;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let example_input = indoc! {"
            ............
            ........0...
            .....0......
            .......0....
            ....0.......
            ......A.....
            ............
            ............
            ........A...
            .........A..
            ............
            ............
        "};
        assert_eq!(solve(example_input), (Some(14), Some(34)));
    }
}
