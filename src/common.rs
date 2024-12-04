use logos::{Lexer, Logos, Source};

pub fn parse_int<'a, T: Logos<'a>>(lex: &mut Lexer<'a, T>) -> Option<u32>
where
    &'a str: From<<<T as Logos<'a>>::Source as Source>::Slice<'a>>
{
    Into::<&str>::into(lex.slice()).parse().ok()
}