#[derive(PartialEq)]
#[derive(Debug)]
pub enum Token<'a> {
    Num(i32),
    Reserved(&'a str),
    Ident(&'a str),
    Return,
    If,
    Else,
    While,
    For,
}

pub struct TokenIterator<'a> {
    pub s: &'a str,
}

struct Keyword {
    word: &'static str,
    token: Token<'static>,
}

// 予約語
const KEYWORDS: [Keyword; 5] = [
    Keyword{ word: "return", token: Token::Return },
    Keyword{ word: "if", token: Token::If },
    Keyword{ word: "else", token: Token::Else },
    Keyword{ word: "while", token: Token::While },
    Keyword{ word: "for", token: Token::For },
];

// 演算子などの記号。長い順に並べる
const RESERVED_TOKENS: [&str; 17] = [
    "==", "!=", "<=", ">=", "<", ">", "(", ")", "{", "}", "+", "-", "*", "/", "=", ";", ","
];

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.s = self.s.trim_start();
        if self.s.is_empty() {
            return None;
        }

        for keyword in KEYWORDS {
            if self.consume_keyword(keyword.word) {
                return Some(keyword.token);
            }
        }

        for token in RESERVED_TOKENS {
            if self.s.starts_with(token) {
                self.s = self.s.split_at(token.len()).1;
                return Some(Token::Reserved(token));
            }
        }

        match self.s.chars().next() {
            Some('a'..='z' | 'A'..='Z') => {
                let (ident, remain) = split_ident(self.s);
                self.s = remain;
                return Some(Token::Ident(ident));
            }
            _ => (),
        }
        let (digit_s, remain_s) = split_digit(self.s);
        if !digit_s.is_empty() {
            self.s = remain_s;
            return Some(Token::Num(i32::from_str_radix(digit_s, 10).unwrap()));
        }
        panic!("Invalid token stream")
    }
}

impl TokenIterator<'_> {

    fn consume_keyword(&mut self, word: &str) -> bool {
        let len = word.len();
        if self.s.starts_with(word) {
            let next = self.s.split_at(len).1.chars().next();
            match next {
                Some(c) => {
                    if !is_ident_char(c) {
                        self.s = self.s.split_at(len).1;
                        return true;
                    }
                },
                _ => {}
            }
        }
        return false;
    }

}

fn split_digit(s: &str) -> (&str, &str) {
    let first_non_num_idx = s.find(|c| !char::is_numeric(c)).unwrap_or(s.len());
    s.split_at(first_non_num_idx)
}

fn split_ident(s: &str) -> (&str, &str) {
    let index = s.find(|c| !is_ident_char(c)).unwrap_or(s.len());
    s.split_at(index)
}

fn is_ident_char(c: char) -> bool {
    char::is_alphanumeric(c) || c == '_'
}

#[test]
fn test() {
    let mut it = TokenIterator { s: "(18-2)/2*4+a" }.peekable();
    assert_eq!(it.next(), Some(Token::Reserved("(")));
    assert_eq!(it.next(), Some(Token::Num(18)));
    assert_eq!(it.next(), Some(Token::Reserved("-")));
    assert_eq!(it.next(), Some(Token::Num(2)));
    assert_eq!(it.next(), Some(Token::Reserved(")")));
    assert_eq!(it.next(), Some(Token::Reserved("/")));
    assert_eq!(it.next(), Some(Token::Num(2)));
    assert_eq!(it.next(), Some(Token::Reserved("*")));
    assert_eq!(it.next(), Some(Token::Num(4)));
    assert_eq!(it.next(), Some(Token::Reserved("+")));
    assert_eq!(it.next(), Some(Token::Ident("a")));
    assert_eq!(it.next(), None);
}

#[test]
fn test_rel() {
    let mut it = TokenIterator { s: "1<2<=3>=2>1" }.peekable();
    assert_eq!(it.next(), Some(Token::Num(1)));
    assert_eq!(it.next(), Some(Token::Reserved("<")));
    assert_eq!(it.next(), Some(Token::Num(2)));
    assert_eq!(it.next(), Some(Token::Reserved("<=")));
    assert_eq!(it.next(), Some(Token::Num(3)));
    assert_eq!(it.next(), Some(Token::Reserved(">=")));
    assert_eq!(it.next(), Some(Token::Num(2)));
    assert_eq!(it.next(), Some(Token::Reserved(">")));
    assert_eq!(it.next(), Some(Token::Num(1)));
    assert_eq!(it.next(), None);
}

#[test]
fn test_expr() {
    let mut it = TokenIterator { s: "a=1;b=a+1;" }.peekable();
    assert_eq!(it.next(), Some(Token::Ident("a")));
    assert_eq!(it.next(), Some(Token::Reserved("=")));
    assert_eq!(it.next(), Some(Token::Num(1)));
    assert_eq!(it.next(), Some(Token::Reserved(";")));
    assert_eq!(it.next(), Some(Token::Ident("b")));
    assert_eq!(it.next(), Some(Token::Reserved("=")));
    assert_eq!(it.next(), Some(Token::Ident("a")));
    assert_eq!(it.next(), Some(Token::Reserved("+")));
    assert_eq!(it.next(), Some(Token::Num(1)));
    assert_eq!(it.next(), Some(Token::Reserved(";")));
    assert_eq!(it.next(), None);
}

#[test]
fn variable() {
    let mut it = TokenIterator { s: "aZ_09=1;b=aZ_09+1;" }.peekable();
    assert_eq!(it.next(), Some(Token::Ident("aZ_09")));
    assert_eq!(it.next(), Some(Token::Reserved("=")));
    assert_eq!(it.next(), Some(Token::Num(1)));
    assert_eq!(it.next(), Some(Token::Reserved(";")));
    assert_eq!(it.next(), Some(Token::Ident("b")));
    assert_eq!(it.next(), Some(Token::Reserved("=")));
    assert_eq!(it.next(), Some(Token::Ident("aZ_09")));
    assert_eq!(it.next(), Some(Token::Reserved("+")));
    assert_eq!(it.next(), Some(Token::Num(1)));
    assert_eq!(it.next(), Some(Token::Reserved(";")));
    assert_eq!(it.next(), None);
}

#[test]
fn test_return() {
    let mut it = TokenIterator { s: "return1=1;return return1;" }.peekable();
    assert_eq!(it.next(), Some(Token::Ident("return1")));
    assert_eq!(it.next(), Some(Token::Reserved("=")));
    assert_eq!(it.next(), Some(Token::Num(1)));
    assert_eq!(it.next(), Some(Token::Reserved(";")));
    assert_eq!(it.next(), Some(Token::Return));
    assert_eq!(it.next(), Some(Token::Ident("return1")));
    assert_eq!(it.next(), Some(Token::Reserved(";")));
}

#[test]
fn test_if_else() {
    let mut it = TokenIterator { s: "if (a == 0) return 1; else return 2;" }.peekable();
    assert_eq!(it.next(), Some(Token::If));
    assert_eq!(it.next(), Some(Token::Reserved("(")));
    assert_eq!(it.next(), Some(Token::Ident("a")));
    assert_eq!(it.next(), Some(Token::Reserved("==")));
    assert_eq!(it.next(), Some(Token::Num(0)));
    assert_eq!(it.next(), Some(Token::Reserved(")")));
    assert_eq!(it.next(), Some(Token::Return));
    assert_eq!(it.next(), Some(Token::Num(1)));
    assert_eq!(it.next(), Some(Token::Reserved(";")));
    assert_eq!(it.next(), Some(Token::Else));
    assert_eq!(it.next(), Some(Token::Return));
    assert_eq!(it.next(), Some(Token::Num(2)));
    assert_eq!(it.next(), Some(Token::Reserved(";")));
}

#[test]
fn test_while() {
    let mut it = TokenIterator { s: "while(a>0)a=a-1;" }.peekable();
    assert_eq!(it.next(), Some(Token::While));
    assert_eq!(it.next(), Some(Token::Reserved("(")));
    assert_eq!(it.next(), Some(Token::Ident("a")));
    assert_eq!(it.next(), Some(Token::Reserved(">")));
    assert_eq!(it.next(), Some(Token::Num(0)));
    assert_eq!(it.next(), Some(Token::Reserved(")")));
    assert_eq!(it.next(), Some(Token::Ident("a")));
    assert_eq!(it.next(), Some(Token::Reserved("=")));
    assert_eq!(it.next(), Some(Token::Ident("a")));
    assert_eq!(it.next(), Some(Token::Reserved("-")));
    assert_eq!(it.next(), Some(Token::Num(1)));
    assert_eq!(it.next(), Some(Token::Reserved(";")));
}

#[test]
fn test_for() {
    let mut it = TokenIterator { s: "for(a=0;a<3;a=a+1)b=b+a;" }.peekable();
    assert_eq!(it.next(), Some(Token::For));
    assert_eq!(it.next(), Some(Token::Reserved("(")));
    assert_eq!(it.next(), Some(Token::Ident("a")));
    assert_eq!(it.next(), Some(Token::Reserved("=")));
    assert_eq!(it.next(), Some(Token::Num(0)));
    assert_eq!(it.next(), Some(Token::Reserved(";")));
    assert_eq!(it.next(), Some(Token::Ident("a")));
    assert_eq!(it.next(), Some(Token::Reserved("<")));
    assert_eq!(it.next(), Some(Token::Num(3)));
    assert_eq!(it.next(), Some(Token::Reserved(";")));
    assert_eq!(it.next(), Some(Token::Ident("a")));
    assert_eq!(it.next(), Some(Token::Reserved("=")));
    assert_eq!(it.next(), Some(Token::Ident("a")));
    assert_eq!(it.next(), Some(Token::Reserved("+")));
    assert_eq!(it.next(), Some(Token::Num(1)));
    assert_eq!(it.next(), Some(Token::Reserved(")")));
    assert_eq!(it.next(), Some(Token::Ident("b")));
    assert_eq!(it.next(), Some(Token::Reserved("=")));
    assert_eq!(it.next(), Some(Token::Ident("b")));
    assert_eq!(it.next(), Some(Token::Reserved("+")));
    assert_eq!(it.next(), Some(Token::Ident("a")));
    assert_eq!(it.next(), Some(Token::Reserved(";")));
}

#[test]
fn test_block() {
    let mut it = TokenIterator { s: "{a=a+1; return a;}" }.peekable();
    assert_eq!(it.next(), Some(Token::Reserved("{")));
    assert_eq!(it.next(), Some(Token::Ident("a")));
    assert_eq!(it.next(), Some(Token::Reserved("=")));
    assert_eq!(it.next(), Some(Token::Ident("a")));
    assert_eq!(it.next(), Some(Token::Reserved("+")));
    assert_eq!(it.next(), Some(Token::Num(1)));
    assert_eq!(it.next(), Some(Token::Reserved(";")));
    assert_eq!(it.next(), Some(Token::Return));
    assert_eq!(it.next(), Some(Token::Ident("a")));
    assert_eq!(it.next(), Some(Token::Reserved(";")));
    assert_eq!(it.next(), Some(Token::Reserved("}")));
}

#[test]
fn test_func() {
    let mut it = TokenIterator { s: "add(a, b){return a+b;}" }.peekable();
    assert_eq!(it.next(), Some(Token::Ident("add")));
    assert_eq!(it.next(), Some(Token::Reserved("(")));
    assert_eq!(it.next(), Some(Token::Ident("a")));
    assert_eq!(it.next(), Some(Token::Reserved(",")));
    assert_eq!(it.next(), Some(Token::Ident("b")));
    assert_eq!(it.next(), Some(Token::Reserved(")")));
    assert_eq!(it.next(), Some(Token::Reserved("{")));
    assert_eq!(it.next(), Some(Token::Return));
    assert_eq!(it.next(), Some(Token::Ident("a")));
    assert_eq!(it.next(), Some(Token::Reserved("+")));
    assert_eq!(it.next(), Some(Token::Ident("b")));
    assert_eq!(it.next(), Some(Token::Reserved(";")));
    assert_eq!(it.next(), Some(Token::Reserved("}")));
}

