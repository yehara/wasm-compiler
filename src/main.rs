use std::env;
use std::iter::Peekable;
use std::process::exit;
use std::str::Chars;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません");
        exit(-1);
    }

    compile(&args[1]);

}

fn compile(exp: &str) {


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

fn strtoi<L: Iterator<Item = char>>(iter: &mut Peekable<L>) -> i32 {
    let mut num = 0;
    while let Some(c) = iter.peek() {
        if c.is_digit(10) {
            num = num * 10 + c.to_digit(10).unwrap() as i32;
            iter.next();
        } else {
            break;
        }
    }
    num
}

#[derive(Debug)]
enum NodeKind {
    Op(char),
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
            NodeKind::Op('+') => {
                println!("   i32.add");
            },
            NodeKind::Op('-') => {
                println!("   i32.sub");
            },
            NodeKind::Op('*') => {
                println!("   i32.mul");
            },
            NodeKind::Op('/') => {
                println!("   i32.div_s");
            },
            _ => {
                eprintln!("式が正しくありません");
                exit(-1);
            }
        }
    }
}

struct Input<'a> {
    input: Peekable<Chars<'a>>,
}

impl <'a> Input<'a> {
    fn new(input: &'a str) -> Self {
        Self { input: input.chars().peekable() }
    }

    fn tokenize(&mut self) -> Node {
        self.expr()
    }

    fn expr(&mut self) -> Node {
        let mut node = self.mul();
        loop {
            match self.input.peek() {
                Some(&'+') | Some(&'-') => {
                    let op = self.input.next().unwrap();
                    let right = self.mul();
                    node = Node::new(NodeKind::Op(op), Node::link(node), Node::link(right));
                },
                Some(&' ') => {
                    self.input.next();
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
            match self.input.peek() {
                Some(&'*') | Some(&'/') => {
                    let op = self.input.next().unwrap();
                    let right = self.unary();
                    node = Node::new(NodeKind::Op(op), Node::link(node), Node::link(right));
                },
                Some(&' ') => {
                    self.input.next();
                },
                _ => {
                    break;
                }
            }
        }
        node
    }

    fn unary(&mut self) -> Node {
        return match self.input.peek() {
            Some(&'+') => {
                self.input.next();
                self.primary()
            },
            Some(&'-') => {
                let op = self.input.next().unwrap();
                let left = Node::new(NodeKind::Num(0), None, None);
                let right = self.primary();
                Node::new(NodeKind::Op(op), Node::link(left), Node::link(right))
            },
            _ => {
                self.primary()
            }
        }
    }

    fn primary(&mut self) -> Node {
        loop {
            match self.input.peek() {
                Some(&'(') => {
                    self.input.next();
                    let node = self.expr();
                    self.input.next();
                    return node
                },
                Some(&_digit @ '0'..='9') => {
                    let num = strtoi(&mut self.input);
                    return Node::new(NodeKind::Num(num), None, None)
                },
                Some(&' ') => {
                    self.input.next();
                },
                _ => {
                    panic!("factor error");
                }
            }
        }
    }
}
