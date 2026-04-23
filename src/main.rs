use std::fs::read_to_string;


#[derive(Debug)]
enum ParserErr {
    Invalid(String),
    Expected(String),
}

mod lexer;
enum BinopKind {
    Add, Sub, Mlt, Div
}
struct Binop {
    left: Box<Expr>,
    right: Box<Expr>,
    kind: BinopKind,
}
enum Type {
    Base(String),
    Pointer(Box<Type>),
}
struct Symbol {
    identifier: String,
}
enum Expr {
    Binop(Binop),
    Symbol(String),
    Number(String),
}
enum Stmt {
    Expr(Expr),
    // if/else, fn dec, struct dec and what not
}
enum Root {
    Block(Vec<Stmt>),
}

struct Parser {
    lexer: lexer::Lexer,
    root: Option<Root>,
}
impl Parser {
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
    fn back(&mut self) {
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
                return Ok(Expr::Symbol(s));
            },
            lexer::Token::Num(n) => {
                return Ok(Expr::Number(n));
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
            t => panic!("Invalid token primary  {}", t),

        }
    }
    fn parse_binop(&mut self) -> Result<Expr, ParserErr> {
        let lhs = self.parse_primary()?;
        match self.current() {
            lexer::Token::Symbol(s) => {
                match s.as_str() {
                    "+" => {
                        self.next();
                        let rhs = self.parse_expr()?;
                        return Ok(Expr::Binop(Binop {
                                left: Box::new(lhs),
                                right: Box::new(rhs), 
                                kind: BinopKind::Add }));
                    },
                    "-" => {
                        self.next();
                        let rhs = self.parse_expr()?;
                        return Ok(Expr::Binop(Binop {
                                left: Box::new(lhs),
                                right: Box::new(rhs), 
                                kind: BinopKind::Add }));
                    },
                    _=> return Err(ParserErr::Expected(String::from("operation"))),
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
        }
        Ok(stmts)
    }
    fn parse(lexer: lexer::Lexer) -> Self {
        let mut p: Parser = Parser{lexer,root:None};
        let root = p.parse_tls().unwrap_or_else(|err| panic!("Error in tls {:?}", err));
        p.root =Some(Root::Block(root));
        return p;
    }
}

fn main()  {
    let code = read_to_string("main.gala").unwrap();
    let lexer = lexer::Lexer::from_code(&code).unwrap_or_else(|err| {
        panic!("error {}", err);});
    println!("end");
    let _ = Parser::parse(lexer); // takes ownership
}


