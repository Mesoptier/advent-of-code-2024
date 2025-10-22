use crate::util::grid::{Coord, Direction, Grid, StrGrid};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

pub const DAY: usize = 16;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct State {
    coord: Coord,
    dir: Direction,
}

impl State {
    fn with_score(self, score: usize) -> StateWithScore {
        StateWithScore { state: self, score }
    }
}

#[derive(Eq, PartialEq)]
struct StateWithScore {
    state: State,
    score: usize,
}

impl PartialOrd<Self> for StateWithScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StateWithScore {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

pub fn solve(input: &str) -> (Option<usize>, Option<usize>) {
    let grid = StrGrid::new(input.as_bytes());
    let start_coord = (1, grid.height() - 2);
    let end_coord = (grid.width() - 2, 1);

    debug_assert_eq!(grid[start_coord], b'S');
    debug_assert_eq!(grid[end_coord], b'E');

    let start_state = State {
        coord: start_coord,
        dir: Direction::East,
    };

    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    fn push_state(
        queue: &mut BinaryHeap<StateWithScore>,
        visited: &mut HashSet<State>,
        state: State,
        score: usize,
    ) {
        if visited.insert(state) {
            queue.push(state.with_score(score));
        }
    }

    push_state(&mut queue, &mut visited, start_state, 0);

    let mut result = None;
    while let Some(StateWithScore {
        state: State { coord, dir },
        score,
    }) = queue.pop()
    {
        let target_coord = dir
            .step(coord, 0, grid.width() - 1, 0, grid.height() - 1)
            .unwrap();
        if target_coord == end_coord {
            result = Some(score + 1);
            break;
        }
        if grid[target_coord] != b'#' {
            push_state(
                &mut queue,
                &mut visited,
                State {
                    coord: target_coord,
                    dir,
                },
                score + 1,
            );
        }

        push_state(
            &mut queue,
            &mut visited,
            State {
                coord,
                dir: dir.clockwise(),
            },
            score + 1000,
        );
        push_state(
            &mut queue,
            &mut visited,
            State {
                coord,
                dir: dir.counter_clockwise(),
            },
            score + 1000,
        );
    }

    (result, None)
}

#[cfg(test)]
mod tests {
    use super::solve;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let example_input = indoc! {"
            ###############
            #.......#....E#
            #.#.###.#.###.#
            #.....#.#...#.#
            #.###.#####.#.#
            #.#.#.......#.#
            #.#.#####.###.#
            #...........#.#
            ###.#.#####.#.#
            #...#.....#.#.#
            #.#.#.###.#.#.#
            #.....#...#.#.#
            #.###.#.#.#.#.#
            #S..#.....#...#
            ###############
        "};
        assert_eq!(solve(example_input), (Some(7036), None));
    }
}
