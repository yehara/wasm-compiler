#[derive(PartialEq)]
pub enum Token<'a> {
    Num(i32),
    Reserved(&'a str),
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
