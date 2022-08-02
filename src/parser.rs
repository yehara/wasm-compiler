#[derive(PartialEq)]
#[derive(Debug)]
pub enum Token<'a> {
    Num(i32),
    Reserved(&'a str),
    Ident(&'a str),
}

pub struct TokenIterator<'a> {
    pub s: &'a str,
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.s = self.s.trim_start();
        if self.s.is_empty() {
            return None;
        }
        if self.s.starts_with("==") {
            self.s = self.s.split_at(2).1;
            return Some(Token::Reserved("=="));
        }
        if self.s.starts_with("!=") {
            self.s = self.s.split_at(2).1;
            return Some(Token::Reserved("!="));
        }
        if self.s.starts_with("<=") {
            self.s = self.s.split_at(2).1;
            return Some(Token::Reserved("<="));
        }
        if self.s.starts_with(">=") {
            self.s = self.s.split_at(2).1;
            return Some(Token::Reserved(">="));
        }
        if self.s.starts_with(">") {
            self.s = self.s.split_at(1).1;
            return Some(Token::Reserved(">"));
        }
        if self.s.starts_with("<") {
            self.s = self.s.split_at(1).1;
            return Some(Token::Reserved("<"));
        }
        if self.s.starts_with("(") {
            self.s = self.s.split_at(1).1;
            return Some(Token::Reserved("("));
        }
        if self.s.starts_with(")") {
            self.s = self.s.split_at(1).1;
            return Some(Token::Reserved(")"));
        }
        if self.s.starts_with("+") {
            self.s = self.s.split_at(1).1;
            return Some(Token::Reserved("+"));
        }
        if self.s.starts_with("-") {
            self.s = self.s.split_at(1).1;
            return Some(Token::Reserved("-"));
        }
        if self.s.starts_with("*") {
            self.s = self.s.split_at(1).1;
            return Some(Token::Reserved("*"));
        }
        if self.s.starts_with("/") {
            self.s = self.s.split_at(1).1;
            return Some(Token::Reserved("/"));
        }
        match self.s.chars().next() {
            Some('a'..='z') => {
                let (ident, rest) = self.s.split_at(1);
                self.s = rest;
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

fn split_digit(s: &str) -> (&str, &str) {
    let first_non_num_idx = s.find(|c| !char::is_numeric(c)).unwrap_or(s.len());
    s.split_at(first_non_num_idx)
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
}