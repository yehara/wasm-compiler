use std::iter::Peekable;
use crate::tokenizer::Token;
use crate::tokenizer::TokenIterator;
use crate::wasmc::{Link, Node, NodeKind};

pub struct Input<'a> {
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
    pub fn new(input: &'a str) -> Self {
        Self { token_iterator: TokenIterator { s: input }.peekable() }
    }

    pub fn tokenize(&mut self) -> Vec<Node> {
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

