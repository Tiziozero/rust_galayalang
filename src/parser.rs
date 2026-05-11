use std::fmt::{Debug, Display};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct StmtId(usize);

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct ExprId(usize);

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct ModId(usize);

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct ItemId(usize);

use crate::lexer::{self, Keyword, Token};

#[derive(Debug)]
pub enum ParserErr {
    Invalid(String),
    Expected(String, Token),
}
#[derive(Clone,Debug)]
pub enum BinopKind {
    Add, Sub, Mlt, Div,
    Eq, Ne, Le, Ge, Gt, Lt,
}
#[derive(Clone,Debug)]
pub enum AssignmentKind {
    Assign,
    AddAssign, SubAssign, MltAssign, DivAssign,
}
#[derive(Clone,Debug)]
pub struct Assignment {
    pub left: ExprId,
    pub right: ExprId,
    pub kind: AssignmentKind,
}
#[derive(Clone,Debug)]
pub struct Binop {
    pub left: ExprId,
    pub right: ExprId,
    pub kind: BinopKind,
}
#[derive(Clone,Debug)]
pub enum TypeSpecifier {
    Base(String),
    Pointer(Box<TypeSpecifier>),
}
#[derive(Clone,Debug)]
pub struct Symbol {
    pub symbol: String,
}
#[derive(Clone, Debug)]
pub struct IfElseAltCond {
    pub cond: ExprId,
    pub block: Block,
}
#[derive(Clone,Debug)]
pub struct VarDec {
    pub s: String, // name
    pub ty: Option<TypeSpecifier>, // type if defied else infer
    pub val: Option<ExprId>, // optional initalisation value
}
#[derive(Clone,Debug)]
pub struct Number {
    pub str: String,
}
#[derive(Clone,Debug)]
pub struct FnCall {
    pub target: ExprId,
    pub args: Vec<ExprId>,
}
#[derive(Clone,Debug)]
pub enum Expr {
    Binop(Binop),
    Symbol(Symbol),
    Number(Number),
    FnCall(FnCall),
}

#[derive(Clone, Debug)]
pub struct FnDecArg {
    pub name: String,
    pub ty: TypeSpecifier,
}
#[derive(Clone, Debug)]
pub struct FnDec {
    pub name: String,
    pub args: Vec<FnDecArg>,
    pub ret_ty: Option<TypeSpecifier>,
    pub body: Option<Block>,
}
#[derive(Clone, Debug)]
pub struct IfStmt {
    pub cond: ExprId,
    pub block: Block,
    pub alt: Vec<IfElseAltCond>,
    pub else_block: Option<Block>,
}
#[derive(Clone, Debug)]
pub enum Stmt {
    IfStmt(IfStmt),
    VarDec(VarDec),
    Expr(ExprId),
    Assignment(Assignment),
    // if/else, fn dec, struct dec and what not
}

#[derive(Clone)]
pub struct Block {
    pub stmts: Vec<StmtId>,
}
impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Block")
    }
}

#[derive(Debug)]
pub enum Item {
    FnDec(FnDec),
}
#[derive(Debug)]
pub struct Module {
    pub items: Vec<ItemId>
}

#[derive(Debug)]
pub struct Parser {
    stmts:  Vec<Stmt>,
    exprs:  Vec<Expr>,
    items:  Vec<Item>,
    mods:   Vec<Module>,
    lexer: lexer::Lexer,
    flags: usize,
    pub root: ModId,
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
    fn new_module(&mut self, m: Module) -> ModId {
        self.mods.push(m);
        return ModId(self.mods.len() - 1);
    }
    fn new_item(&mut self, item: Item) -> ItemId {
        self.items.push(item);
        return ItemId(self.items.len() - 1);
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
    fn parse_fn_dec_args(&mut self) -> Result<Vec<FnDecArg>, ParserErr> {
        let mut args = Vec::<FnDecArg>::new();
        loop {
            let ident = self.expect_ident()?;
            self.expect(":")?;
            let ty = self.parse_type()?;
            args.push(FnDecArg { name: ident, ty });
            if !self.current().is_symbol(",") {
                break;
            }
            self.next(); // ","
        }
        Ok(args)
    }
    fn parse_block(&mut self) -> Result<Block,ParserErr> {
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
    fn parse_if_condition(&mut self) -> Result<ExprId, ParserErr> {
        self.parse_expr() // anny assignment expr
        
    }
    fn parse_if_stmt(&mut self) -> Result<StmtId, ParserErr> {
        self.expect_kw(Keyword::If)?;
        let cond = self.parse_if_condition()?;
        let body = self.parse_block()?;
        let mut alts = Vec::<IfElseAltCond>::new();
        'if_else_loop: loop {
            if self.current().is_kw(Keyword::Else)
                && self.peek().is_kw(Keyword::If) {
                self.next(); // "if"
                self.next(); // "else"
                let alt_cond = self.parse_if_condition()?;
                let alt_block = self.parse_block()?;
                let a = IfElseAltCond {cond: alt_cond, block: alt_block };
                alts.push(a);
            } else {
                break 'if_else_loop;
            }
        }
        let else_block = if self.current().is_kw(Keyword::Else) {
            self.next();
            Some(self.parse_block()?)
        } else { None };
        let s = IfStmt {
            cond, block: body,
            alt: alts, else_block,
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
    fn parse_fn_dec(&mut self) -> Result<ItemId,ParserErr> {
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
        let ret_ty: Option<TypeSpecifier> = match self.current() {
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
        Ok(self.new_item(Item::FnDec(fndec)))
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
            Err(ParserErr::Expected(String::from(s), self.current()))
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
                    String::from(format!("Invalid primary token  {}", t)))),

        }
    }
    fn parse_postfix(&mut self) -> Result<ExprId, ParserErr> {
        let mut primary = self.parse_primary()?;
        loop { match self.current() {
            Token::Symbol(s,_) if s == "(" => {
                self.next();
                let target = primary;
                // if there are args
                let args = if !self.current().is_symbol(")") {
                    println!("Expect args in fncall");
                    let mut args = Vec::<ExprId>::new();
                    loop {
                        args.push(self.parse_expr()?);
                        if !self.current().is_symbol(",") {
                            break;
                        }
                        self.next(); // ","
                    }
                    args
                } else {
                    Vec::<ExprId>::new()
                };
                self.expect(")")?;
                let e = Expr::FnCall(FnCall{target, args});
                primary = self.new_expr(e);
            },
            _ => break
        }
        }
        Ok(primary)
    }
    fn parse_binop(&mut self) -> Result<ExprId, ParserErr> {
        let lhs = self.parse_postfix()?;
        match self.current() {
            lexer::Token::Symbol(s,_span) => {
                match s.as_str() {
                    "+" | "-" | "*" | "/" | "<" | ">" |
                    "==" | "!=" | "<=" | ">="
                        => {
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
                                "<" => BinopKind::Lt,
                                ">" => BinopKind::Gt,
                                "==" => BinopKind::Eq,
                                "!=" => BinopKind::Ne,
                                "<=" => BinopKind::Le,
                                ">=" => BinopKind::Ge,
                                _ => return Err(ParserErr::Invalid(
                                        String::from(
                                            "Invalid symbol in binop?")))
                            }});
                        Ok(self.new_expr(e))
                    },
                    _ => {
                        return Ok(lhs);
                    },
                }
            }
            _ => {
                return Ok(lhs);
            },
        }
    }
    fn parse_expr(&mut self) -> Result<ExprId, ParserErr> {
        self.parse_binop()
    }
    pub fn get_expr(&self, id: ExprId) -> Result<&Expr, ParserErr> {
        self.exprs.get(id.0).ok_or(
            ParserErr::Invalid(format!("expr {:?} doesn't exist", id)))
    }
    pub fn get_module(&self, id: ModId) -> Result<&Module, ParserErr> {
        self.mods.get(id.0).ok_or(
            ParserErr::Invalid(format!("module {:?} doesn't exist", id)))
    }
    pub fn get_item(&self, id: ItemId) -> Result<&Item, ParserErr> {
        self.items.get(id.0).ok_or(
            ParserErr::Invalid(format!("item {:?} doesn't exist", id)))
    }

    pub fn get_stmt(&self, id: StmtId) -> Result<&Stmt, ParserErr> {
        self.stmts.get(id.0).ok_or(
            ParserErr::Invalid(format!("stmt {:?} doesn't exist", id)))
    }
    // horrible looking function
    fn parse_vardec(&mut self) -> Result<StmtId, ParserErr> {
        let ident = self.expect_ident()?;
        match self.current() {
            lexer::Token::Symbol(s,_span) => {
                match s.as_str() {
                    ":=" => { // "a := ..."
                        self.next();
                        let rhs = self.parse_expr()?;
                        let e = Stmt::VarDec(VarDec {
                            s: ident,
                            ty: Option::None,
                            val: Option::Some(rhs)
                        });
                        Ok(self.new_stmt(e))
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
                                    let e = Stmt::VarDec(VarDec{
                                        s: ident,
                                        ty: Some(t),
                                        val: Some(rhs),
                                    });
                                    Ok(self.new_stmt(e))
                                }
                            // "a: type"
                            _ => Ok(self.new_stmt(Stmt::VarDec(VarDec{
                                s: ident,
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
    fn parse_type(&mut self) -> Result<TypeSpecifier, ParserErr> {
        match self.current() {
            lexer::Token::Symbol(s,_span) => {
                match s.as_str() {
                    "*" => {
                        self.next();
                        return Ok(TypeSpecifier::Pointer(Box::new(self.parse_type()?)));
                    },
                    _ => panic!("invalid symbol in type"),
                }
            }
            lexer::Token::Ident(i,_span) => {
                self.next();
                return Ok(TypeSpecifier::Base(i));
            },
            _ => panic!("Handle"),
        }
    }
    fn parse_expr_stmt(&mut self) -> Result<StmtId, ParserErr> {
        if self.current().is_ident() && self.peek().is_vardec_symbol() {
            let r = self.parse_vardec()?;
            self.expect(";")?; // expect semicolon
            return Ok(r);
        }
        // could have index or field access as lvalues,
        // so check if it's an assignment
        let s = self.parse_expr()?;
        if self.current().is_assingment_symbol() {
            let kind = self.next();
            let assignment = Assignment {
                left: s,
                right: self.parse_expr()?,
                kind: if kind.is_symbol("=") {
                    AssignmentKind::Assign
                } else if kind.is_symbol("+=") {
                    AssignmentKind::AddAssign
                } else if kind.is_symbol("-=") {
                    AssignmentKind::SubAssign
                } else if kind.is_symbol("*=") {
                    AssignmentKind::MltAssign
                } else if kind.is_symbol("/=") {
                    AssignmentKind::DivAssign
                } else {
                    panic!("what");
                }
            };
            self.expect(";")?; // expect semicolon
            Ok(self.new_stmt(Stmt::Assignment(assignment)))
        } else {
            self.expect(";")?; // expect semicolon
            Ok(self.new_stmt(Stmt::Expr(s)))
        }
    }
    fn parse_module_tls(&mut self) -> Result<Vec<ItemId>, ParserErr> {
        let mut stmts: Vec<ItemId> = vec![];
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
            exprs:  Vec::new(),
            stmts:  Vec::new(),
            items:  Vec::new(),
            mods:   Vec::new(),
            lexer,
            root: ModId(0),
        };
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
        p.root = p.new_module(Module { items: root });
        return p;
    }
}
impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
