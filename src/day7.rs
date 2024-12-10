use crate::common::parse_long;
use aoc_runner_derive::aoc;
use itertools::Itertools;
use logos::{Lexer, Logos};
use bitvec::prelude::*;

#[derive(Logos, Debug, PartialEq, Eq)]
enum Token {
    #[regex(r"\d+:", parse_answer, priority = 100)]
    Test(u64),

    #[regex(r"\d+", parse_long)]
    Term(u64),

    #[token(" ", logos::skip)]
    Whitespace,

    #[token("\n")]
    Newline,
}

fn parse_answer(lex: &mut Lexer<Token>) -> Option<u64> {
    Into::<&str>::into(lex.slice().strip_suffix(':').unwrap())
        .parse()
        .ok()
}

#[derive(Debug)]
struct Equation {
    answer: u64,
    terms: Vec<u64>,
}

enum EquationAttemptOutcome {
    Found,
    TooSmall,
    TooBig(usize),
}

impl Equation {
    fn from_tokens<I>(tokens: &mut I) -> Option<Self>
    where
        I: Iterator<Item = Token>,
    {
        if let Some(Token::Test(answer)) = tokens.next() {
            let terms = tokens
                .take_while(|t| matches!(t, Token::Term(_)))
                .map(|t| {
                    if let Token::Term(n) = t {
                        n
                    } else {
                        unreachable!()
                    }
                })
                .collect();
            Some(Equation { answer, terms })
        } else {
            None
        }
    }

    fn minimum_answer(&self) -> u64 {
        self.terms.iter().sum()
    }

    fn maximum_answer(&self) -> u64 {
        self.terms.iter().product()
    }

    fn attempt<const L: usize>(&self, seq: &BitSlice<u32>) -> EquationAttemptOutcome {
        let mut acc = self.terms[0];
        for i in 1..L {
            acc = if seq[i - 1] {
                acc * self.terms[i]
            } else {
                acc + self.terms[i]
            };
            if acc > self.answer {
                return EquationAttemptOutcome::TooBig(i);
            }
        }
        if acc == self.answer {
            EquationAttemptOutcome::Found
        } else {
            EquationAttemptOutcome::TooSmall
        }
    }

    fn has_possible_solution_part_1_sized<const L: usize>(&self) -> bool {
        let mut i = [2u32.pow(L as u32 - 1) - 1];
        loop {
            let bits = i.view_bits();
            let outcome = self.attempt::<L>(bits);
            if let EquationAttemptOutcome::Found = outcome {
                return true;
            }
            if i[0] == 0 {
                return false;
            }
            if let EquationAttemptOutcome::TooBig(index) = outcome {
                i[0] -= 2u32.pow((L - index - 1) as u32);
            }
            i[0] -= 1;
        }
    }

    fn has_possible_solution_part_1(&self) -> bool {
        match self.terms.len() {
            0 => unreachable!(),
            1 => self.terms[0] == self.answer,
            2 => self.has_possible_solution_part_1_sized::<2>(),
            3 => self.has_possible_solution_part_1_sized::<3>(),
            4 => self.has_possible_solution_part_1_sized::<4>(),
            5 => self.has_possible_solution_part_1_sized::<5>(),
            6 => self.has_possible_solution_part_1_sized::<6>(),
            7 => self.has_possible_solution_part_1_sized::<7>(),
            8 => self.has_possible_solution_part_1_sized::<8>(),
            9 => self.has_possible_solution_part_1_sized::<9>(),
            10 => self.has_possible_solution_part_1_sized::<10>(),
            11 => self.has_possible_solution_part_1_sized::<11>(),
            12 => self.has_possible_solution_part_1_sized::<12>(),
            13 => self.has_possible_solution_part_1_sized::<13>(),
            14 => self.has_possible_solution_part_1_sized::<14>(),
            15 => self.has_possible_solution_part_1_sized::<15>(),
            16 => self.has_possible_solution_part_1_sized::<16>(),
            _ => unreachable!()
        }
    }
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u64 {
    Token::lexer(input)
        .filter_map(|r| r.ok())
        .batching(Equation::from_tokens)
        .filter(|equation| equation.has_possible_solution_part_1())
        .map(|equation| equation.answer)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let input = "1: 1 2\n2: 3 4\n";
        let mut lex = Token::lexer(input);
        assert_eq!(lex.next(), Some(Ok(Token::Test(1))));
        assert_eq!(lex.next(), Some(Ok(Token::Term(1))));
        assert_eq!(lex.next(), Some(Ok(Token::Term(2))));
        assert_eq!(lex.next(), Some(Ok(Token::Newline)));
        assert_eq!(lex.next(), Some(Ok(Token::Test(2))));
        assert_eq!(lex.next(), Some(Ok(Token::Term(3))));
        assert_eq!(lex.next(), Some(Ok(Token::Term(4))));
        assert_eq!(lex.next(), Some(Ok(Token::Newline)));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn test_equation_from_tokens() {
        let input = "100: 10 10 1 1 1 1 1 1 1 1
";
        assert_eq!(part1(input), 100);
    }
}
