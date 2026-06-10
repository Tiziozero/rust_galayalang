use std::fmt::{Debug, Display};
use crate::resolver;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct StmtId(usize);
impl StmtId {
    pub fn new(n: usize) -> Self {
        Self(n)
    }
    pub fn id(&self) -> usize {
        self.0
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct ExprId(usize);
impl ExprId {
    pub fn new(n: usize) -> Self {
        Self(n)
    }
    pub fn id(&self) -> usize {
        self.0
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct ModId(usize);
impl ModId {
    pub fn new(n: usize) -> Self {
        Self(n)
    }
    pub fn id(&self) -> usize {
        self.0
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct ItemId(usize);
impl ItemId {
    pub fn new(n: usize) -> Self {
        Self(n)
    }
    pub fn id(&self) -> usize {
        self.0
    }
}

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
pub struct StructField{
    pub name: String,
    pub ty: TypeSpecifier,
}
impl Expr {
    pub fn is_lvalue(&self) -> bool {
        match self {
            Expr::Symbol(_) => true,
            _ => false,
        }
   }
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
impl TypeSpecifier {
    pub fn name(&self) -> String {
        match self {
            Self::Base(s) => s.clone(),
            Self::Pointer(b) => format!("*{}", b.name()),
        }
    }
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
    StructLit(StructLit),
    FieldAccess(FieldAccess),
}
#[derive(Clone,Debug)]
pub struct FieldAccess {
    pub target: ExprId,
    pub field_name: String,
}
#[derive(Clone, Debug)]
pub struct StructLit {
    pub ty: String,
    pub fields: Vec<StructLitField>,
}
#[derive(Clone, Debug)]
pub struct StructLitField {
    pub name: String,
    pub expr: ExprId,
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
    Return(ExprId),
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

#[derive(Debug,Clone)]
pub enum Item {
    FnDec(FnDec),
    StructDec(StructDec),
}
#[derive(Debug,Clone)]
pub struct StructDec {
    pub name: String,
    pub fields: Vec<StructField>,
}
#[derive(Debug,Clone)]
pub struct Module {
    pub items: Vec<ItemId>
}

#[derive()]
pub struct Parser<'ctx> {
    ctx: &'ctx mut resolver::Context,
    lexer: lexer::Lexer,
    parse_struct_lit: bool,
    pub root: ModId,
}
impl<'ctx> Parser<'ctx> {
    fn new_stmt(&mut self, stmt: Stmt, span: lexer::Span) -> StmtId {
        self.ctx.new_stmt(stmt, span)
    }
    fn new_expr(&mut self, expr: Expr, span: lexer::Span) -> ExprId {
        self.ctx.new_expr(expr, span)
    }
    fn new_module(&mut self, m: Module, span: lexer::Span) -> ModId {
        self.ctx.new_mod(m, span)
    }
    fn new_item(&mut self, item: Item, span: lexer::Span) -> ItemId {
        self.ctx.new_item(item, span)
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
            let ty = self.parse_type_specifier()?;
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
        let prev = self.parse_struct_lit;
        self.parse_struct_lit = false;
        let expr = self.parse_expr(); // anny assignment expr
        self.parse_struct_lit = prev;
        expr
    }
    fn get_current_span(&mut self) -> lexer::Span {
        match self.current() {
            lexer::Token::Symbol(_,s) => s,
            lexer::Token::Keyword(_,s) => s,
            lexer::Token::Ident(_,s) => s,
            lexer::Token::Num(_,s) => s,
            lexer::Token::EOF => panic!("what"),
        }
    }
    fn get_prev_span(&mut self) -> lexer::Span {
        
        match self.lexer.prev().unwrap().clone() {
            lexer::Token::Symbol(_,s) => s,
            lexer::Token::Keyword(_,s) => s,
            lexer::Token::Ident(_,s) => s,
            lexer::Token::Num(_,s) => s,
            lexer::Token::EOF => panic!("what"),
        }
    }
    fn parse_return_stmt(&mut self) -> Result<StmtId, ParserErr> {
        let span = self.get_current_span();
        self.expect_kw(Keyword::Return)?;
        let expr = self.parse_expr()?;
        self.expect(";")?;
        Ok(self.new_stmt(Stmt::Return(expr), span))
    }
    fn parse_if_stmt(&mut self) -> Result<StmtId, ParserErr> {
        self.expect_kw(Keyword::If)?;
        let span = self.get_prev_span();
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

        Ok(self.new_stmt(Stmt::IfStmt(s), span))
    }
    fn parse_stmt(&mut self) -> Result<StmtId, ParserErr> {
        match self.current() {
            lexer::Token::Keyword(lexer::Keyword::If, _) => {
                self.parse_if_stmt()
            },
            lexer::Token::Keyword(lexer::Keyword::Return, _) => {
                self.parse_return_stmt()
            },
            _ => self.parse_expr_stmt()
        }
    }
    fn parse_struct_dec(&mut self) -> Result<ItemId,ParserErr> {
        let span = self.get_current_span();
        self.expect_kw(lexer::Keyword::Struct)?;
        let name = self.expect_ident()?;
        self.expect("{")?;
        let mut fields = Vec::<StructField>::new();
        while !self.current().is_symbol("}") {
            let field_name = self.expect_ident()?;
            self.expect(":")?;
            let ty = self.parse_type_specifier()?;
            fields.push(StructField{name:field_name, ty });
            if self.current().is_symbol(",") {
                self.next();
            }
        }
        self.expect("}")?;
        Ok(self.new_item(Item::StructDec(StructDec { name, fields }),span))
    }
    fn parse_fn_dec(&mut self) -> Result<ItemId,ParserErr> {
        // make sure it's kw fn
        self.expect_kw(Keyword::Fn)?;
        let span = self.get_prev_span();

        let name = self.expect_ident()?;
        self.expect("(")?; // expect "(" args ")"
        let  args = if self.is_symbol(")") { // set to empty vector
            vec![]
        } else { self.parse_fn_dec_args()? };
        self.expect(")")?;
        let ret_ty: Option<TypeSpecifier> = match self.current() {
            Token::Symbol(s,_) if s == ":" => {
                self.next();
                Some(self.parse_type_specifier()?)
            },
            _=> None
        };

        // optional Body
        let body = match self.current() {
            Token::Symbol(s,_) if s == "{" => {
                Some(self.parse_block()?)
            },
            _=> panic!("Need body"),
        };

        let fndec = FnDec {
            name, args, ret_ty, body
        };
        Ok(self.new_item(Item::FnDec(fndec), span))
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
        let span = self.get_current_span();
        match t {
            lexer::Token::Ident(s,_span) => {
                self.next();

                // struct lit
                if self.current().is_symbol("{") && self.parse_struct_lit {
                    self.next();
                    let mut fields = Vec::<StructLitField>::new();
                    'arg_loop: while self.current().is_ident() {
                        let name = self.expect_ident()?;
                        self.expect("=")?;
                        let expr = self.parse_expr()?;
                        fields.push(StructLitField { name, expr });
                        if !self.current().is_symbol(",") {
                            break 'arg_loop;
                        } else {
                            self.next(); // consume ","
                        }
                    }
                    self.expect("}")?;
                    Ok(self.new_expr(Expr::StructLit(StructLit{
                        ty: s.clone(), fields }), span))
                } else {
                let e = Expr::Symbol(Symbol { symbol: s });
                    Ok(self.new_expr(e, span))
                }
            },
            lexer::Token::Num(n,_span) => {
                self.next();
                let e = Expr::Number(Number { str: n});
                Ok(self.new_expr(e, span))
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
        let span = self.get_current_span();
        let mut primary = self.parse_primary()?;
        loop {
            match self.current() {
            Token::Symbol(s,_) if s == "." => {
                self.next(); // "."
                let target = primary;
                let field_name = self.expect_ident()?;
                return Ok(self.new_expr(Expr::FieldAccess(FieldAccess { target, field_name }), span));
            },
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
                primary = self.new_expr(e,span.clone());
            },
            _ => break
        }
        }
        Ok(primary)
    }
    fn parse_binop(&mut self) -> Result<ExprId, ParserErr> {
        let span = self.get_current_span();
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
                        Ok(self.new_expr(e,span))
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
        Ok(self.ctx.get_expr(id).unwrap())
    }
    pub fn get_module(&self, id: ModId) -> Result<&Module, ParserErr> {
        Ok(self.ctx.get_module(id).unwrap())
    }
    pub fn get_item(&self, id: ItemId) -> Result<&Item, ParserErr> {
        Ok(self.ctx.get_item(id).unwrap())
    }

    pub fn get_stmt(&self, id: StmtId) -> Result<&Stmt, ParserErr> {
        Ok(self.ctx.get_stmt(id).unwrap())
    }
    // horrible looking function
    fn parse_vardec(&mut self) -> Result<StmtId, ParserErr> {
        let span = self.get_current_span();
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
                        Ok(self.new_stmt(e, span))
                    },
                    ":" => { // "a : type..."
                        self.next(); // ":"
                        let t = self.parse_type_specifier()?;
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
                                    Ok(self.new_stmt(e, span))
                                }
                            // "a: type"
                            _ => /*Ok(self.new_stmt(Stmt::VarDec(VarDec{
                                s: ident,
                                ty: Some(t),
                                val: None,
                            })))*/ panic!("Must have assignment, can't have just dec (\"a : type;\" is illegal)"),
                        }
                    },
                    _ => panic!("Handle"),
                }
            },
            _=>panic!("expected vardec symbol"),
        }
    }
    fn parse_type_atomic(&mut self) -> Result<TypeSpecifier, ParserErr> {
        match self.current() {
            lexer::Token::Ident(s,_) => {
                self.next(); // consume
                return Ok(TypeSpecifier::Base(s.clone()))
            }
            _ => panic!("Invalid type atomic")
        }
    }
    fn parse_type_specifier(&mut self) -> Result<TypeSpecifier, ParserErr> {
        match self.current() {
            lexer::Token::Symbol(s,_span) => {
                match s.as_str() {
                    "*" => {
                        self.next();
                        return Ok(TypeSpecifier::Pointer(Box::new(self.parse_type_specifier()?)));
                    },
                    _ => panic!("invalid symbol in type"),
                }
            }
            lexer::Token::Ident(_,_span) => {
                self.parse_type_atomic()
            },
            _ => panic!("Handle"),
        }
    }
    fn parse_expr_stmt(&mut self) -> Result<StmtId, ParserErr> {
        let span = self.get_current_span();
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
            Ok(self.new_stmt(Stmt::Assignment(assignment), span))
        } else {
            self.expect(";")?; // expect semicolon
            Ok(self.new_stmt(Stmt::Expr(s), span))
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
                        lexer::Keyword::Struct =>
                            stmts.push(self.parse_struct_dec().unwrap()),
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
    pub fn parse(ctx: &'ctx mut resolver::Context, lexer: lexer::Lexer) -> ModId {
        let mut p: Parser = Parser{
            ctx,
            parse_struct_lit: true,
            lexer,
            root: ModId(0),
        };
        let span = p.get_current_span();
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
        p.root = p.new_module(Module { items: root }, span);
        return p.root;
    }
    fn peinvalid(&self, s: String, exprid: ExprId) -> ParserErr {
        let expr = self.ctx.get_expr(exprid).unwrap();
        let exprref = self.ctx.get_expr_ref(exprid);
        let span = self.ctx.exprs_span.get(exprid.0).unwrap();
        let err = match exprref {
            Some(r) => format!("{}: {:?} at {:?}", s, r, span),
            None => format!("{}: {:?} at {:?}", s, expr, span),
        };
        ParserErr::Invalid(err)
    }
    fn peinvalid_stmt(&self, s: String, stmtid: StmtId) -> ParserErr {
        let stmt = self.ctx.get_stmt(stmtid).unwrap();
        let stmtref = self.ctx.get_vardec_ref(stmtid);
        let span = self.ctx.stmts_span.get(stmtid.0).unwrap();
        let err = match stmtref {
            Some(r) => format!("{}: {:?} at {:?}", s, r, span),
            None => format!("{}: {:?} at {:?}", s, stmt, span),
        };
        ParserErr::Invalid(err)
    }
    fn peinvalid_item(&self, s: String, itemid: ItemId) -> ParserErr {
        let item = self.ctx.get_item(itemid).unwrap();
        let span = self.ctx.items_span.get(itemid.0).unwrap();
        let err = match (self.ctx.get_item_fn_ref(itemid), self.ctx.get_item_ty_ref(itemid)) {
            (Some(r), _) => format!("{}: {:?} at {:?}", s, r, span),
            (_, Some(r)) => format!("{}: {:?} at {:?}", s, r, span),
            (None, None) => format!("{}: {:?} at {:?}", s, item, span),
        };
        ParserErr::Invalid(err)
    }
}
impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
