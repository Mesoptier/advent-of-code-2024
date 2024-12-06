use advent_of_code_2024::day_main;
use nom::InputIter;

day_main!(6);

fn solve(input: &str) -> (Option<usize>, Option<()>) {
    let mut input = input.as_bytes();
    let width = input.position(|b| b == b'\n').unwrap();
    let line_width = width + 1;
    let height = input.len() / line_width;

    let to_input_index = |x: usize, y: usize| x + y * line_width;

    // Find guard character (^)
    let guard_index = input.position(|b| b == b'^').unwrap();

    let mut guard_x = (guard_index % line_width);
    let mut guard_y = (guard_index / line_width);
    let mut guard_dir = Direction::North;

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
            _ => break, // Out of bounds
        };
        if input[next_x + next_y * line_width] == b'#' {
            guard_dir = guard_dir.turn_right();
        } else {
            guard_x = next_x;
            guard_y = next_y;
        }
    }

    (Some(count), None)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

    #[test]
    fn test_solve() {
        let example_input = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...\n";
        assert_eq!(solve(example_input), (Some(41) /* Heh, Sum 41! */, None));
    }
}
