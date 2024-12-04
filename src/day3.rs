use aoc_runner_derive::aoc;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"mul\(\d+,\d+\)|don't\(\)|do\(\)").unwrap();
}

enum Token {
    Mul((u32, u32)),
    Do,
    Dont,
}

fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    RE.find_iter(input).map(|m| {
        match m.as_str().bytes().nth(2).unwrap() {
            b'l' => {
                let mut slice = m.as_str();
                slice = &slice[4..]; // Take off "mul("
                slice = &slice[..slice.len() - 1]; // Take off ")"
                let (l, r) = slice.split_at(slice.find(',').unwrap());
                Token::Mul((l.parse().unwrap(), r[1..].parse().unwrap()))
            }
            b'(' => Token::Do,
            b'n' => Token::Dont,
            _ => unreachable!(),
        }
    })
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    tokenize(input)
        .filter(|t| matches!(t, Token::Mul((_, _))))
        .map(|t| match t {
            Token::Mul((l, r)) => l * r,
            _ => unreachable!(),
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    tokenize(input)
        .fold((true, 0u32), |(enabled, acc), captures| {
            if matches!(captures, Token::Do) {
                (true, acc)
            } else if !enabled || matches!(captures, Token::Dont) {
                (false, acc)
            } else if let Token::Mul((l, r)) = captures {
                (true, acc + l * r)
            } else {
                unreachable!()
            }
        })
        .1
}
