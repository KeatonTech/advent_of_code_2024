use aoc_runner_derive::{aoc, aoc_generator};
use genawaiter::{stack::let_gen, yield_};
use itertools::Itertools;
use std::iter::once;
use std::num::NonZeroU16;

#[derive(Debug, Clone, Copy)]
pub struct FileSystemEntry {
    id: Option<NonZeroU16>,
    length: u8,
}

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<FileSystemEntry> {
    input
        .chars()
        .into_iter()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .chain(once(0))
        .tuples()
        .scan(NonZeroU16::MIN, |acc, (entry, blank)| {
            let file = FileSystemEntry {
                id: Some(*acc),
                length: entry,
            };
            let blank = FileSystemEntry {
                id: None,
                length: blank,
            };
            *acc = acc.saturating_add(1);
            Some([file, blank])
        })
        .flat_map(|entries| entries)
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(input: &[FileSystemEntry]) -> usize {
    let mut cursor_forward = 0usize;
    let mut cursor_backward = (input.len() - 1) / 2 * 2;
    let mut already_consumed_from_backward_cursor = 0u8;

    let_gen!(defragmented_generator, {
        loop {
            if (cursor_forward >= cursor_backward) {
                if already_consumed_from_backward_cursor > 0 {
                    yield_!(FileSystemEntry {
                        id: input[cursor_backward].id,
                        length: input[cursor_backward].length
                            - already_consumed_from_backward_cursor
                    });
                }
                return;
            }

            yield_!(input[cursor_forward]);
            cursor_forward += 1;

            assert!(input[cursor_forward].id.is_none());
            let mut gap_size = input[cursor_forward].length;
            if (gap_size == 0) {
                cursor_forward += 1;
                continue;
            }

            loop {
                let backward_cursor_length =
                    input[cursor_backward].length - already_consumed_from_backward_cursor;
                if gap_size >= backward_cursor_length {
                    gap_size -= backward_cursor_length;
                    yield_!(FileSystemEntry {
                        id: input[cursor_backward].id,
                        length: backward_cursor_length
                    });
                    already_consumed_from_backward_cursor = 0;
                    cursor_backward -= 2;
                    if (gap_size == 0) {
                        break;
                    }
                } else {
                    yield_!(FileSystemEntry {
                        id: Some(input[cursor_backward].id.unwrap()),
                        length: gap_size
                    });
                    already_consumed_from_backward_cursor += gap_size;
                    break;
                }
            }

            cursor_forward += 1;
        }
    });

    let mut index = 0usize;
    let mut total = 0usize;
    for entry in defragmented_generator {
        assert!(entry.id.is_some());
        let id = entry.id.unwrap().get() as usize - 1;
        total += index * id * entry.length as usize;
        total += id * ((entry.length as usize * (entry.length as usize - 1)) / 2);
        index += entry.length as usize;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "23331331214";
        let parsed = parse_input(input);
        assert_eq!(parsed.len(), 12);
        assert_eq!(parsed[0].id, Some(NonZeroU16::MIN));
        assert_eq!(parsed[0].length, 2);
        assert_eq!(parsed[1].id, None);
        assert_eq!(parsed[1].length, 3);
        assert_eq!(parsed[2].id, Some(NonZeroU16::new(2).unwrap()));
        assert_eq!(parsed[2].length, 3);
        assert_eq!(parsed[3].id, None);
        assert_eq!(parsed[3].length, 3);
        assert_eq!(parsed[4].id, Some(NonZeroU16::new(3).unwrap()));
        assert_eq!(parsed[4].length, 1);
        assert_eq!(parsed[5].id, None);
        assert_eq!(parsed[5].length, 3);
        assert_eq!(parsed[6].id, Some(NonZeroU16::new(4).unwrap()));
        assert_eq!(parsed[6].length, 3);
        assert_eq!(parsed[7].id, None);
        assert_eq!(parsed[7].length, 1);
        assert_eq!(parsed[8].id, Some(NonZeroU16::new(5).unwrap()));
        assert_eq!(parsed[8].length, 2);
        assert_eq!(parsed[9].id, None);
        assert_eq!(parsed[9].length, 1);
        assert_eq!(parsed[10].id, Some(NonZeroU16::new(6).unwrap()));
        assert_eq!(parsed[10].length, 4);
        assert_eq!(parsed[11].id, None);
        assert_eq!(parsed[11].length, 0);
    }

    #[test]
    fn test_part1() {
        let input = "2333133121414131402";
        let parsed = parse_input(input);
        assert_eq!(part1(&parsed), 1928);
    }
}
