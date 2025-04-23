use crate::util::grid::{Coord, Direction, Grid, VecGrid};

pub const DAY: usize = 15;

pub fn solve(input: &str) -> (Option<usize>, Option<usize>) {
    let (grid_input, moves_input) = {
        let mut split = input.splitn(2, "\n\n");
        (split.next().unwrap(), split.next().unwrap())
    };

    let mut grid = {
        let width = grid_input.chars().position(|c| c == '\n').unwrap();
        let data = grid_input
            .chars()
            .filter(|c| matches!(c, '.' | '@' | 'O' | '#'))
            .collect();
        VecGrid::from_data(width, data)
    };

    let mut robot_coord = grid
        .iter()
        .find_map(|(coord, tile)| {
            if matches!(tile, '@') {
                Some(coord)
            } else {
                None
            }
        })
        .unwrap();

    let moves = moves_input.chars().filter_map(|c| match c {
        '^' => Some(Direction::North),
        'v' => Some(Direction::South),
        '<' => Some(Direction::West),
        '>' => Some(Direction::East),
        _ => None,
    });

    for dir in moves {
        fn try_move(grid: &mut VecGrid<char>, coord: Coord, dir: Direction) -> Option<Coord> {
            let target_coord = dir
                .step(coord, 0, grid.width() - 1, 0, grid.height() - 1)
                .unwrap();

            let can_move = match grid[target_coord] {
                '.' => true,
                '@' => unreachable!(),
                '#' => false,
                'O' => try_move(grid, target_coord, dir).is_some(),
                _ => unreachable!(),
            };

            if can_move {
                (grid[target_coord], grid[coord]) = (grid[coord], grid[target_coord]);
                Some(target_coord)
            } else {
                None
            }
        }

        if let Some(next_robot_coord) = try_move(&mut grid, robot_coord, dir) {
            robot_coord = next_robot_coord;
        }
    }

    let result1 = grid
        .iter()
        .filter_map(|(coord, tile)| match tile {
            'O' => Some(100 * coord.1 + coord.0),
            _ => None,
        })
        .sum();

    (Some(result1), None)
}

#[cfg(test)]
mod tests {
    use super::solve;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let example_input = indoc! {"
            ########
            #..O.O.#
            ##@.O..#
            #...O..#
            #.#.O..#
            #...O..#
            #......#
            ########

            <^^>>>vv<v>>v<<
        "};
        assert_eq!(solve(example_input), (Some(2028), Some(9021)));
    }
}
