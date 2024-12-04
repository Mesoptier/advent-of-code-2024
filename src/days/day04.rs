use crate::days::DaySolution;
use copy_range::CopyRange;

pub struct Day04;

impl DaySolution for Day04 {
    type Output1 = u32;
    type Output2 = u32;

    fn solve(&self, input: &str) -> (Option<Self::Output1>, Option<Self::Output2>) {
        let input = input.as_bytes();
        let width = input.iter().position(|&c| c == b'\n').unwrap();
        let height = input.len() / (width + 1);

        let x_range: CopyRange<_> = (0..width as isize).into();
        let y_range: CopyRange<_> = (0..height as isize).into();

        let char_at = |x: isize, y: isize| -> char {
            let index = x + y * (width as isize + 1);
            input[index as usize] as char
        };

        // Part 1
        let mut count1 = 0;

        const TARGET_WORD: [char; 4] = ['X', 'M', 'A', 'S'];

        for y in y_range {
            for x in x_range {
                for dy in -1..=1 {
                    if !y_range.contains(&(y + dy * (TARGET_WORD.len() - 1) as isize)) {
                        continue;
                    }

                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        if !x_range.contains(&(x + dx * (TARGET_WORD.len() - 1) as isize)) {
                            continue;
                        }

                        let found_target = 'found_target: {
                            for i in 0..TARGET_WORD.len() as isize {
                                if char_at(x + dx * i, y + dy * i) != TARGET_WORD[i as usize] {
                                    break 'found_target false;
                                }
                            }

                            true
                        };
                        if found_target {
                            count1 += 1;
                        }
                    }
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
}

#[test]
fn test_day_04() {
    let example_input = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX\n";
    assert_eq!(Day04.solve(example_input), (Some(18), Some(9)));
}
