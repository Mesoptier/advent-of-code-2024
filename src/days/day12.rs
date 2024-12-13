use crate::util::grid::{Coord, Grid, StrGrid, VecGrid};
use std::collections::VecDeque;

pub const DAY: usize = 12;

pub fn solve(input: &str) -> (Option<u32>, Option<u32>) {
    let grid = StrGrid::new(input.as_bytes());

    let mut visited = VecGrid::from_data(grid.width(), vec![false; grid.width() * grid.height()]);
    let mut count1 = 0;
    let mut count2 = 0;

    for (coord, _) in grid.iter() {
        if visited[coord] {
            continue;
        }

        let (area, perimeter, corners) = fill_region(&grid, coord, &mut visited);
        count1 += area * perimeter;
        count2 += area * corners;
    }

    (Some(count1), Some(count2))
}

fn fill_region<'g>(
    grid: &'g impl Grid<'g, Item = &'g u8>,
    coord: Coord,
    visited: &mut VecGrid<bool>,
) -> (u32, u32, u32) {
    let mut queue = VecDeque::new();
    queue.push_back(coord);
    visited[coord] = true;

    let mut area = 0;
    let mut perimeter = 0;
    let mut corners = 0;

    let target_label = grid.get(coord).unwrap();
    let is_target_label = |coord: Coord| -> bool { grid.get(coord) == Some(target_label) };

    while let Some(coord) = queue.pop_front() {
        let (x, y) = coord;
        let min_x = 0;
        let max_x = grid.width() - 1;
        let min_y = 0;
        let max_y = grid.height() - 1;

        // Nearby cells
        let nc = [
            min_x < x && min_y < y && is_target_label((x - 1, y - 1)),
            min_y < y && is_target_label((x, y - 1)),
            x < max_x && min_y < y && is_target_label((x + 1, y - 1)),
            min_x < x && is_target_label((x - 1, y)),
            true,
            x < max_x && is_target_label((x + 1, y)),
            min_x < x && y < max_y && is_target_label((x - 1, y + 1)),
            y < max_y && is_target_label((x, y + 1)),
            x < max_x && y < max_y && is_target_label((x + 1, y + 1)),
        ];

        // Count area
        area += 1;

        // Count perimeter + queue next coords
        for (i, next_coord) in [
            (1, (x, y.wrapping_sub(1))),
            (3, (x.wrapping_sub(1), y)),
            (5, (x + 1, y)),
            (7, (x, y + 1)),
        ] {
            if !nc[i] {
                perimeter += 1;
            } else if !visited[next_coord] {
                visited[next_coord] = true;
                queue.push_back(next_coord);
            }
        }

        // Count corners (inner || outer)
        for [tl, t, l] in [[0, 1, 3], [2, 5, 1], [8, 7, 5], [6, 3, 7]] {
            // Inner || Outer corner:
            // .#?      ?.?
            // ##?      .#?
            // ???      ???
            if !nc[tl] && nc[t] && nc[l] || !nc[t] && !nc[l] {
                corners += 1;
            }
        }
    }

    (area, perimeter, corners)
}

#[cfg(test)]
mod tests {
    use super::solve;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let example_input = indoc! {"
            RRRRIICCFF
            RRRRIICCCF
            VVRRRCCFFF
            VVRCCCJFFF
            VVVVCJJCFE
            VVIVCCJJEE
            VVIIICJJEE
            MIIIIIJJEE
            MIIISIJEEE
            MMMISSJEEE
        "};
        assert_eq!(solve(example_input), (Some(1930), Some(1206)));
    }
}
