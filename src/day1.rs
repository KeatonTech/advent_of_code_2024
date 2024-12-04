use aoc_runner_derive::aoc;
use counter::Counter;
use logos::Logos;
use itertools::Itertools;
use crate::common::parse_int;

#[derive(Logos, Debug, PartialEq, Eq)]
enum Token {
    #[regex(r"\d+", parse_int)]
    Number(u32),

    #[regex(r"\s*", logos::skip)]
    Whitespace,

    #[token("\n")]
    Newline,
}


fn generate_pairs<'a>(input: &'a str) -> impl Iterator<Item = [u32;2]> + 'a {
    Token::lexer(input)
        .tuples::<(_, _, _)>()
        .map(|(l, r, newline)| {
            assert_eq!(newline.unwrap(), Token::Newline);
            if let Ok(Token::Number(left)) = l {
                if let Ok(Token::Number(right)) = r {
                    return [left, right]
                }
            }
            panic!("unexpected token {:?}", l);
        })
}

fn collect_pairs<'a, L: Extend<u32>, R: Extend<u32>>(left: &mut L, right: &mut R, input: &'a str) {
    for [l, r] in generate_pairs(input) {
        left.extend([l]);
        right.extend([r]);
    }
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let mut left = vec![];
    let mut right = vec![];
    collect_pairs(&mut left, &mut right, input);
    left.sort_unstable();
    right.sort_unstable();
    left.iter().zip(right.iter()).map(|(l, r)| l.abs_diff(*r)).sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let mut left = vec![];
    let mut right: Counter<u32, u32> = Counter::new();
    collect_pairs(&mut left, &mut right, input);
    left.into_iter()
        .map(|n| *right.get(&n).unwrap_or(&0) * n)
        .sum()
}