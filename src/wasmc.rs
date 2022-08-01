use std::iter::Peekable;
use crate::parser::Token;
use crate::parser::TokenIterator;

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
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
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
            NodeKind::Equal => {
                println!("   i32.eq");
            },
            NodeKind::NotEqual => {
                println!("   i32.ne");
            },
            NodeKind::GreaterThan => {
                println!("   i32.gt_s");
            },
            NodeKind::GreaterThanOrEqual => {
                println!("   i32.ge_s");
            },
            NodeKind::LessThan => {
                println!("   i32.lt_s");
            },
            NodeKind::LessThanOrEqual => {
                println!("   i32.le_s");
            }
        }
    }
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
        return self.equality();
    }

    fn equality(&mut self) -> Node {
        let mut node = self.relational();
        loop {
            match self.token_iterator.peek() {
                Some(Token::Reserved("==")) => {
                    self.token_iterator.next();
                    let right = self.relational();
                    node = Node::new(NodeKind::Equal, Node::link(node), Node::link(right));
                },
                Some(Token::Reserved("!=")) => {
                    self.token_iterator.next();
                    let right = self.relational();
                    node = Node::new(NodeKind::NotEqual, Node::link(node), Node::link(right));
                },
                _ => {
                    break;
                }
            }
        }
        node
    }

    fn relational(&mut self) -> Node {
        let mut node = self.add();
        loop {
            match self.token_iterator.peek() {
                Some(Token::Reserved(">=")) => {
                    self.token_iterator.next();
                    let right = self.add();
                    node = Node::new(NodeKind::GreaterThanOrEqual, Node::link(node), Node::link(right));
                },
                Some(Token::Reserved(">")) => {
                    self.token_iterator.next();
                    let right = self.add();
                    node = Node::new(NodeKind::GreaterThan, Node::link(node), Node::link(right));
                },
                Some(Token::Reserved("<=")) => {
                    self.token_iterator.next();
                    let right = self.add();
                    node = Node::new(NodeKind::LessThanOrEqual, Node::link(node), Node::link(right));
                },
                Some(Token::Reserved("<")) => {
                    self.token_iterator.next();
                    let right = self.add();
                    node = Node::new(NodeKind::LessThan, Node::link(node), Node::link(right));
                },
                _ => {
                    break;
                }
            }
        }
        node
    }
    fn add(&mut self) -> Node {
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

