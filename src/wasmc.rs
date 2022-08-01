use std::iter::Peekable;

pub fn compile(exp: &str) {

    let mut input = Input::new(exp);
    let node = input.tokenize();
    //println!("{:?}", node);

    println!("(module");
    println!("  (func $main (result i32)");

    node.gen();

    println!("  )");
    println!("  (export \"main\" (func $main))");
    println!(")");

}


#[derive(Debug)]
enum NodeKind {
    Add,
    Sub,
    Mult,
    Div,
    Num(i32)
}

type Link = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
    kind: NodeKind,
    lhs: Link,
    rhs: Link
}

impl Node {
    fn new(kind: NodeKind, lhs: Link, rhs: Link) -> Self {
        Self { kind, lhs, rhs }
    }
    fn link(node: Node) -> Link {
        Some(Box::new(node))
    }
    fn gen(&self) {
        if let Some(child) = &self.lhs {
            child.gen();
        }
        if let Some(child) = &self.rhs {
            child.gen();
        }
        match self.kind {
            NodeKind::Num(num) => {
                println!("   i32.const {}", num);
            },
            NodeKind::Add => {
                println!("   i32.add");
            },
            NodeKind::Sub => {
                println!("   i32.sub");
            },
            NodeKind::Mult => {
                println!("   i32.mul");
            },
            NodeKind::Div => {
                println!("   i32.div_s");
            },
        }
    }
}

#[derive(PartialEq)]
enum Token<'a> {
    Num(i32),
    Reserved(&'a str),
}

struct Input<'a> {
    token_iterator: Peekable<TokenIterator<'a>>,
}

impl <'a> Input<'a> {
    fn new(input: &'a str) -> Self {
        Self { token_iterator: TokenIterator { s: input }.peekable() }
    }

    fn tokenize(&mut self) -> Node {
        self.expr()
    }

    fn expr(&mut self) -> Node {
        let mut node = self.mul();
        loop {
            match self.token_iterator.peek() {
                Some(Token::Reserved("+")) => {
                    self.token_iterator.next();
                    let right = self.mul();
                    node = Node::new(NodeKind::Add, Node::link(node), Node::link(right));
                },
                Some(Token::Reserved("-")) => {
                    self.token_iterator.next();
                    let right = self.mul();
                    node = Node::new(NodeKind::Sub, Node::link(node), Node::link(right));
                },
                _ => {
                    break;
                }
            }
        }
        node
    }

    fn mul(&mut self) -> Node {
        let mut node = self.unary();
        loop {
            match self.token_iterator.peek() {
                Some(Token::Reserved("*")) => {
                    self.token_iterator.next();
                    let right = self.unary();
                    node = Node::new(NodeKind::Mult, Node::link(node), Node::link(right));
                },
                Some(Token::Reserved("/")) => {
                    self.token_iterator.next();
                    let right = self.unary();
                    node = Node::new(NodeKind::Div, Node::link(node), Node::link(right));
                },
                _ => {
                    break;
                }
            }
        }
        node
    }

    fn unary(&mut self) -> Node {
        return match self.token_iterator.peek() {
            Some(Token::Reserved("+")) => {
                self.token_iterator.next();
                self.primary()
            },
            Some(Token::Reserved("-")) => {
                self.token_iterator.next();
                let left = Node::new(NodeKind::Num(0), None, None);
                let right = self.primary();
                Node::new(NodeKind::Sub, Node::link(left), Node::link(right))
            },
            _ => {
                self.primary()
            }
        }
    }

    fn primary(&mut self) -> Node {
        loop {
            match self.token_iterator.peek() {
                Some(Token::Reserved("(")) => {
                    self.token_iterator.next();
                    let node = self.expr();
                    self.expect(Token::Reserved(")"));
                    return node
                },
                Some(Token::Num(num)) => {
                    let node = Node::new(NodeKind::Num(*num), None, None);
                    self.token_iterator.next();
                    return node;
                },
                _ => {
                    panic!("factor error");
                }
            }
        }
    }

    fn expect(&mut self, expected: Token) -> Token {
        let next = self.token_iterator.next();
        match next {
            Some(token) => {
                if token != expected {
                    panic!("unexpected token");
                }
                return token;
            },
            _ => {
                panic!("Invalid token stream");
            }
        }

    }
}

struct TokenIterator<'a> {
    s: &'a str,
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
