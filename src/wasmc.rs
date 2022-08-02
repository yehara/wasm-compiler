use std::iter::Peekable;
use crate::parser::Token;
use crate::parser::TokenIterator;

pub fn compile(exp: &str) {

    let mut input = Input::new(exp);
    let nodes = input.tokenize();
    //println!("{:?}", node);

    println!("(module");
    println!("  (func $main (result i32)");

    // 固定で(a..z)に対応する 26 個(のローカル変数を用意しておく
    for _ in 0..26 {
        println!("   (local i32)");
    }

    let mut first = true;

    for node in nodes {
        if !first {
            println!("   drop");
        } else {
            first = false;
        }
        node.gen();
    }

    println!("  )");
    println!("  (export \"main\" (func $main))");
    println!(")");

}


#[derive(Debug)]
#[derive(PartialEq)]
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
    Assign,
    LVar(i32),
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

    fn gen_lval(&self) {
        match self.lhs.as_ref().unwrap().kind {
            NodeKind::LVar(index) => {
                self.rhs.as_ref().unwrap().gen();
                // 変数に保存しつつ、スタックに残しておく
                println!("   local.tee {}", index);
            },
            _ => {
                panic!("代入の左辺値が変数ではありません");
            }
        }
    }

    fn gen(&self) {

        if self.kind == NodeKind::Assign {
            self.gen_lval();
            return;
        }

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
            },
            NodeKind::LVar(index) => {
                println!("   local.get {}", index);
            },
            _ => ()
        }
    }
}


struct Input<'a> {
    token_iterator: Peekable<TokenIterator<'a>>,
}

/*
program    = stmt*
stmt       = expr ";"
expr       = assign
assign     = equality ("=" assign)?
equality   = relational ("==" relational | "!=" relational)*
relational = add ("<" add | "<=" add | ">" add | ">=" add)*
add        = mul ("+" mul | "-" mul)*
mul        = unary ("*" unary | "/" unary)*
unary      = ("+" | "-")? primary
primary    = num | ident | "(" expr ")"
 */
impl <'a> Input<'a> {
    fn new(input: &'a str) -> Self {
        Self { token_iterator: TokenIterator { s: input }.peekable() }
    }

    fn tokenize(&mut self) -> Vec<Node> {
        self.program()
    }

    fn program(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();
        while self.token_iterator.peek() != None {
            nodes.push(self.stmt());
        }
        nodes
    }

    fn stmt(&mut self) -> Node {
        let node = self.expr();
        self.expect(Token::Reserved(";"));
        node
    }

    fn expr(&mut self) -> Node {
        return self.assign();
    }

    fn assign(&mut self) -> Node {
        let mut node = self.equality();
        match self.token_iterator.peek() {
            Some(Token::Reserved("=")) => {
                self.token_iterator.next();
                let right = self.assign();
                node = Node::new(NodeKind::Assign, Node::link(node),Node::link(right));
            },
            _ => ()
        }
        node
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
                Some(Token::Ident(name)) => {
                    let index = name.as_bytes()[0] - b'a';
                    let node = Node::new(NodeKind::LVar(index as i32), None, None);
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

