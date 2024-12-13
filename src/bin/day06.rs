use advent_of_code_2024::day_main;
use nom::InputIter;
use std::collections::HashSet;

day_main!(6);

fn solve(input: &str) -> (Option<usize>, Option<usize>) {
    let input = input.as_bytes();
    let width = input.position(|b| b == b'\n').unwrap();
    let line_width = width + 1;
    let height = input.len() / line_width;

    // Find guard character (^)
    let guard_index = input.position(|b| b == b'^').unwrap();
    let init_guard_x = guard_index % line_width;
    let init_guard_y = guard_index / line_width;
    let init_guard_dir = Direction::North;

    // Part 1
    let count1 = {
        let mut guard_x = init_guard_x;
        let mut guard_y = init_guard_y;
        let mut guard_dir = init_guard_dir;

        let mut visited = vec![false; width * height];
        let mut count = 0;

        loop {
            let visited_index = guard_x + guard_y * width;
            if !visited[visited_index] {
                visited[visited_index] = true;
                count += 1;
            }

            let (next_x, next_y) = match guard_dir {
                Direction::North if guard_y > 0 => (guard_x, guard_y - 1),
                Direction::East if guard_x < width - 1 => (guard_x + 1, guard_y),
                Direction::South if guard_y < height - 1 => (guard_x, guard_y + 1),
                Direction::West if guard_x > 0 => (guard_x - 1, guard_y),
                _ => break, // Out of bounds.
            };
            if input[next_x + next_y * line_width] == b'#' {
                // Obstructed.
                guard_dir = guard_dir.turn_right();
            } else {
                guard_x = next_x;
                guard_y = next_y;
            }
        }

        count
    };

    // Part 2
    let count2 = {
        let mut count = 0;

        for obstruction_y in 0..height {
            for obstruction_x in 0..width {
                if obstruction_x == init_guard_x && obstruction_y == init_guard_y {
                    // Can't place obstruction at guard's starting position.
                    continue;
                }
                if input[obstruction_x + obstruction_y * line_width] == b'#' {
                    // Already obstructed.
                    continue;
                }

                let mut guard_x = init_guard_x;
                let mut guard_y = init_guard_y;
                let mut guard_dir = init_guard_dir;

                let mut visited = HashSet::<(usize, usize, Direction)>::new();

                loop {
                    if !visited.insert((guard_x, guard_y, guard_dir)) {
                        // Loop found.
                        count += 1;
                        break;
                    }

                    let (next_x, next_y) = match guard_dir {
                        Direction::North if guard_y > 0 => (guard_x, guard_y - 1),
                        Direction::East if guard_x < width - 1 => (guard_x + 1, guard_y),
                        Direction::South if guard_y < height - 1 => (guard_x, guard_y + 1),
                        Direction::West if guard_x > 0 => (guard_x - 1, guard_y),
                        _ => break, // Out of bounds.
                    };
                    if input[next_x + next_y * line_width] == b'#'
                        || next_x == obstruction_x && next_y == obstruction_y
                    {
                        // Obstructed.
                        guard_dir = guard_dir.turn_right();
                    } else {
                        guard_x = next_x;
                        guard_y = next_y;
                    }
                }
            }
        }

        count
    };

    (Some(count1), Some(count2))
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let example_input = indoc! {"
            ....#.....
            .........#
            ..........
            ..#.......
            .......#..
            ..........
            .#..^.....
            ........#.
            #.........
            ......#...
        "};
        assert_eq!(solve(example_input), (Some(41) /* Heh, Sum 41! */, Some(6)));
    }
}
