use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::VecDeque;
use std::iter::once;

const EMPTY: usize = usize::MAX;

#[derive(Debug, Clone, Copy)]
pub struct FileSystemEntry {
    id: usize,
    length: usize,
}

fn parse_entries_and_gaps_forward<'a>(
    input: &'a str,
) -> impl Iterator<Item = [FileSystemEntry; 2]> + 'a {
    input
        .bytes()
        .into_iter()
        .map(|b| (b - ('0' as u8)) as usize)
        .tuples()
        .scan(0usize, |acc, (entry, blank)| {
            let file = FileSystemEntry {
                id: *acc,
                length: entry,
            };
            let blank = FileSystemEntry {
                id: EMPTY,
                length: blank,
            };
            *acc += 1;
            Some([file, blank])
        })
}

fn parse_entries_backward<'a>(input: &'a str) -> impl Iterator<Item = FileSystemEntry> + 'a {
    let max_id = (input.len() - 1) / 2;
    input
        .bytes()
        .into_iter()
        .rev()
        .step_by(2)
        .map(|b| (b - '0' as u8) as usize)
        .enumerate()
        .map(move |(index, entry)| FileSystemEntry {
            id: max_id - index,
            length: entry,
        })
}

fn yield_entry(entry: FileSystemEntry, index: &mut usize, total: &mut usize) {
    let id = entry.id as usize;
    *total += *index * id * entry.length as usize;
    *total += id * ((entry.length as usize * (entry.length as usize - 1)) / 2);
    *index += entry.length as usize;
}

#[aoc(day9, part1)]
pub fn part1(input: &str) -> usize {
    let mut input_forward = parse_entries_and_gaps_forward(input);
    let mut non_empty_backward = parse_entries_backward(input);

    let mut item_forward: FileSystemEntry;
    let mut blank_forward: FileSystemEntry;
    [item_forward, blank_forward] = input_forward.next().unwrap();
    let mut item_backward = non_empty_backward.next().unwrap();
    let mut already_consumed_from_backward_cursor = 0usize;

    let mut index = 0usize;
    let mut total = 0usize;

    loop {
        if item_forward.id >= item_backward.id {
            if already_consumed_from_backward_cursor > 0 {
                yield_entry(
                    FileSystemEntry {
                        id: item_backward.id,
                        length: item_backward.length - already_consumed_from_backward_cursor,
                    },
                    &mut index,
                    &mut total,
                );
            }
            return total;
        }

        yield_entry(item_forward, &mut index, &mut total);

        let mut gap_size = blank_forward.length;
        if gap_size == 0 {
            [item_forward, blank_forward] = input_forward.next().unwrap();
            continue;
        }

        loop {
            let backward_cursor_length =
                item_backward.length - already_consumed_from_backward_cursor;
            if gap_size >= backward_cursor_length {
                gap_size -= backward_cursor_length;
                yield_entry(
                    FileSystemEntry {
                        id: item_backward.id,
                        length: backward_cursor_length,
                    },
                    &mut index,
                    &mut total,
                );
                already_consumed_from_backward_cursor = 0;
                item_backward = non_empty_backward.next().unwrap();
                if gap_size == 0 {
                    break;
                }
            } else {
                yield_entry(
                    FileSystemEntry {
                        id: item_backward.id,
                        length: gap_size,
                    },
                    &mut index,
                    &mut total,
                );
                already_consumed_from_backward_cursor += gap_size;
                break;
            }
        }

        [item_forward, blank_forward] = input_forward.next().unwrap();
    }
}

#[derive(Debug, Clone, Copy)]
struct Gap {
    start_index: usize,
    length: usize,
}

impl PartialEq for Gap {
    fn eq(&self, other: &Self) -> bool {
        self.start_index == other.start_index
    }
}

impl Eq for Gap {}

impl PartialOrd for Gap {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Gap {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start_index.cmp(&other.start_index)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PinnedFileSystemEntry {
    start_index: usize,
    id: usize,
    length: usize,
}

impl PartialOrd for PinnedFileSystemEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PinnedFileSystemEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start_index.cmp(&other.start_index)
    }
}

fn parse_entries_and_gaps_forward_part_2<'a>(
    input: &'a str,
) -> impl Iterator<Item = (PinnedFileSystemEntry, Gap)> + 'a {
    let mut index = 0usize;
    let mut location = 0usize;

    input
        .bytes()
        .into_iter()
        .map(|b| (b - ('0' as u8)) as usize)
        .chain(once(0))
        .tuples::<(_, _)>()
        .map(move |(entry_len, gap_len)| {
            let entry = PinnedFileSystemEntry {
                start_index: location,
                id: index,
                length: entry_len,
            };
            index += 1;
            location += entry_len;
            let gap = Gap {
                start_index: location,
                length: gap_len,
            };
            location += gap_len;
            (entry, gap)
        })
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> usize {
    let mut gaps: [VecDeque<u32>; 10] = Default::default();
    let mut entries: Vec<PinnedFileSystemEntry> = Vec::with_capacity(input.len() / 2);
    for (entry, gap) in parse_entries_and_gaps_forward_part_2(input) {
        gaps[gap.length].push_back(gap.start_index as u32);
        entries.push(entry);
    }

    entries
        .into_iter()
        .rev()
        .map(|entry| {
            let mut maybe_fillable_gap: Option<(u32, usize)> = None;
            let mut earliest_start_index = entry.start_index as u32;
            for l in entry.length..10 {
                if gaps[l].len() > 0 && gaps[l][0] < earliest_start_index {
                    earliest_start_index = gaps[l][0];
                    maybe_fillable_gap = Some((gaps[l][0], l));
                }
            }

            let Some(gap) = maybe_fillable_gap else {
                return entry;
            };

            gaps[gap.1].pop_front();

            if gap.1 > entry.length {
                let new_gap_length = gap.1 - entry.length;
                let new_gap_start_index = gap.0 + entry.length as u32;
                let gap_insertion_point = gaps[new_gap_length]
                    .binary_search_by(|gap| gap.cmp(&new_gap_start_index))
                    .unwrap_err();
                gaps[new_gap_length].insert(
                    gap_insertion_point,
                    new_gap_start_index
                );
            }

            PinnedFileSystemEntry {
                start_index: gap.0 as usize,
                id: entry.id,
                length: entry.length,
            }
        })
        .fold(0usize, |mut total, mut entry| {
            yield_entry(
                FileSystemEntry {
                    id: entry.id,
                    length: entry.length,
                },
                &mut entry.start_index,
                &mut total,
            );
            total
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "2333133121414131402";
        assert_eq!(part1(input), 1928);
    }

    #[test]
    fn test_part2() {
        let input = "2333133121414131402";
        assert_eq!(part2(input), 2858);
    }
}
