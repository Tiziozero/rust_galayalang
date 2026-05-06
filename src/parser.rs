use std::{fmt::Display, thread::current};
// type NodeId = usize; // use this for nodes ig?
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
// pub struct NodeId(usize);
pub struct StmtId(usize);
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct ExprId(usize);
macro_rules! usr_msg {
    (fmt: 'static &str, token: lexer::Token) => {
        println!("Parser Error: {} {}", fmt, token);
    };
}
use crate::lexer::{self, Token};

#[derive(Debug)]
pub enum ParserErr {
    Invalid(String),
    Expected(String),
}
#[derive(Clone,Debug)]
pub enum BinopKind {
    Add, Sub, Mlt, Div, Assign
}
#[derive(Clone,Debug)]
pub struct Binop {
    pub left: ExprId,
    pub right: ExprId,
    pub kind: BinopKind,
}
#[derive(Clone,Debug)]
pub enum Type {
    Base(String),
    Pointer(Box<Type>),
}
#[derive(Clone,Debug)]
pub struct Symbol {
    pub symbol: String,
}
#[derive(Clone,Debug)]
pub struct VarDec {
    pub s: Symbol,
    pub ty: Option<Type>,
    pub val: Option<ExprId>,
}
#[derive(Clone,Debug)]
pub struct Number {
    pub str: String,
}
#[derive(Clone,Debug)]
pub enum Expr {
    Binop(Binop),
    Symbol(Symbol),
    Number(Number),
    VarDec(VarDec),
}
#[derive(Clone, Debug)]
pub enum Stmt {
    Expr(ExprId),
    // if/else, fn dec, struct dec and what not
}
impl Expr { // get id. thanks claude!
}
#[derive(Debug)]
pub struct Scope {
    pub stmts: Vec<StmtId>,
}


#[derive(Debug)]
pub struct Parser {
    stmts: Vec<Stmt>,
    exprs: Vec<Expr>,
    lexer: lexer::Lexer,
    pub root: Scope,
}
impl Parser {
    fn new_stmt(&mut self, stmt: Stmt) -> StmtId {
        self.stmts.push(stmt);
        return StmtId(self.stmts.len() - 1);
    }
    fn new_expr(&mut self, expr: Expr) -> ExprId {
        self.exprs.push(expr);
        return ExprId(self.exprs.len() - 1);
    }
    fn current(&mut self) -> lexer::Token {
        return if let Some(t) = self.lexer.current() {
            t.clone()
        } else {
            lexer::Token::EOF
        }
    }
    fn _peek(&mut self) -> lexer::Token {
        return if let Some(t) = self.lexer.peek() {
            t.clone()
        } else {
            lexer::Token::EOF
        }
    }
    fn next(&mut self) -> lexer::Token {
        return if let Some(t) = self.lexer.next() {
            t.clone()
        } else {
            lexer::Token::EOF
        }
    }
    fn _back(&mut self) {
        self.lexer.back();
    }
    fn parse_struct(&mut self) -> Result<StmtId,ParserErr> {
        panic!("Handle struct dec");
    }
    fn parse_fn(&mut self) -> Result<StmtId,ParserErr> {
        panic!("Handle");
    }
    fn expect(&mut self, s: &'static str) -> Result<(), ParserErr> {
        if let Token::Keyword(_,_) = self.current() {
            Err(ParserErr::Invalid(String::from("Exected symbol")))
        } else {
            Err(ParserErr::Invalid(String::from("Exected symbol")))
        }
    }
    fn parse_primary(&mut self) -> Result<ExprId, ParserErr> {
        let t =  self.next();
        match t {
            lexer::Token::Ident(s,_span) => {
                let e = Expr::Symbol(Symbol { symbol: s });
                Ok(self.new_expr(e))
            },
            lexer::Token::Num(n,_span) => {
                let e = Expr::Number(Number { str: n});
                Ok(self.new_expr(e))
            }
            lexer::Token::Symbol(s,_span) if s.eq("(") => {
                self.next();
                let expr = self.parse_expr()?;
                if let lexer::Token::Symbol(s,_) = self.current()
                    && s.as_str() == ")" {
                    self.next();
                    return Ok(expr);
                } else {
                    panic!("Expected {} got {}", ")",
                        self.current());
                }
            }
            t => return Err(ParserErr::Invalid(
                    String::from(format!("Invalid token primary  {}", t)))),

        }
    }
    fn parse_binop(&mut self) -> Result<ExprId, ParserErr> {
        let lhs = self.parse_primary()?;
        match self.current() {
            lexer::Token::Symbol(s,_span) => {
                match s.as_str() {
                    "+" | "-" | "*" | "/" | "="  => {
                        self.next();
                        let rhs = self.parse_expr()?;
                        let e = Expr::Binop(Binop {
                                left: lhs,
                                right: rhs, 
                                kind: match s.as_str() {
                                    "+" => BinopKind::Add,
                                    "-" => BinopKind::Sub,
                                    "*" => BinopKind::Mlt,
                                    "/" => BinopKind::Div,
                                    "=" => BinopKind::Assign,
                                    _ => return Err(ParserErr::Invalid(
                                            String::from(
                                                "Invalid symbol in binop?")))
                                }});
                        Ok(self.new_expr(e))
                    },
                    ":=" | ":" | "::" => return self.parse_vardec(lhs),
                    _=> return Ok(lhs),
                }
            }
            _ => return Ok(lhs),
        }
    }
    pub fn get_expr(&self, id: ExprId) -> Result<&Expr, ParserErr> {
        self.exprs.get(id.0).ok_or(
            ParserErr::Invalid(format!("expr {:?} doesn't exist", id)))
    }

    pub fn get_stmt(&self, id: StmtId) -> Result<&Stmt, ParserErr> {
        self.stmts.get(id.0).ok_or(
            ParserErr::Invalid(format!("stmt {:?} doesn't exist", id)))
    }
    fn parse_vardec(&mut self, lhs: ExprId) -> Result<ExprId, ParserErr> {
        let symbol: Symbol;
        match self.get_expr(lhs)? {
            Expr::Symbol(s) => symbol = s.clone(),
            _ => {
                return Err(ParserErr::Invalid(String::from(
                            "vardec lhs must be a symbol.")));
            }
        }
        match self.current() {
            lexer::Token::Symbol(s,_span) => {
                match s.as_str() {
                    ":=" => { // "a := ..."
                        self.next();
                        let rhs = self.parse_expr()?;
                        let e = Expr::VarDec(VarDec {
                            s: symbol,
                            ty: Option::None,
                            val: Option::Some(rhs)
                        });
                        Ok(self.new_expr(e))
                    },
                    ":" => { // "a : type..."
                        println!("{}|{}", self.next(), self.current()); // consume token
                        let t = self.parse_type()?;
                        // "a: type = ..." or "a: type"
                        println!("{}", self.current()); // consume token
                        match self.current() {
                            // "a: type = ..."
                            lexer::Token::Symbol(next,_span)
                                if next.as_str() == "=" => {
                                println!("Vardec with type and value");
                                self.next();
                                let rhs = self.parse_expr()?;
                                let e = Expr::VarDec(VarDec{
                                    s: symbol,
                                    ty: Some(t),
                                    val: Some(rhs),
                                });
                                Ok(self.new_expr(e))
                            }
                            // "a: type"
                            _ => Ok(self.new_expr(Expr::VarDec(VarDec{
                                    s: symbol,
                                    ty: Some(t),
                                    val: None,
                                }))),
                        }
                    },
                    _ => panic!("Handle"),
                }
            },
            _=>panic!("expected vardec symbol"),
        }
    }
    fn parse_expr(&mut self) -> Result<ExprId, ParserErr> {
        self.parse_binop()
    }
    fn parse_type(&mut self) -> Result<Type, ParserErr> {
        match self.current() {
            lexer::Token::Symbol(s,_span) => {
                match s.as_str() {
                    "*" => {
                        self.next();
                        return Ok(Type::Pointer(Box::new(self.parse_type()?)));
                    },
                    _ => panic!("invalid symbol in type"),
                }
            }
            lexer::Token::Ident(i,_span) => {
                self.next();
                return Ok(Type::Base(i));
            },
            _ => panic!("Handle"),
        }
    }
    fn parse_expr_stmt(&mut self) -> Result<StmtId, ParserErr> {
        let s =Stmt::Expr(self.parse_expr()?);
        Ok(self.new_stmt(s))
    }
    fn parse_tls(&mut self) -> Result<Vec<StmtId>, ParserErr> {
        let mut stmts: Vec<StmtId> = vec![];
        loop {
            let t = self.current();
            match t {
                lexer::Token::EOF => break,
                lexer::Token::Keyword(kw,_span) => {
                    match kw.as_str() {
                        "fn" => stmts.push(self.parse_fn().unwrap()),
                        // "struct" => stmts.push(self.parse_struct().unwrap()),
                        _ => panic!("unhandled/unknown kw {}", kw),
                    }
                },
                _ =>
                    stmts.push(self.parse_expr_stmt()?),
                    // return Err(ParserErr::Invalid(String::from("Invalid Kw"))),
            }
            // consume semicolon
            if let lexer::Token::Symbol(s,_span) = self.current() {
                if s == ";" {
                    self.next();
                }
            }
        }
        Ok(stmts)
    }
    pub fn parse(lexer: lexer::Lexer) -> Self {
        let mut p: Parser = Parser{
            exprs: Vec::new(),
            stmts: Vec::new(),
            lexer,root:Scope{stmts:Vec::new()}};
        let root = p.parse_tls().unwrap_or_else(|err| panic!("Error in tls {:?}", err));
        println!("AST: {:?}", root);
        p.root=Scope { stmts: root };
        return p;
    }
}
impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
