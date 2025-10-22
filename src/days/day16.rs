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

    let mut lowest_score_map = HashMap::new();
    let mut lowest_score = None;

    // Part 1: Find the path with the lowest score. Once it is found, keep searching until all paths
    // with that score are discovered for Part 2.
    {
        let mut queue = BinaryHeap::new();

        fn push_state(
            queue: &mut BinaryHeap<StateWithScore>,
            lowest_score_map: &mut HashMap<State, usize>,
            state: State,
            score: usize,
        ) {
            if lowest_score_map.contains_key(&state) {
                return;
            }
            queue.push(state.with_score(score));
            lowest_score_map.insert(state, score);
        }

        push_state(
            &mut queue,
            &mut lowest_score_map,
            State {
                coord: start_coord,
                dir: Direction::East,
            },
            0,
        );

        while let Some(StateWithScore {
            state: State { coord, dir },
            score,
        }) = queue.pop()
        {
            if let Some(lowest_score) = lowest_score {
                if score > lowest_score {
                    break;
                }
            }

            let next_coord = dir
                .step(coord, 0, grid.width() - 1, 0, grid.height() - 1)
                .unwrap();
            if grid[next_coord] != b'#' {
                push_state(
                    &mut queue,
                    &mut lowest_score_map,
                    State {
                        coord: next_coord,
                        dir,
                    },
                    score + 1,
                );
            }

            if next_coord == end_coord {
                lowest_score = Some(score + 1);
                continue;
            }

            push_state(
                &mut queue,
                &mut lowest_score_map,
                State {
                    coord,
                    dir: dir.clockwise(),
                },
                score + 1000,
            );
            push_state(
                &mut queue,
                &mut lowest_score_map,
                State {
                    coord,
                    dir: dir.counter_clockwise(),
                },
                score + 1000,
            );
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

        while let Some(StateWithScore { state, score }) = queue.pop_front() {
            if lowest_score_map.get(&state) != Some(&score) {
                continue;
            }

            let State { coord, dir } = state;
            best_path_coords.insert(coord);

            if score >= 1 {
                queue.push_back(
                    State {
                        coord: dir
                            .opposite()
                            .step(coord, 0, grid.width() - 1, 0, grid.height() - 1)
                            .unwrap(),
                        dir,
                    }
                    .with_score(score - 1),
                );
            }

            if score >= 1000 {
                queue.push_back(
                    State {
                        coord,
                        dir: dir.clockwise(),
                    }
                    .with_score(score - 1000),
                );

                queue.push_back(
                    State {
                        coord,
                        dir: dir.counter_clockwise(),
                    }
                    .with_score(score - 1000),
                );
            }
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
