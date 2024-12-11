use advent_of_code_2024::day_main;
use advent_of_code_2024::util::grid::{Coord, Grid, RefGrid, StrGrid, VecGrid};
use std::collections::VecDeque;

day_main!(10);

fn solve(input: &str) -> (Option<usize>, Option<usize>) {
    let grid = StrGrid::new(input.as_bytes());
    let grid = grid.map(|&c| (c as char).to_digit(10).unwrap());

    let mut count1 = 0;

    for (coord, height) in grid.iter() {
        if height == 0 {
            // Trailhead -> search for trails.
            count1 += count_trails(&grid, coord);
        }
    }

    (Some(count1), None)
}

fn count_trails<'g, G>(grid: &G, trailhead_coord: Coord) -> usize
where
    G: Grid<Item<'g> = u32> + 'g,
{
    let mut visited = VecGrid::from_data(grid.width(), vec![false; grid.width() * grid.height()]);
    let mut queue = VecDeque::new();

    queue.push_back((trailhead_coord, 0));
    visited[trailhead_coord] = true;

    while let Some((coord, height)) = queue.pop_front() {}

    todo!()
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
