use std::collections::HashSet;
use std::iter::Peekable;
use std::sync::atomic::{AtomicU32, Ordering};
use crate::tokenizer::Token;
use crate::tokenizer::TokenIterator;

pub fn compile(exp: &str) {

    let mut input = Input::new(exp);
    let nodes = input.tokenize();
    //println!("{:?}", node);

    println!("(module");
    for node in &nodes {
        node.gen_func();
    }
    println!("  (export \"main\" (func $main))");
    println!(")");

}


#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Default)]
enum NodeKind {
    #[default] Nop,
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
    LVar(String),
    Num(i32),
    Return,
    If,
    While,
    For,
    Block,
    Function(String),
    Call(String),
}

type Link = Option<Box<Node>>;

#[derive(Debug)]
#[derive(Default)]
struct Node {
    id: u32,
    kind: NodeKind,
    lhs: Link,
    rhs: Link,
    // "if" "(" cond ")" then "else" els
    // "while" "(" cond ")" body
    // "for" "(" init ";" cond ";" inc ")" body
    cond: Link,
    then: Link,
    els: Link,
    body: Link,
    init: Link,
    inc: Link,
    stmts: Vec<Link>,

    // Function
    params: Vec<String>,
    // Call
    args: Vec<Link>,
}

static NODE_COUNTER: AtomicU32 = AtomicU32::new(0);

fn node_id() -> u32 {
    NODE_COUNTER.fetch_add(1, Ordering::SeqCst)
}

impl Node {
    fn new(kind: NodeKind, lhs: Link, rhs: Link) -> Self {
        Self { id: node_id(), kind, lhs, rhs, ..Default::default() }
    }

    fn new_if(cond: Link, then: Link, els: Link) -> Self {
        Self { id: node_id(), kind: NodeKind::If, cond, then, els, ..Default::default() }
    }

    fn new_while(cond: Link, body: Link) -> Self {
        Self { id: node_id(), kind: NodeKind::While, cond, body, ..Default::default() }
    }

    fn new_for(init: Link, cond: Link, inc: Link, body: Link) -> Self {
        Self { id: node_id(), kind: NodeKind::For, init, cond, inc, body, ..Default::default() }
    }

    fn new_block(stmts: Vec<Link>) -> Self {
        Self { id: node_id(), kind: NodeKind::Block, stmts, ..Default::default() }
    }

    fn new_function(name: String, params: Vec<String>, body: Link) -> Self {
        Self { id: node_id(), kind: NodeKind::Function(name), params,  body, ..Default::default() }
    }

    fn new_call(name: String, args: Vec<Link>) -> Self {
        Self { id: node_id(), kind: NodeKind::Call(name), args, ..Default::default() }
    }

    fn link(node: Node) -> Link {
        Some(Box::new(node))
    }

    fn gen_func(&self) {
        let name = if let NodeKind::Function(name) = &self.kind { name } else {
            panic!("関数ではありません");
        };
        println!("  (func ${}", name);
        let mut param_set : HashSet<String> = HashSet::new();
        for param in self.params.iter() {
            println!("    (param ${} i32)", param);
            param_set.insert(param.to_string());
        }
        println!("    (result i32)");
        let mut vars : HashSet<String> = HashSet::new();
        self.gen_locals(&mut param_set, &mut vars);
        self.body.as_ref().unwrap().gen_block();
        println!("    i32.const 0");
        println!("  )");
    }

    fn gen_lval(&self) {
        match &self.lhs.as_ref().unwrap().kind {
            NodeKind::LVar(name) => {
                self.rhs.as_ref().unwrap().gen();
                // 変数に保存しつつ、スタックに残しておく
                println!("    local.tee ${}", name);
            },
            _ => {
                panic!("代入の左辺値が変数ではありません");
            }
        }
    }

    fn gen_if(&self) {
        self.cond.as_ref().unwrap().gen();
        println!("    (if");
        println!("      (then");
        self.then.as_ref().unwrap().gen_stmt();
        println!("      )");
        if let Some(els) = &self.els {
            println!("      (else");
            els.gen_stmt();
            println!("      )");
        }
        println!("    )");
    }

    fn gen_while(&self) {
        println!("    (block $block{}", self.id);
        println!("      (loop $loop{}", self.id);
        self.cond.as_ref().unwrap().gen();
        println!("        i32.const 0");
        println!("        i32.eq");
        println!("        br_if $block{}", self.id);
        self.body.as_ref().unwrap().gen_stmt();
        println!("        br $loop{}", self.id);
        println!("      )");
        println!("    )");
    }

    fn gen_for(&self) {
        if let Some(init) = &self.init {
            init.gen();
            println!("    drop");
        }
        println!("    (block $block{}", self.id);
        println!("      (loop $loop{}", self.id);
        if let Some(cond) = &self.cond {
            cond.gen();
            println!("        i32.const 0");
            println!("        i32.eq");
            println!("        br_if $block{}", self.id);
        }
        self.body.as_ref().unwrap().gen_stmt();
        if let Some(inc) = &self.inc {
            inc.gen();
            println!("        drop");
        }
        println!("        br $loop{}", self.id);
        println!("      )");
        println!("    )");
    }

    fn gen_block(&self) {
        for stmt in &self.stmts {
            stmt.as_ref().unwrap().gen_stmt();
        }
    }

    fn gen_call(&self) {
        let name = if let NodeKind::Call(name) = &self.kind { name } else {
            panic!("関数呼び出しではありません");
        };
        for arg in &self.args {
            arg.as_ref().unwrap().gen();
        }
        println!("    call ${}", name);
    }

    fn gen_stmt(&self) {
        match self.kind {
            NodeKind::If => self.gen_if(),
            NodeKind::While => self.gen_while(),
            NodeKind::For => self.gen_for(),
            NodeKind::Block => self.gen_block(),
            NodeKind::Return => self.gen_return(),
            _ => {
                self.gen();
                println!("    drop");
            }
        }
    }

    fn gen_return(&self) {
        self.lhs.as_ref().unwrap().gen();
        println!("    return");
    }

    // スタックに値を残す式
    fn gen(&self) {

        match self.kind {
            NodeKind::Assign => {
                self.gen_lval();
                return;
            },
            NodeKind::Call(_) => {
                self.gen_call();
                return;
            },
            _ => ()
        }

        if let Some(child) = &self.lhs {
            child.gen();
        }
        if let Some(child) = &self.rhs {
            child.gen();
        }
        match &self.kind {
            NodeKind::Num(num) => println!("    i32.const {}", num),
            NodeKind::Add =>
                println!("    i32.add"),
            NodeKind::Sub =>
                println!("    i32.sub"),
            NodeKind::Mult =>
                println!("    i32.mul"),
            NodeKind::Div =>
                println!("    i32.div_s"),
            NodeKind::Equal =>
                println!("    i32.eq"),
            NodeKind::NotEqual =>
                println!("    i32.ne"),
            NodeKind::GreaterThan =>
                println!("    i32.gt_s"),
            NodeKind::GreaterThanOrEqual =>
                println!("    i32.ge_s"),
            NodeKind::LessThan =>
                println!("    i32.lt_s"),
            NodeKind::LessThanOrEqual =>
                println!("    i32.le_s"),
            NodeKind::LVar(name) =>
                println!("    local.get ${}", name),
            _ => ()
        }
    }

    fn gen_locals(&self, params: &mut HashSet<String>, vars: &mut HashSet<String>) {
        match &self.kind {
            NodeKind::LVar(name) => {
                if !params.contains(name) && !vars.contains(name) {
                    vars.insert(name.to_string());
                    println!("    (local ${} i32)", name);
                }
            },
            NodeKind::Function(_) => {
                for param in &self.params {
                    let name = param.as_str();
                    if !params.contains(name) && !vars.contains(name) {
                        vars.insert(name.to_string());
                        println!("    (local ${} i32)", name);
                    }
                }
            }
            _ => ()
        }
        if let Some(child) = &self.lhs {
            child.gen_locals(params, vars);
        }
        if let Some(child) = &self.rhs {
            child.gen_locals(params, vars);
        }
        if let Some(child) = &self.init {
            child.gen_locals(params, vars);
        }
        if let Some(child) = &self.cond {
            child.gen_locals(params, vars);
        }
        if let Some(child) = &self.inc {
            child.gen_locals(params, vars);
        }
        if let Some(child) = &self.body {
            child.gen_locals(params, vars);
        }
        if let Some(child) = &self.then {
            child.gen_locals(params, vars);
        }
        if let Some(child) = &self.els {
            child.gen_locals(params, vars);
        }
        for stmt in &self.stmts {
            stmt.as_ref().unwrap().gen_locals(params, vars);
        }

    }
}

struct Input<'a> {
    token_iterator: Peekable<TokenIterator<'a>>,
}

/*
program    = func*
func       = ident "(" (ident ( "," ident)* )?  ")" "{" stmt* "}"
stmt       = "return" expr
           | expr ";"
           | if "(" expr ")" stmt ("else" stmt)?
           | while "(" expr ")" stmt
           | for "(" expr? ";" expr? ";" expr? ")" stmt
           | block
block      = "{" stmt* "}"
expr       = assign
assign     = equality ("=" assign)?
equality   = relational ("==" relational | "!=" relational)*
relational = add ("<" add | "<=" add | ">" add | ">=" add)*
add        = mul ("+" mul | "-" mul)*
mul        = unary ("*" unary | "/" unary)*
unary      = ("+" | "-")? primary
primary    = num
           | ident ("(" (expr ( "," expr)* )? ")")?
           | "(" expr ")"
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
            nodes.push(self.func());
        }
        nodes
    }

    fn func(&mut self) -> Node {
        match self.token_iterator.next() {
            Some(Token::Ident(func_name)) => {
                let mut params = Vec::new();

                self.expect(Token::Reserved("("));
                match self.token_iterator.next() {
                    Some(Token::Reserved(")")) => {}
                    Some(Token::Ident(param_name)) => {
                        params.push(param_name.to_string());
                        while self.token_iterator.peek() != Some(&Token::Reserved(")")) {
                            self.expect(Token::Reserved(","));
                            match self.token_iterator.next() {
                                Some(Token::Ident(param_name)) => {
                                    params.push(param_name.to_string());
                                }
                                _ => {
                                    panic!("関数のパラメータ宣言にエラーがあります")
                                }
                            }
                        }
                        self.token_iterator.next();
                    },
                    _ => {
                        panic!("関数のパラメータ宣言にエラーがあります")
                    }
                }
                let block = self.block();
                return Node::new_function(func_name.to_string(), params, Node::link(block));
            },
            _ => {
                panic!("関数宣言ではありません");
            }
        }
    }

    fn stmt(&mut self) -> Node {
        let node= match self.token_iterator.peek() {
            Some(Token::Return) => {
                self.token_iterator.next();
                let lhs = self.expr();
                Node::new(NodeKind::Return, Node::link(lhs),None)
            },
            Some(Token::If) => {
                self.token_iterator.next();
                self.expect(Token::Reserved("("));
                let cond = self.expr();
                self.expect(Token::Reserved(")"));
                let then = self.stmt();
                let els_link = match self.token_iterator.peek() {
                    Some(Token::Else) => {
                        self.token_iterator.next();
                        Node::link(self.stmt())
                    },
                    _ => None
                };
                return Node::new_if(Node::link(cond), Node::link(then), els_link);
            }
            Some(Token::While) => {
                self.token_iterator.next();
                self.expect(Token::Reserved("("));
                let cond = self.expr();
                self.expect(Token::Reserved(")"));
                let body = self.stmt();
                return Node::new_while(Node::link(cond), Node::link(body));
            }
            Some(Token::For) => {
                self.token_iterator.next();
                self.expect(Token::Reserved("("));
                let init_link = match self.token_iterator.peek() {
                    Some(Token::Reserved(";")) => {
                        self.token_iterator.next();
                        None
                    },
                    _ => {
                        let init = self.expr();
                        self.expect(Token::Reserved(";"));
                        Node::link(init)
                    }
                };
                let cond_link = match self.token_iterator.peek() {
                    Some(Token::Reserved(";")) => {
                        self.token_iterator.next();
                        None
                    },
                    _ => {
                        let cond = self.expr();
                        self.expect(Token::Reserved(";"));
                        Node::link(cond)
                    }
                };
                let inc_link = match self.token_iterator.peek() {
                    Some(Token::Reserved(")")) => {
                        self.token_iterator.next();
                        None
                    },
                    _ => {
                        let inc = self.expr();
                        self.expect(Token::Reserved(")"));
                        Node::link(inc)
                    }
                };
                let body = self.stmt();
                return Node::new_for(init_link, cond_link, inc_link,Node::link(body));
            }
            Some(Token::Reserved("{")) => {
                return self.block();
            }
            _ => {
                self.expr()
            }
        };
        self.expect(Token::Reserved(";"));
        node
    }

    fn block(&mut self) -> Node {
        self.expect(Token::Reserved("{"));
        let mut stmts:Vec<Link> = Vec::new();
        loop {
            if self.token_iterator.peek() == Some(&Token::Reserved("}")) {
                self.token_iterator.next();
                break;
            }
            let stmt = self.stmt();
            stmts.push(Node::link(stmt));
        }
        return Node::new_block(stmts);
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
                let name_str = name.to_string();
                self.token_iterator.next();
                match self.token_iterator.peek() {
                    Some(Token::Reserved("(")) => {
                        self.token_iterator.next();
                        let mut args = Vec::new();
                        match self.token_iterator.peek() {
                            Some(Token::Reserved(")")) => {}
                            _ => {
                                args.push(Node::link(self.expr()));
                                while self.token_iterator.peek() != Some(&Token::Reserved(")")) {
                                    self.expect(Token::Reserved(","));
                                    args.push(Node::link(self.expr()));
                                }
                            }
                        }
                        self.token_iterator.next();
                        return Node::new_call(name_str, args);
                    }
                    _ => {
                        return Node::new(NodeKind::LVar(name_str), None, None);
                    }
                }
            },
            _ => {
                panic!("factor error");
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

