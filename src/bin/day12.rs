use advent_of_code_2024::day_main;
use advent_of_code_2024::util::grid::{Coord, Direction, Grid, StrGrid, VecGrid};
use std::collections::VecDeque;

day_main!(12);

fn solve(input: &str) -> (Option<u32>, Option<u32>) {
    let grid = StrGrid::new(input.as_bytes());

    let mut visited = VecGrid::from_data(grid.width(), vec![false; grid.width() * grid.height()]);
    let mut count1 = 0;

    for (coord, _) in grid.iter() {
        if visited[coord] {
            continue;
        }

        let (area, perimeter) = fill_region(&grid, coord, &mut visited);
        count1 += area * perimeter;
    }

    (Some(count1), None)
}

fn fill_region<'g>(
    grid: &'g impl Grid<'g, Item = &'g u8>,
    coord: Coord,
    visited: &mut VecGrid<bool>,
) -> (u32, u32) {
    let mut queue = VecDeque::new();
    queue.push_back(coord);
    visited[coord] = true;

    let mut area = 0;
    let mut perimeter = 0;

    let target_label = grid.get(coord).unwrap();

    while let Some(coord) = queue.pop_front() {
        area += 1;
        perimeter += 4;

        for next_coord in Direction::DIRECTIONS
            .into_iter()
            .filter_map(|dir| dir.step(coord, 0, grid.width() - 1, 0, grid.height() - 1))
        {
            if grid.get(next_coord).unwrap() != target_label {
                continue;
            }

            perimeter -= 1;

            if visited[next_coord] {
                continue;
            }

            visited[next_coord] = true;
            queue.push_back(next_coord);
        }
    }

    (area, perimeter)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_solve() {
        let example_input = "RRRRIICCFF\nRRRRIICCCF\nVVRRRCCFFF\nVVRCCCJFFF\nVVVVCJJCFE\nVVIVCCJJEE\nVVIIICJJEE\nMIIIIIJJEE\nMIIISIJEEE\nMMMISSJEEE\n";
        assert_eq!(solve(example_input), (Some(1930), None));
    }
}
