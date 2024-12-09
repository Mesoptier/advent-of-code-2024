use advent_of_code_2024::day_main;
use itertools::Itertools;

day_main!(9);

struct Blocks {
    checksum: usize,
    position: usize,
}

impl Blocks {
    fn push(&mut self, block: usize) {
        self.checksum += self.position * block;
        self.position += 1;
    }
}

fn solve(input: &str) -> (Option<usize>, Option<usize>) {
    let input = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect_vec();

    let checksum_1 = {
        let mut back_index = input.len() - 1;
        let mut back_blocks = input[back_index];

        let mut blocks = Blocks {
            checksum: 0,
            position: 0,
        };
        for (index, &num_blocks) in input.iter().enumerate() {
            if back_index <= index {
                break;
            }

            if index % 2 == 0 {
                // Fill front
                let block_id = index / 2;
                for _ in 0..num_blocks {
                    blocks.push(block_id);
                }
            } else {
                // Fill back
                for _ in 0..num_blocks {
                    while back_blocks == 0 {
                        back_index -= 2;
                        back_blocks = input[back_index];
                    }

                    let block_id = back_index / 2;
                    back_blocks -= 1;
                    blocks.push(block_id);
                }
            }
        }

        // Remaining blocks
        while back_blocks > 0 {
            let block_id = back_index / 2;
            back_blocks -= 1;
            blocks.push(block_id);
        }

        blocks.checksum
    };

    (Some(checksum_1), None)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_solve() {
        let example_input = "2333133121414131402";
        assert_eq!(solve(example_input), (Some(1928), None));
    }
}
