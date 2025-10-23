use nom::InputIter;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::ops::Index;

pub const DAY: usize = 16;

struct Grid<'a> {
    data: &'a [u8],
    width: usize,
}

impl<'a> Grid<'a> {
    fn new(input: &'a str) -> Self {
        let data = input.as_bytes();
        Self {
            width: data.position(|c| c == b'\n').unwrap() + 1,
            data,
        }
    }

    fn start_coord(&self) -> Coord {
        Coord(self.data.len() - self.width * 2 + 1)
    }
    fn end_coord(&self) -> Coord {
        Coord(self.width * 2 - 3)
    }

    fn step(&self, coord: Coord, direction: Direction) -> Coord {
        match direction {
            Direction::NORTH => Coord(coord.0 - self.width),
            Direction::EAST => Coord(coord.0 + 1),
            Direction::SOUTH => Coord(coord.0 + self.width),
            Direction::WEST => Coord(coord.0 - 1),
            _ => unreachable!(),
        }
    }
}

impl Index<Coord> for Grid<'_> {
    type Output = u8;

    fn index(&self, index: Coord) -> &Self::Output {
        self.data.index(index.0)
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Coord(usize);

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Direction(usize);

impl Direction {
    const NORTH: Self = Self(0);
    const EAST: Self = Self(1);
    const SOUTH: Self = Self(2);
    const WEST: Self = Self(3);

    fn clockwise(self) -> Self {
        Self((self.0 + 1) % 4)
    }
    fn counter_clockwise(self) -> Self {
        Self((self.0 + 3) % 4)
    }
    fn opposite(self) -> Self {
        Self((self.0 + 2) % 4)
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct State(usize);

impl State {
    fn new(coord: Coord, dir: Direction) -> Self {
        Self((coord.0 << 2) + dir.0)
    }

    fn coord(self) -> Coord {
        Coord(self.0 >> 2)
    }
    fn dir(self) -> Direction {
        Direction(self.0 % 4)
    }

    fn with_score(self, score: usize) -> StateWithScore {
        StateWithScore { state: self, score }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
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
    let grid = Grid::new(input);

    let start_coord = grid.start_coord();
    let end_coord = grid.end_coord();

    debug_assert_eq!(grid[start_coord], b'S');
    debug_assert_eq!(grid[end_coord], b'E');

    let mut lowest_score_map = HashMap::new();
    let mut lowest_score = None;

    // Part 1: Find the path with the lowest score. Once it is found, keep searching until all paths
    // with that score are discovered for Part 2.
    {
        let mut queue = BinaryHeap::new();

        let start_state = State::new(start_coord, Direction::EAST).with_score(0);

        queue.push(start_state);
        lowest_score_map.insert(start_state.state, start_state.score);

        while let Some(StateWithScore { state, score }) = queue.pop() {
            if let Some(lowest_score) = lowest_score {
                if score > lowest_score {
                    break;
                }
            }

            let next_states = [
                (state.dir(), 1),
                (state.dir().clockwise(), 1001),
                (state.dir().counter_clockwise(), 1001),
            ]
            .map(|(dir, delta_score)| {
                State::new(grid.step(state.coord(), dir), dir).with_score(score + delta_score)
            });

            for next_state in next_states {
                if grid[next_state.state.coord()] == b'#' {
                    // Don't crash into walls.
                    continue;
                }
                if lowest_score_map.contains_key(&next_state.state) {
                    // Already found a lower-score path to this state.
                    continue;
                }
                if next_state.state.coord() == end_coord {
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

        queue.push_back(State::new(end_coord, Direction::NORTH).with_score(lowest_score.unwrap()));
        queue.push_back(State::new(end_coord, Direction::EAST).with_score(lowest_score.unwrap()));

        while let Some(state) = queue.pop_front() {
            if lowest_score_map.get(&state.state) != Some(&state.score) {
                continue;
            }

            let prev_states = [
                (state.state.dir(), 1),
                (state.state.dir().clockwise(), 1001),
                (state.state.dir().counter_clockwise(), 1001),
            ]
            .map(|(dir, score)| StateWithScore {
                state: State::new(
                    grid.step(state.state.coord(), state.state.dir().opposite()),
                    dir,
                ),
                score: state.score.wrapping_sub(score),
            });

            queue.extend(prev_states);
            best_path_coords.insert(state.state.coord());
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
