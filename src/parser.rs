use std::fmt::Display;
pub type NodeId = usize; // use this for nodes ig?
macro_rules! usr_msg {
    (fmt: 'static &str, token: lexer::Token) => {
        println!("Parser Error: {} {}", fmt, token);
    };
}
use crate::lexer;

#[derive(Debug)]
pub enum ParserErr {
    Invalid(String),
    Expected(String),
}
#[derive(Debug)]
pub enum BinopKind {
    Add, Sub, Mlt, Div, Assign
}
#[derive(Debug)]
pub struct Binop {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub kind: BinopKind,
    pub id: NodeId,
}
#[derive(Debug)]
pub enum Type {
    Base(String),
    Pointer(Box<Type>),
}
#[derive(Debug)]
pub struct Symbol {
    pub symbol: String,
    pub id: NodeId,
}
#[derive(Debug)]
pub struct VarDec {
    pub s: Symbol,
    pub ty: Option<Type>,
    pub val: Option<Box<Expr>>,
}
#[derive(Debug)]
struct Number {
    pub str: String,
    pub id: NodeId,
}
#[derive(Debug)]
pub enum Expr {
    Binop(Binop),
    Symbol(Symbol),
    Number(Number),
    VarDec(VarDec),
}
#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    // if/else, fn dec, struct dec and what not
}
impl Expr { // get id. thanks claude!
    pub fn id(&self) -> NodeId {
        match self {
            Expr::Binop(b)   => b.id,
            Expr::Symbol(s)  => s.id,
            Expr::Number(n)  => n.id,
            Expr::VarDec(v)  => v.s.id,
        }
    }
}
#[derive(Debug)]
pub enum Root {
    Block(Vec<Stmt>),
}
impl Display for Root {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Root::Block(b) => {
                for s in b {
                    return write!(f, "block stmt: {:?}\n", s);
                }
            },
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Parser {
    lexer: lexer::Lexer,
    current_id: NodeId,
    pub root: Option<Root>,
}
impl Parser {
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
    fn parse_struct(&mut self) -> Option<Stmt> {
        None
    }
    fn parse_fn(&mut self) -> Option<Stmt> {
        None
    }
    fn parse_primary(&mut self) -> Result<Expr, ParserErr> {
        let t =  self.next();
        match t {
            lexer::Token::Ident(s) => {
                return Ok(Expr::Symbol(Symbol { symbol: s, id: self.id_next() }));
            },
            lexer::Token::Num(n) => {
                return Ok(Expr::Number(Number { str: n, id: self.id_next() }));
            }
            lexer::Token::Symbol(s) if s.eq("(") => {
                self.next();
                let expr = self.parse_expr()?;
                if let lexer::Token::Symbol(_) = self.current() {
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
    fn parse_binop(&mut self) -> Result<Expr, ParserErr> {
        let lhs = self.parse_primary()?;
        match self.current() {
            lexer::Token::Symbol(s) => {
                match s.as_str() {
                    "+" | "-" | "*" | "/" | "="  => {
                        self.next();
                        let rhs = self.parse_expr()?;
                        return Ok(Expr::Binop(Binop {
                                id: self.id_next(),
                                left: Box::new(lhs),
                                right: Box::new(rhs), 
                                kind: match s.as_str() {
                                    "+" => BinopKind::Add,
                                    "-" => BinopKind::Sub,
                                    "*" => BinopKind::Mlt,
                                    "/" => BinopKind::Div,
                                    "=" => BinopKind::Assign,
                                    _ => return Err(ParserErr::Invalid(String::from("Invalid symbol in binop?")))
                                }}));
                    },
                    ":=" => {
                        self.next();
                        match lhs {
                            Expr::Symbol(s) => {
                                let rhs = self.parse_expr()?;
                                return Ok(Expr::VarDec(VarDec {
                                    s: s,
                                    ty: Option::None,
                                    val: Option::Some(Box::new(rhs))
                                }));
                            }
                            _ =>
                                return Err(ParserErr::Invalid(String::from(
                                            "vardec target must be a symbol"))),
                        }
                    },
                    _=> return Ok(lhs),
                }
            }
            _ => return Ok(lhs),
        }
    }
    fn parse_expr(&mut self) -> Result<Expr, ParserErr> {
        self.parse_binop()
    }
    fn parse_tls(&mut self) -> Result<Vec<Stmt>, ParserErr> {
        let mut stmts: Vec<Stmt> = vec![];
        loop {
            let t = self.current();
            match t {
                lexer::Token::EOF => break,
                lexer::Token::Keyword(kw) => {
                    match kw.as_str() {
                        "fn" => stmts.push(self.parse_fn().unwrap()),
                        "struct" => stmts.push(self.parse_struct().unwrap()),
                        _ => panic!("unhandled/unknown kw {}", kw),
                    }
                },
                _ =>
                    stmts.push(Stmt::Expr(self.parse_expr()?)),
            }
            // consume semicolon
            if let lexer::Token::Symbol(s) = self.current() {
                if s == ";" {
                    self.next();
                }
            }
        }
        Ok(stmts)
    }
    fn id_next(&mut self) -> NodeId {
        self.current_id += 1;
        return self.current_id;
    }
    pub fn parse(lexer: lexer::Lexer) -> Self {
        let mut p: Parser = Parser{lexer,root:None, current_id:0};
        let root = p.parse_tls().unwrap_or_else(|err| panic!("Error in tls {:?}", err));
        p.root =Some(Root::Block(root));
        return p;
    }
}
impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
