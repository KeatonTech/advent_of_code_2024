use aoc_runner_derive::aoc;
use std::iter::once;
use std::sync::atomic::AtomicU32;
use std::{cmp, thread};
use std::ops::Shr;

const PART1_PATTERN_BYTES_FORWARD: u32 = u32::from_be_bytes([b'X', b'M', b'A', b'S']);
const PART1_PATTERN_BYTES_BACKWARD: u32 = u32::from_be_bytes([b'S', b'A', b'M', b'X']);

const PART2_PATTERN: [[u8; 3]; 3] = [[b'M', 0, b'S'], [0, b'A', 0], [b'M', 0, b'S']];

pub fn input_to_grid<const N: usize>(input: &str) -> [[u8; N]; N] {
    let mut ret: [[u8; N]; N] = [[0; N]; N];
    input.lines().enumerate().for_each(|(row, line)| {
        line.bytes().enumerate().for_each(|(col, c)| {
            ret[row][col] = c;
        })
    });
    ret
}

fn iterate_left_to_right<const N: usize>(grid: &[[u8; N]; N]) -> impl Iterator<Item = u8> + '_ {
    (0..N).flat_map(move |row| {
        (0..N)
            .rev()
            .map(move |col| grid[row][col])
            .chain(once(b'\n'))
    })
}

fn iterate_top_to_bottom<const N: usize>(grid: &[[u8; N]; N]) -> impl Iterator<Item = u8> + '_ {
    (0..N).flat_map(move |col| (0..N).map(move |row| grid[row][col]).chain(once(b'\n')))
}

fn iterate_bottomright_to_topleft<const N: usize>(
    grid: &[[u8; N]; N],
) -> impl Iterator<Item = u8> + '_ {
    (4..(N * 2 - 3)).flat_map(move |bar| {
        let start_coord: [usize; 2] = [N.saturating_sub(bar), N.saturating_sub(N * 2 - bar)];
        let bar_length = cmp::min(bar, N * 2 - bar);
        (0..bar_length)
            .rev()
            .map(move |pos| grid[start_coord[0] + pos][start_coord[1] + pos])
            .chain(once(b'\n'))
    })
}

fn iterate_bottomleft_to_topright<const N: usize>(
    grid: &[[u8; N]; N],
) -> impl Iterator<Item = u8> + '_ {
    (4..(N * 2 - 3)).flat_map(move |bar| {
        let start_coord: [usize; 2] = [N.saturating_sub(N * 2 - bar), cmp::min(bar - 1, N - 1)];
        let bar_length = cmp::min(bar, N * 2 - bar);
        (0..bar_length)
            .rev()
            .map(move |pos| grid[start_coord[0] + pos][start_coord[1] - pos])
            .chain(once(b'\n'))
    })
}

fn count_matches_in_iter<I>(iter: I) -> u32
where
    I: Iterator<Item = u8>,
{
    iter.scan(0u32, |acc, next| {
        *acc = if cfg!(target_endian = "big") {
            acc.overflowing_shl(8).0 | (next as u32)
        } else {
            acc.shr(8) | ((next as u32) << 24)
        };
        Some(*acc)
    })
    .filter(|pattern|
        *pattern == PART1_PATTERN_BYTES_FORWARD || *pattern == PART1_PATTERN_BYTES_BACKWARD
    )
    .count() as u32
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u32 {
    part1_sized::<140>(input)
}

fn part1_sized<const N: usize>(input: &str) -> u32 {
    let grid = input_to_grid::<N>(input);
    let sum = AtomicU32::new(0);

    thread::scope(|s| {
        s.spawn(|| {
            sum.fetch_add(
                count_matches_in_iter(iterate_left_to_right(&grid)),
                std::sync::atomic::Ordering::Relaxed,
            );
        });
        s.spawn(|| {
            sum.fetch_add(
                count_matches_in_iter(iterate_bottomright_to_topleft(&grid)),
                std::sync::atomic::Ordering::Relaxed,
            );
        });
        s.spawn(|| {
            sum.fetch_add(
                count_matches_in_iter(iterate_top_to_bottom(&grid)),
                std::sync::atomic::Ordering::Relaxed,
            );
        });
        s.spawn(|| {
            sum.fetch_add(
                count_matches_in_iter(iterate_bottomleft_to_topright(&grid)),
                std::sync::atomic::Ordering::Relaxed,
            );
        });
    });

    sum.into_inner()
}

trait GridView<const N: usize> {
    fn get(&self, x: usize, y: usize) -> u8;
}

struct StraightGridView<'a, const N: usize>(&'a [[u8; N]; N]);
impl<'a, const N: usize> GridView<N> for StraightGridView<'a, N> {
    #[inline]
    fn get(&self, x: usize, y: usize) -> u8 {
        self.0[y][x]
    }
}

struct NinetyDegreeClockwiseGridView<'a, const N: usize>(&'a [[u8; N]; N]);
impl<'a, const N: usize> GridView<N> for NinetyDegreeClockwiseGridView<'a, N> {
    #[inline]
    fn get(&self, x: usize, y: usize) -> u8 {
        self.0[x][N - 1 - y]
    }
}

struct NinetyDegreeCounterClockwiseGridView<'a, const N: usize>(&'a [[u8; N]; N]);
impl<'a, const N: usize> GridView<N> for NinetyDegreeCounterClockwiseGridView<'a, N> {
    #[inline]
    fn get(&self, x: usize, y: usize) -> u8 {
        self.0[N - 1 - x][y]
    }
}

struct OneEightyDegreeGridView<'a, const N: usize>(&'a [[u8; N]; N]);
impl<'a, const N: usize> GridView<N> for OneEightyDegreeGridView<'a, N> {
    #[inline]
    fn get(&self, x: usize, y: usize) -> u8 {
        self.0[N - 1 - y][N - 1 - x]
    }
}

fn count_pattern_in_grid_view<const N: usize, G: GridView<N>>(view: &G) -> u32 {
    (0..N - 2)
        .flat_map(|sy| {
            (0..N - 2).map(move |sx| {
                (view.get(sx, sy) == PART2_PATTERN[0][0]
                    && view.get(sx + 2, sy) == PART2_PATTERN[0][2]
                    && view.get(sx + 1, sy + 1) == PART2_PATTERN[1][1]
                    && view.get(sx, sy + 2) == PART2_PATTERN[2][0]
                    && view.get(sx + 2, sy + 2) == PART2_PATTERN[2][2]) as u32
            })
        })
        .sum()
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u32 {
    part2_sized::<140>(input)
}

fn part2_sized<const N: usize>(input: &str) -> u32 {
    let grid = input_to_grid::<N>(input);
    let sum = AtomicU32::new(0);

    thread::scope(|s| {
        s.spawn(|| {
            sum.fetch_add(
                count_pattern_in_grid_view(&StraightGridView(&grid)),
                std::sync::atomic::Ordering::Relaxed,
            );
        });
        s.spawn(|| {
            sum.fetch_add(
                count_pattern_in_grid_view(&NinetyDegreeClockwiseGridView(&grid)),
                std::sync::atomic::Ordering::Relaxed,
            );
        });
        s.spawn(|| {
            sum.fetch_add(
                count_pattern_in_grid_view(&NinetyDegreeCounterClockwiseGridView(&grid)),
                std::sync::atomic::Ordering::Relaxed,
            );
        });
        s.spawn(|| {
            sum.fetch_add(
                count_pattern_in_grid_view(&OneEightyDegreeGridView(&grid)),
                std::sync::atomic::Ordering::Relaxed,
            );
        });
    });

    sum.into_inner()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part1() {
        assert_eq!(part1_sized::<10>(EXAMPLE), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_sized::<10>(EXAMPLE), 9);
    }
}
