use advent_of_code_2024::day_main;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::ops::RangeBounds;

day_main!(8);

fn solve(input: &str) -> (Option<usize>, Option<usize>) {
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

    let count1 = {
        let mut antinodes = HashSet::<(isize, isize)>::new();
        for antennas in antennas_map.values() {
            antennas
                .iter()
                .tuple_combinations()
                .for_each(|(&(x0, y0), &(x1, y1))| {
                    let dx = x0 - x1;
                    let dy = y0 - y1;

                    antinodes.insert((x0 + dx, y0 + dy));
                    antinodes.insert((x1 - dx, y1 - dy));
                })
        }

        antinodes
            .iter()
            .filter(|(x, y)| x_range.contains(x) && y_range.contains(y))
            .count()
    };

    let count2 = {
        let mut antinodes = HashSet::<(isize, isize)>::new();
        for antennas in antennas_map.values() {
            antennas
                .iter()
                .tuple_combinations()
                .for_each(|(&(x0, y0), &(x1, y1))| {
                    let dx = x0 - x1;
                    let dy = y0 - y1;

                    let d = gcd::euclid_usize(dx.unsigned_abs(), dy.unsigned_abs()) as isize;
                    let dx = dx / d;
                    let dy = dy / d;

                    for (x, y, dx, dy) in [(x0, y0, dx, dy), (x1, y1, -dx, -dy)] {
                        for i in 0.. {
                            let xi = x + dx * i;
                            let yi = y + dy * i;
                            if x_range.contains(&xi) && y_range.contains(&yi) {
                                antinodes.insert((xi, yi));
                            } else {
                                break;
                            }
                        }
                    }
                })
        }

        antinodes.len()
    };

    (Some(count1), Some(count2))
}

#[cfg(test)]
mod tests {
    use crate::solve;
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
