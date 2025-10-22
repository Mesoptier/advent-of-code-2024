use crate::util::grid::{Coord, Direction, Grid, StrGrid};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

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

#[derive(Eq, PartialEq, Copy, Clone)]
struct StateWithScore {
    state: State,
    score: usize,
}

impl StateWithScore {
    fn next_states(self) -> [Self; 3] {
        [
            (self.state.dir, 1),
            (self.state.dir.clockwise(), 1001),
            (self.state.dir.counter_clockwise(), 1001),
        ]
        .map(|(dir, score)| Self {
            state: State {
                coord: dir.unchecked_step(self.state.coord),
                dir,
            },
            score: self.score + score,
        })
    }

    fn prev_states(self) -> [Self; 3] {
        [
            (self.state.dir, 1),
            (self.state.dir.clockwise(), 1001),
            (self.state.dir.counter_clockwise(), 1001),
        ]
        .map(|(dir, score)| Self {
            state: State {
                coord: self.state.dir.opposite().unchecked_step(self.state.coord),
                dir,
            },
            score: self.score.wrapping_sub(score),
        })
    }
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

    let mut lowest_score_map = HashMap::new();
    let mut lowest_score = None;

    // Part 1: Find the path with the lowest score. Once it is found, keep searching until all paths
    // with that score are discovered for Part 2.
    {
        let mut queue = BinaryHeap::new();

        let start_state = StateWithScore {
            state: State {
                coord: start_coord,
                dir: Direction::East,
            },
            score: 0,
        };

        queue.push(start_state);
        lowest_score_map.insert(start_state.state, start_state.score);

        while let Some(state) = queue.pop() {
            if let Some(lowest_score) = lowest_score {
                if state.score > lowest_score {
                    break;
                }
            }

            for next_state in state.next_states() {
                if grid[next_state.state.coord] == b'#' {
                    // Don't crash into walls.
                    continue;
                }
                if lowest_score_map.contains_key(&next_state.state) {
                    // Already found a lower-score path to this state.
                    continue;
                }
                if next_state.state.coord == end_coord {
                    lowest_score = Some(next_state.score);
                }
                queue.push(next_state);
                lowest_score_map.insert(next_state.state, next_state.score);
            }
        }
    }

    // Part 2: backtrack from end coord (facing North or East), counting cells where backtrack_score + cache_score == lowest_score.
    let result_part2 = {
        let mut best_path_coords = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(
            State {
                coord: end_coord,
                dir: Direction::North,
            }
            .with_score(lowest_score.unwrap()),
        );
        queue.push_back(
            State {
                coord: end_coord,
                dir: Direction::East,
            }
            .with_score(lowest_score.unwrap()),
        );

        while let Some(state) = queue.pop_front() {
            if lowest_score_map.get(&state.state) != Some(&state.score) {
                continue;
            }

            queue.extend(state.prev_states());
            best_path_coords.insert(state.state.coord);
        }

        Some(best_path_coords.len())
    };

    (lowest_score, result_part2)
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
        assert_eq!(solve(example_input), (Some(7036), Some(45)));
    }
}
