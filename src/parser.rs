use std::fmt::{Debug, Display};
// type NodeId = usize; // use this for nodes ig?
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
// pub struct NodeId(usize);
pub struct StmtId(usize);
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct ExprId(usize);
use crate::lexer::{self, Keyword, Token};

#[derive(Debug)]
pub enum ParserErr {
    Invalid(String),
    Expected(String, Token),
}
#[derive(Clone,Debug)]
pub enum BinopKind {
    Add, Sub, Mlt, Div, Assign,
    AddAssign, SubAssign, MltAssign, DivAssign,
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
pub struct FnDecArg {
    name: String,
    ty: Type,
}
#[derive(Clone, Debug)]
pub struct FnDec {
    name: String,
    args: Vec<FnDecArg>,
    ret_ty: Option<Type>,
    body: Option<Block>,
}
#[derive(Clone, Debug)]
pub struct IfStmt {
    cond: ExprId,
    block: Block,
}
#[derive(Clone, Debug)]
pub enum Stmt {
    IfStmt(IfStmt),
    FnDec(FnDec),
    Expr(ExprId),
    // if/else, fn dec, struct dec and what not
}
impl Expr { // get id. thanks claude!
}
#[derive(Debug)]
pub struct Scope {
    pub stmts: Vec<StmtId>,
}
#[derive(Clone)]
pub struct Block {
    pub stmts: Vec<StmtId>,
}
impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fn Body")
    }
}

const DONT_PARSE_STRUCT_LITS: usize = 0b1;

#[derive(Debug)]
pub struct Parser {
    stmts: Vec<Stmt>,
    exprs: Vec<Expr>,
    lexer: lexer::Lexer,
    flags: usize,
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
    fn peek(&mut self) -> lexer::Token {
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
    fn parse_fn_dec_args(&mut self) -> Result<Vec<FnDecArg>, ParserErr> {
        Ok(vec![])
    }
    fn parse_block(&mut self) -> Result<Block,ParserErr> {
        let old = self.flags;
        self.flags &= !DONT_PARSE_STRUCT_LITS;
        self.expect("{")?;
        let mut stmts = Vec::<StmtId>::new();
        'fn_body: loop {
            match self.current() {
                lexer::Token::Symbol(s,_) if s == "}" => {
                    break 'fn_body;
                }
                _ => stmts.push(self.parse_stmt()?),
            }
        }
        self.expect("}")?;
        let fn_body = Block {stmts: stmts};
        self.flags = old;
        return Ok(fn_body);
    }
    fn expect_kw(&mut self, kw: Keyword) -> Result<Keyword,ParserErr>{
        match self.current() {
            Token::Keyword(k, _) =>{
                if k == kw {
                    self.next();
                    Ok(k)
                } else {
                    Err(ParserErr::Expected(String::from(
                                format!("keyword {:?}", kw)),self.current()))
                }
            }
            _=>Err(ParserErr::Expected(String::from(
                        format!("keyword {:?}", kw)),self.current())),
        }
    }
    fn is_kw(&mut self, kw: Keyword) -> bool {
        match self.current() {
            Token::Keyword(k, _) =>{
                if k == kw {
                    self.next();
                    true
                } else {
                    false
                }
            }
            _=> false
        }
    }
    fn parse_if_condition(&mut self) -> Result<ExprId, ParserErr> {
        let old = self.flags;
        self.flags &= DONT_PARSE_STRUCT_LITS;
        let r = self.parse_assignment_expr(); // anny assignment expr
        self.flags = old;
        r
    }
    fn parse_if_stmt(&mut self) -> Result<StmtId, ParserErr> {
        self.expect_kw(Keyword::If)?;
        let cond = self.parse_if_condition()?;
        let body = self.parse_block()?;
        let s = IfStmt {
            cond, block: body
        };
        Ok(self.new_stmt(Stmt::IfStmt(s)))
    }
    fn parse_stmt(&mut self) -> Result<StmtId, ParserErr> {
        match self.current() {
            lexer::Token::Keyword(lexer::Keyword::If, _) => {
                self.parse_if_stmt()
            },
            _ => self.parse_expr_stmt()
        }
    }
    fn parse_fn_dec(&mut self) -> Result<StmtId,ParserErr> {
        // make sure it's kw fn
        if !matches!(self.current(), Token::Keyword(lexer::Keyword::Fn, _)) {
            return Err(ParserErr::Expected(
                    String::from("keyword fn"), self.current()));
        }
        let _token = self.next(); // consume token
        let name = self.expect_ident()?;
        self.expect("(")?; // expect "(" args ")"
        let  args = if self.is_symbol(")") { // set to empty vector
            vec![]
        } else { self.parse_fn_dec_args()? };
        self.expect(")")?;
        let ret_ty: Option<Type> = match self.current() {
            Token::Symbol(s,_) if s == ":" => {
                self.next();
                Some(self.parse_type()?)
            },
            _=> None
        };

        // optional Body
        let body = match self.current() {
            Token::Symbol(s,_) if s == "{" => {
                Some(self.parse_block()?)
            },
            _=> None,
        };

        let fndec = FnDec {
            name, args, ret_ty, body
        };
        Ok(self.new_stmt(Stmt::FnDec(fndec)))
    }
    // consumes
    fn expect_ident(&mut self) -> Result<String,ParserErr>{
        match self.current() {
            Token::Ident(ident, _) =>{
                self.next(); Ok(ident)
            }
            _=>Err(ParserErr::Expected(String::from("ident"),self.current())),
        }
    }
    fn is_symbol(&mut self, s: &'static str) -> bool {
        if let Token::Symbol(o,_) = self.current() {
            if o.as_str() == s { return true; } // consume
            false
        } else {
            false
        }
    }
    fn expect(&mut self, s: &'static str) -> Result<(), ParserErr> {
        if let Token::Symbol(o,_) = self.current() {
            if o.as_str() == s { self.next(); return Ok(()); } // consume
            Err(ParserErr::Expected(String::from(s), self.current()))
        } else {
            Err(ParserErr::Invalid(String::from(
                        format!("Expected symbol {}", s))))
        }
    }
    fn parse_primary(&mut self) -> Result<ExprId, ParserErr> {
        let t =  self.current();
        match t {
            lexer::Token::Ident(s,_span) => {
                self.next();
                let e = Expr::Symbol(Symbol { symbol: s });
                Ok(self.new_expr(e))
            },
            lexer::Token::Num(n,_span) => {
                self.next();
                let e = Expr::Number(Number { str: n});
                Ok(self.new_expr(e))
            }
            lexer::Token::Symbol(s,_span) if s.eq("(") => {
                self.next();
                let expr = self.parse_assignment_expr()?;
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
                    String::from(format!("Invalid primary token  {}", t)))),

        }
    }
    fn parse_binop(&mut self) -> Result<ExprId, ParserErr> {
        let lhs = self.parse_primary()?;
        match self.current() {
            lexer::Token::Symbol(s,_span) => {
                match s.as_str() {
                    "+" | "-" | "*" | "/" => {
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
                    "=" | "+=" | "-=" | "*=" | "/=" => {
                        self.next();
                        let rhs = self.parse_expr()?;
                        let e = Expr::Binop(Binop {
                            left: lhs,
                            right: rhs, 
                            kind: match s.as_str() {
                                "+=" => BinopKind::AddAssign,
                                "-=" => BinopKind::SubAssign,
                                "*=" => BinopKind::MltAssign,
                                "/=" => BinopKind::DivAssign,
                                "=" => BinopKind::Assign,
                                _ => return Err(ParserErr::Invalid(
                                        String::from(
                                            "Invalid symbol in binop?")))
                            }});
                        Ok(self.new_expr(e))
                    }
                    _=> return Ok(lhs),
                }
            }
            _ => return Ok(lhs),
        }
    }
    fn is_ident(&mut self) -> bool {
        if matches!(self.current(), Token::Ident(_,_)) {
            true
        } else {
            false
        }
    }
    fn parse_expr(&mut self) -> Result<ExprId, ParserErr> {
        self.parse_binop()
    }
    fn parse_assignment_expr(&mut self) -> Result<ExprId, ParserErr> {
        if self.is_ident() {
            match self.peek() {
                lexer::Token::Symbol(s,_span) => {
                    match s.as_str() {
                        ":=" | ":" | "::" => { // vardec
                            let lhs = self.parse_primary()?;
                            return self.parse_vardec(lhs);
                        }
                        _=>  {}
                    }
                }
                _ => {},
            }
        }
        self.parse_expr()
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
                        let t = self.parse_type()?;
                        // "a: type = ..." or "a: type"
                        match self.current() {
                            // "a: type = ..."
                            lexer::Token::Symbol(next,_span)
                                if next.as_str() == "=" => {
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
        let s = Stmt::Expr(self.parse_assignment_expr()?);
        self.expect(";")?; // expect semicolon
        Ok(self.new_stmt(s))
    }
    fn parse_module_tls(&mut self) -> Result<Vec<StmtId>, ParserErr> {
        let mut stmts: Vec<StmtId> = vec![];
        loop {
            let t = self.current();
            match t {
                lexer::Token::EOF => break,
                lexer::Token::Keyword(kw,_span) => {
                    match kw {
                        lexer::Keyword::Fn => 
                            stmts.push(self.parse_fn_dec().unwrap()),
                            // _ => panic!("unhandled/unknown kw {:?}", kw),
                        _ => return Err(ParserErr::Invalid(
                                String::from(format!("invalid kw")))),
                    }
                },
                _ =>
                    return Err(ParserErr::Invalid(format!("Invalid token {:?}", t))),
                    // return Err(ParserErr::Invalid(String::from("Invalid Kw"))),
            }
        }
        Ok(stmts)
    }
    pub fn parse(lexer: lexer::Lexer) -> Self {
        let mut p: Parser = Parser{
            flags: 0,
            exprs: Vec::new(),
            stmts: Vec::new(),
            lexer,root:Scope{stmts:Vec::new()}};
        let root = match  p.parse_module_tls() {
            Ok(root) => root,
            Err(e) => match e {
                ParserErr::Invalid(invalid) => {
                    panic!("Invalid token: {}", invalid);
                }
                ParserErr::Expected(expected, got) => {
                    panic!("expected {}, got: {:?}" ,expected, got);
                }
            },
        };
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
