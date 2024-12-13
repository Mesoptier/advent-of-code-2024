pub const DAY: usize = 4;

pub fn solve(input: &str) -> (Option<u32>, Option<u32>) {
    let input = input.as_bytes();
    let width = input.iter().position(|&c| c == b'\n').unwrap();
    let line_width = width + 1;
    let height = input.len() / (width + 1);

    let char_at = |x: isize, y: isize| -> char {
        let index = x + y * (width as isize + 1);
        input[index as usize] as char
    };

    // Part 1
    let mut count1 = 0;

    for index in 0..input.len() {
        let target = match input[index] {
            b'X' => [b'M', b'A', b'S'],
            b'S' => [b'A', b'M', b'X'],
            _ => continue,
        };

        for offset in [
            1,              // right
            line_width - 1, // down left
            line_width,     // down
            line_width + 1, // down right
        ] {
            // Break if out of bounds (any next offset will also be out of bounds).
            if index + offset * 3 >= input.len() {
                break;
            }

            // Note: wrapping around is fine, since then '\n' will be included in the string,
            // so it will never match.

            if input[index + offset * 1] == target[0]
                && input[index + offset * 2] == target[1]
                && input[index + offset * 3] == target[2]
            {
                count1 += 1;
            }
        }
    }

    // Part 2
    let mut count2 = 0;

    for y in 1..height as isize - 1 {
        for x in 1..width as isize - 1 {
            if char_at(x, y) != 'A' {
                continue;
            }
            if !matches!(
                [char_at(x + 1, y + 1), char_at(x - 1, y - 1)],
                ['M', 'S'] | ['S', 'M']
            ) {
                continue;
            }
            if !matches!(
                [char_at(x - 1, y + 1), char_at(x + 1, y - 1)],
                ['M', 'S'] | ['S', 'M']
            ) {
                continue;
            }

            count2 += 1;
        }
    }

    (Some(count1), Some(count2))
}
#[cfg(test)]
mod tests {
    use super::solve;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let example_input = indoc! {"
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
        "};
        assert_eq!(solve(example_input), (Some(18), Some(9)));
    }
}
