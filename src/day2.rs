use crate::common::parse_int;
use aoc_runner_derive::aoc;
use derive_more::TryInto;
use itertools::Itertools;
use logos::Logos;

#[derive(Logos, Debug, PartialEq, Eq, TryInto)]
enum Token {
    #[regex(r"\d+", parse_int)]
    Number(u32),

    #[token(" ", logos::skip)]
    Whitespace,

    #[token("\n")]
    Newline,
}

fn iterate_reports<'a, T, H>(input: &'a str, handler: H) -> impl Iterator<Item = T> + 'a
where
    H: Fn(&Vec<u32>) -> T + 'static,
{
    let mut report: Vec<u32> = Vec::with_capacity(20);
    Token::lexer(input).batching(move |it| loop {
        match it.next() {
            Some(Ok(Token::Number(n))) => report.push(n),
            Some(Ok(Token::Newline)) => {
                let result = handler(&report);
                report.clear();
                return Some(result);
            }
            None => return None,
            _ => unreachable!(),
        }
    })
}

fn return_first_invalid_index<'a, I>(report: I) -> Option<usize>
where
    I: Iterator<Item = &'a u32>,
{
    let mut is_ascending: Option<bool> = None;
    report
        .tuple_windows()
        .enumerate()
        .filter_map(|(i, (l, r))| {
            let rhs_greater = *r > *l;
            if let Some(last_iteration_was_ascending) = is_ascending {
                if last_iteration_was_ascending != rhs_greater {
                    return Some(i);
                }
            }
            is_ascending = Some(rhs_greater);

            let abs_diff = r.abs_diff(*l);
            if abs_diff == 1 || abs_diff == 2 || abs_diff == 3 {
                None
            } else {
                Some(i)
            }
        })
        .next()
}

fn skip_index<'a, I, T>(index: &'a usize, iterator: I) -> impl Iterator<Item = T> + 'a
where
    I: Iterator<Item = T> + 'a,
{
    iterator
        .enumerate()
        .filter_map(|(i, v)| if i == *index { None } else { Some(v) })
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    iterate_reports(input, |report| {
        return_first_invalid_index(report.iter()).is_none() as u32
    })
    .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    iterate_reports(input, |report| {
        match return_first_invalid_index(report.iter()) {
            None => 1,
            Some(first_invalid_index) => {
                (return_first_invalid_index(skip_index(&first_invalid_index, report.iter()))
                    .is_none()
                    || return_first_invalid_index(skip_index(
                        &(first_invalid_index + 1),
                        report.iter(),
                    ))
                    .is_none()
                    || return_first_invalid_index(skip_index(&0, report.iter())).is_none())
                    as u32
            }
        }
    })
    .sum()
}
