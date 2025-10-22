use crate::util::grid::{Coord, Direction, Grid, VecGrid};

pub const DAY: usize = 15;

pub fn solve(input: &str) -> (Option<usize>, Option<usize>) {
    let (grid_input, moves_input) = {
        let mut split = input.splitn(2, "\n\n");
        (split.next().unwrap(), split.next().unwrap())
    };

    let grid_1 = {
        let width = grid_input.chars().position(|c| c == '\n').unwrap();
        let data = grid_input
            .chars()
            .filter(|c| matches!(c, '.' | '@' | 'O' | '#'))
            .collect();
        VecGrid::from_data(width, data)
    };

    let grid_2 = {
        let width = grid_1.width() * 2;
        let data = grid_1
            .iter()
            .flat_map(|(_, c)| match c {
                '#' => ['#', '#'],
                'O' => ['[', ']'],
                '.' => ['.', '.'],
                '@' => ['@', '.'],
                _ => unreachable!(),
            })
            .collect();
        VecGrid::from_data(width, data)
    };

    let moves = moves_input.chars().filter_map(|c| match c {
        '^' => Some(Direction::North),
        'v' => Some(Direction::South),
        '<' => Some(Direction::West),
        '>' => Some(Direction::East),
        _ => None,
    });

    (
        Some(solve_part(grid_1, moves.clone())),
        Some(solve_part(grid_2, moves.clone())),
    )
}

fn solve_part(mut grid: VecGrid<char>, moves: impl Iterator<Item = Direction>) -> usize {
    let mut robot_coord = grid
        .iter()
        .find_map(
            |(coord, c)| {
                if matches!(c, '@') {
                    Some(coord)
                } else {
                    None
                }
            },
        )
        .unwrap();

    for dir in moves {
        fn can_move(grid: &VecGrid<char>, coord: Coord, dir: Direction) -> bool {
            let target_coord = dir
                .step(coord, 0, grid.width() - 1, 0, grid.height() - 1)
                .unwrap();

            let is_horizontal = matches!(dir, Direction::East | Direction::West);

            match grid[target_coord] {
                '.' => true,
                '@' => unreachable!(),
                '#' => false,
                'O' => can_move(grid, target_coord, dir),
                '[' => {
                    can_move(grid, target_coord, dir)
                        && (is_horizontal
                            || can_move(grid, (target_coord.0 + 1, target_coord.1), dir))
                }
                ']' => {
                    can_move(grid, target_coord, dir)
                        && (is_horizontal
                            || can_move(grid, (target_coord.0 - 1, target_coord.1), dir))
                }
                _ => unreachable!(),
            }
        }

        fn do_move(grid: &mut VecGrid<char>, coord: Coord, dir: Direction) {
            let target_coord = dir
                .step(coord, 0, grid.width() - 1, 0, grid.height() - 1)
                .unwrap();

            let is_horizontal = matches!(dir, Direction::East | Direction::West);

            match grid[target_coord] {
                'O' => do_move(grid, target_coord, dir),
                '[' => {
                    do_move(grid, target_coord, dir);
                    if !is_horizontal {
                        do_move(grid, (target_coord.0 + 1, target_coord.1), dir);
                    }
                }
                ']' => {
                    if !is_horizontal {
                        do_move(grid, (target_coord.0 - 1, target_coord.1), dir);
                    }
                    do_move(grid, target_coord, dir);
                }
                _ => {}
            };

            grid[target_coord] = grid[coord];
            grid[coord] = '.'
        }

        if can_move(&grid, robot_coord, dir) {
            do_move(&mut grid, robot_coord, dir);
            robot_coord = dir
                .step(robot_coord, 0, grid.width() - 1, 0, grid.height() - 1)
                .unwrap()
        }
    }

    grid.iter()
        .filter_map(|(coord, c)| match c {
            'O' | '[' => Some(100 * coord.1 + coord.0),
            _ => None,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::solve;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let example_input = indoc! {"
            ##########
            #..O..O.O#
            #......O.#
            #.OO..O.O#
            #..O@..O.#
            #O#..O...#
            #O..O..O.#
            #.OO.O.OO#
            #....O...#
            ##########

            <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
            vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
            ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
            <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
            ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
            ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
            >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
            <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
            ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
            v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
        "};
        assert_eq!(solve(example_input), (Some(10092), Some(9021)));
    }
}
