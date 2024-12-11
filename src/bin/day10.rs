use advent_of_code_2024::day_main;
use advent_of_code_2024::util::grid::{Coord, Direction, Grid, StrGrid, VecGrid};
use std::collections::VecDeque;

day_main!(10);

fn solve(input: &str) -> (Option<usize>, Option<usize>) {
    let grid = StrGrid::new(input.as_bytes()).map(|&c| (c as char).to_digit(10).unwrap());

    let mut count1 = 0;

    for (coord, height) in grid.iter() {
        if height == 0 {
            // Trailhead -> search for trails.
            count1 += count_trails(&grid, coord);
        }
    }

    (Some(count1), None)
}

fn count_trails<'a>(grid: &'a impl Grid<'a, Item = u32>, trailhead_coord: Coord) -> usize {
    let mut visited = VecGrid::from_data(grid.width(), vec![false; grid.width() * grid.height()]);
    let mut queue = VecDeque::new();

    queue.push_back((trailhead_coord, 0));
    visited[trailhead_coord] = true;

    let mut count = 0;

    while let Some((coord, height)) = queue.pop_front() {
        if height == 9 {
            count += 1;
            continue;
        }

        for next_coord in Direction::DIRECTIONS
            .iter()
            .filter_map(|dir| dir.step(coord, 0, grid.width() - 1, 0, grid.height() - 1))
        {
            if visited[next_coord] {
                continue;
            }
            if grid.get(next_coord).unwrap() == height + 1 {
                queue.push_back((next_coord, height + 1));
                visited[next_coord] = true;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_solve() {
        let example_input =
            "89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732\n";
        assert_eq!(solve(example_input), (Some(36), None));
    }
}
