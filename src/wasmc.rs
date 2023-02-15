use std::fs::File;
use std::io::{stdout, Write};
use std::iter::Peekable;
use crate::ast::{Assign, AstNode, BiOperator, BiOpKind, Block, Call, ForNode, Function, IfNode, Module, Number, Param, ReturnNode, Variable, WasmWriter, WatWriter, WhileNode};
use crate::parser::Token;
use crate::parser::TokenIterator;

pub fn compile(exp: &str) {

    let mut input = Input::new(exp);
    let module = input.tokenize();

    //let _ = module.write_wat(&mut stdout());

    let mut wat_file = File::create("out.wat").unwrap();
    let _ = module.write_wat(&mut wat_file);
    let _ = module.write_wat(&mut stdout());
    let _ = wat_file.flush();


    let mut wasm_file = File::create("out.wasm").unwrap();
    let _ = module.write_wasm(None, None, &mut wasm_file);
    let _ = wasm_file.flush();
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

    fn tokenize(&mut self) -> Module {
        self.program()
    }

    fn program(&mut self) -> Module {
        let mut module = Module::new();
        while self.token_iterator.peek() != None {
            module.add_function(self.func());
        }
        module
    }

    fn func(&mut self) -> Function {
        match self.token_iterator.next() {
            Some(Token::Ident(func_name)) => {
                let mut params : Vec<Param> = Vec::new();
                self.expect(Token::Reserved("("));
                match self.token_iterator.next() {
                    Some(Token::Reserved(")")) => {}
                    Some(Token::Ident(param_name)) => {
                        params.push(Param::new(param_name.to_string()));
                        while self.token_iterator.peek() != Some(&Token::Reserved(")")) {
                            self.expect(Token::Reserved(","));
                            match self.token_iterator.next() {
                                Some(Token::Ident(param_name)) => {
                                    params.push(Param::new(param_name.to_string()));
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
                return Function::new(func_name.to_string(), params, Box::new(block));
            },
            _ => {
                panic!("関数宣言ではありません");
            }
        }
    }

    fn stmt(&mut self) -> Box<dyn AstNode> {
        let node : Box<dyn AstNode> = match self.token_iterator.peek() {
            Some(Token::Return) => {
                self.token_iterator.next();
                let lhs = self.expr();
                Box::new(ReturnNode::new(lhs))
            },
            Some(Token::If) => {
                self.token_iterator.next();
                self.expect(Token::Reserved("("));
                let cond = self.expr();
                self.expect(Token::Reserved(")"));
                let then = self.stmt();
                let els = match self.token_iterator.peek() {
                    Some(Token::Else) => {
                        self.token_iterator.next();
                        Some(self.stmt())
                    },
                    _ => None
                };
                return Box::new(IfNode::new(cond, then, els))
            }
            Some(Token::While) => {
                self.token_iterator.next();
                self.expect(Token::Reserved("("));
                let cond = self.expr();
                self.expect(Token::Reserved(")"));
                let body = self.stmt();
                return Box::new(WhileNode::new(cond, body));
            }
            Some(Token::For) => {
                self.token_iterator.next();
                self.expect(Token::Reserved("("));
                let init = match self.token_iterator.peek() {
                    Some(Token::Reserved(";")) => {
                        self.token_iterator.next();
                        None
                    },
                    _ => {
                        let init = self.expr();
                        self.expect(Token::Reserved(";"));
                        Some(init)
                    }
                };
                let cond = match self.token_iterator.peek() {
                    Some(Token::Reserved(";")) => {
                        self.token_iterator.next();
                        None
                    },
                    _ => {
                        let cond = self.expr();
                        self.expect(Token::Reserved(";"));
                        Some(cond)
                    }
                };
                let inc = match self.token_iterator.peek() {
                    Some(Token::Reserved(")")) => {
                        self.token_iterator.next();
                        None
                    },
                    _ => {
                        let inc = self.expr();
                        self.expect(Token::Reserved(")"));
                        Some(inc)
                    }
                };
                let body = self.stmt();
                return Box::new(ForNode::new(init, cond, inc, body));
            }
            Some(Token::Reserved("{")) => {
                return Box::new(self.block());
            }
            _ => {
                self.expr()
            }
        };
        self.expect(Token::Reserved(";"));
        node
    }

    fn block(&mut self) -> Block {

        let mut block = Block::new();

        self.expect(Token::Reserved("{"));
        loop {
            if self.token_iterator.peek() == Some(&Token::Reserved("}")) {
                self.token_iterator.next();
                break;
            }
            let stmt = self.stmt();
            block.add_statement(stmt);
        }
        return block;
    }

    fn expr(&mut self) -> Box<dyn AstNode> {
        return self.assign();
    }

    fn assign(&mut self) -> Box<dyn AstNode> {
        let mut node = self.equality();
        match self.token_iterator.peek() {
            Some(Token::Reserved("=")) => {
                self.token_iterator.next();
                let right = self.assign();
                node = Box::new(Assign::new(node, right));
            },
            _ => ()
        }
        node
    }

    fn equality(&mut self) -> Box<dyn AstNode> {
        let mut node = self.relational();

        loop {
            match self.token_iterator.peek() {
                Some(Token::Reserved("==")) => {
                    self.token_iterator.next();
                    let right = self.relational();
                    node = Box::new(BiOperator::new(BiOpKind::Equal, node, right));
                },
                Some(Token::Reserved("!=")) => {
                    self.token_iterator.next();
                    let right = self.relational();
                    node = Box::new(BiOperator::new(BiOpKind::NotEqual, node, right));
                },
                _ => {
                    break;
                }
            }
        }
        node
    }

    fn relational(&mut self) -> Box<dyn AstNode> {
        let mut node = self.add();
        loop {
            match self.token_iterator.peek() {
                Some(Token::Reserved(">=")) => {
                    self.token_iterator.next();
                    let right = self.add();
                    node = Box::new(BiOperator::new(BiOpKind::GreaterThanOrEqual, node, right));
                },
                Some(Token::Reserved(">")) => {
                    self.token_iterator.next();
                    let right = self.add();
                    node = Box::new(BiOperator::new(BiOpKind::GreaterThan, node, right));
                },
                Some(Token::Reserved("<=")) => {
                    self.token_iterator.next();
                    let right = self.add();
                    node = Box::new(BiOperator::new(BiOpKind::LessThanOrEqual, node, right));
                },
                Some(Token::Reserved("<")) => {
                    self.token_iterator.next();
                    let right = self.add();
                    node = Box::new(BiOperator::new(BiOpKind::LessThan, node, right));
                },
                _ => {
                    break;
                }
            }
        }
        node
    }
    fn add(&mut self) -> Box<dyn AstNode> {
        let mut node = self.mul();
        loop {
            match self.token_iterator.peek() {
                Some(Token::Reserved("+")) => {
                    self.token_iterator.next();
                    let right = self.mul();
                    node = Box::new(BiOperator::new(BiOpKind::Add, node, right));
                },
                Some(Token::Reserved("-")) => {
                    self.token_iterator.next();
                    let right = self.mul();
                    node = Box::new(BiOperator::new(BiOpKind::Sub, node, right));
                },
                _ => {
                    break;
                }
            }
        }
        node
    }

    fn mul(&mut self) -> Box<dyn AstNode> {
        let mut node = self.unary();
        loop {
            match self.token_iterator.peek() {
                Some(Token::Reserved("*")) => {
                    self.token_iterator.next();
                    let right = self.unary();
                    node = Box::new(BiOperator::new(BiOpKind::Mult, node, right));
                },
                Some(Token::Reserved("/")) => {
                    self.token_iterator.next();
                    let right = self.unary();
                    node = Box::new(BiOperator::new(BiOpKind::Div, node, right));
                },
                _ => {
                    break;
                }
            }
        }
        node
    }

    fn unary(&mut self) -> Box<dyn AstNode> {
        return match self.token_iterator.peek() {
            Some(Token::Reserved("+")) => {
                self.token_iterator.next();
                self.primary()
            },
            Some(Token::Reserved("-")) => {
                self.token_iterator.next();
                let left = Box::new(Number::new(0));
                let right = self.primary();
                Box::new(BiOperator::new(BiOpKind::Sub, left, right))
            },
            _ => {
                self.primary()
            }
        }
    }

    fn primary(&mut self) -> Box<dyn AstNode> {
        match self.token_iterator.peek() {
            Some(Token::Reserved("(")) => {
                self.token_iterator.next();
                let node = self.expr();
                self.expect(Token::Reserved(")"));
                return node
            },
            Some(Token::Num(num)) => {
                let node = Box::new(Number::new(*num));
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
                                args.push(self.expr());
                                while self.token_iterator.peek() != Some(&Token::Reserved(")")) {
                                    self.expect(Token::Reserved(","));
                                    args.push(self.expr());
                                }
                            }
                        }
                        self.token_iterator.next();
                        return Box::new(Call::new(name_str, args));
                    }
                    _ => {
                        return Box::new(Variable::new(name_str));
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
                    panic!("unexpected token {:?}", expected);
                }
                return token;
            },
            _ => {
                panic!("Invalid token stream");
            }
        }

    }
}

