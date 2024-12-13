use itertools::Itertools;

pub const DAY: usize = 9;

struct Blocks {
    checksum: usize,
    position: usize,
}

impl Blocks {
    fn push(&mut self, block: usize) {
        self.checksum += self.position * block;
        self.position += 1;
    }

    fn push_file(&mut self, file_id: usize, size: usize) {
        for _ in 0..size {
            self.push(file_id);
        }
    }

    fn skip_free(&mut self, size: usize) {
        self.position += size;
    }
}

pub fn solve(input: &str) -> (Option<usize>, Option<usize>) {
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

    let checksum_2 = {
        let num_files = (input.len() + 1) / 2;

        // Files marked as moved, so they'll be skipped for regular placement.
        let mut moved_files = vec![false; num_files];
        // List of files not (yet) moved, these will be considered for filling free space.
        let mut unmoved_files = Vec::from_iter(0..num_files);

        let mut blocks = Blocks {
            checksum: 0,
            position: 0,
        };

        for (index, &size) in input.iter().enumerate() {
            if index % 2 == 0 {
                // Fill files
                let file_id = index / 2;
                if moved_files[file_id] {
                    blocks.skip_free(size);
                } else {
                    blocks.push_file(file_id, size);
                }
            } else {
                // Fill free space
                let max_placed_id = (index - 1) / 2;
                let mut free_size = size;
                for index in (0..unmoved_files.len()).rev() {
                    let file_id = unmoved_files[index];

                    // This check allows us to avoid removing files from unmoved_files on regular placement.
                    if file_id <= max_placed_id {
                        break;
                    }

                    let size = input[file_id * 2];
                    if size <= free_size {
                        free_size -= size;
                        blocks.push_file(file_id, size);
                        moved_files[file_id] = true;
                        unmoved_files.remove(index);
                    }
                }

                blocks.skip_free(free_size);
            }
        }

        blocks.checksum
    };

    (Some(checksum_1), Some(checksum_2))
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_solve() {
        let example_input = "2333133121414131402";
        assert_eq!(solve(example_input), (Some(1928), Some(2858)));
    }
}
