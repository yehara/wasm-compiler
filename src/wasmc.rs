use std::collections::HashSet;
use std::sync::atomic::{AtomicU32, Ordering};
use crate::parser::Input;

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
pub enum NodeKind {
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

pub type Link = Option<Box<Node>>;

#[derive(Debug)]
#[derive(Default)]
pub struct Node {
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
    pub(crate) fn new(kind: NodeKind, lhs: Link, rhs: Link) -> Self {
        Self { id: node_id(), kind, lhs, rhs, ..Default::default() }
    }

    pub fn new_if(cond: Link, then: Link, els: Link) -> Self {
        Self { id: node_id(), kind: NodeKind::If, cond, then, els, ..Default::default() }
    }

    pub fn new_while(cond: Link, body: Link) -> Self {
        Self { id: node_id(), kind: NodeKind::While, cond, body, ..Default::default() }
    }

    pub fn new_for(init: Link, cond: Link, inc: Link, body: Link) -> Self {
        Self { id: node_id(), kind: NodeKind::For, init, cond, inc, body, ..Default::default() }
    }

    pub fn new_block(stmts: Vec<Link>) -> Self {
        Self { id: node_id(), kind: NodeKind::Block, stmts, ..Default::default() }
    }

    pub fn new_function(name: String, params: Vec<String>, body: Link) -> Self {
        Self { id: node_id(), kind: NodeKind::Function(name), params,  body, ..Default::default() }
    }

    pub fn new_call(name: String, args: Vec<Link>) -> Self {
        Self { id: node_id(), kind: NodeKind::Call(name), args, ..Default::default() }
    }

    pub fn link(node: Node) -> Link {
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

