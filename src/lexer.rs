use std::{fmt};
#[derive(Clone, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub col: usize,
}
impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "l{}c{}",self.line, self.col)
    }
}
fn span(s:usize,e:usize,l:usize,c:usize)->Span {
    Span{start:s, end:e, line:l, col:c}
}
#[derive(Debug,Clone, PartialEq)]
pub enum Keyword {
    Fn, If, Else, Return, Struct,
}
#[derive(Clone)]
pub enum Token {
    Ident(String,Span),
    Num(String,Span),
    Keyword(Keyword,Span),
    Symbol(String,Span), // can be double like "+="
    EOF,
}
impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match self {
            _ => panic!("handle token cmp {:?} {:?}", self, other)
        }
    }
    fn ne(&self, other: &Self) -> bool {
        match self {
            _ => panic!("handle token cmp {:?} {:?}", self, other)
        }
    }
}
#[derive(Debug,Clone)]
pub struct Lexer {
    tokens: Vec<Token>,
    index: usize,
    chars: Vec<char>,
    c_index: usize,
    line: usize, column: usize,
}
impl Lexer {
    pub fn peek(&mut self) -> Option<&Token> {
        if self.index < self.tokens.len() as usize -1 {
            return Some(&self.tokens[self.index as usize +1]);
        } else {
            return None;
        }
    }
    pub fn current(&mut self) -> Option<&Token> {
        if self.index > self.tokens.len() as usize - 1 {
            return None;
        }
        let t = &self.tokens[self.index as usize];
        return Some(t);
    }
    pub fn next(&mut self) -> Option<&Token> {
        if self.index > self.tokens.len() as usize - 1 {
            return None;
        }
        let t = &self.tokens[self.index as usize];
        self.index += 1;
        return Some(t);
    }
}
#[derive(Debug)]
pub enum LexerErr {
    // Fail(String),
    Invalid(String),
}
impl fmt::Display for LexerErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexerErr::Invalid(s) => write!(f, "{}", s),
        }
    }
}
impl Lexer {
    fn c_advance(&mut self) -> char {
        let c = self.chars[self.c_index];
        self.column += 1;
        self.c_index+= 1;
        c
    }
    fn c_current(&mut self) -> Option<char> {
        if let Some(c) = self.chars.get(self.c_index) {
            Some(*c)
        } else {
            None
        }
    }
    fn lexe_ident(&mut self) -> Result<(), LexerErr>  {
        let start = self.c_index;
        let line = self.line;
        let column = self.column;
        let mut name = String::new();
        while let Some(c) = self.c_current() {
            match c {
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                    name.push(c);
                    self.c_advance();
                }
                _ => break
            }
        }
        match name.as_str() {
            "fn" =>
                self.tokens.push(Token::Keyword(Keyword::Fn,
                    span(start, self.c_index, line,column))),
            "if" =>
                self.tokens.push(Token::Keyword(Keyword::If,
                    span(start, self.c_index, line,column))),
            "else" =>
                self.tokens.push(Token::Keyword(Keyword::Else,
                    span(start, self.c_index, line,column))),
            "otherwise" =>
                self.tokens.push(Token::Keyword(Keyword::Else,
                    span(start, self.c_index, line,column))),
            "return" => 
                self.tokens.push(Token::Keyword(Keyword::Return,
                    span(start, self.c_index, line,column))),
            "struct" => 
                self.tokens.push(Token::Keyword(Keyword::Struct,
                    span(start, self.c_index, line,column))),
            _ => 
                self.tokens.push(Token::Ident(name,
                    span(start, self.c_index, line,column))),
        }
        Ok(())
    }
    fn lexe_number(&mut self) -> Result<(), LexerErr>  {
        let start = self.c_index;
        let line = self.line;
        let column = self.column;
        let mut num = String::new();
        while let Some(c) = self.c_current() {
            match c {
                '0'..='9'=>
                    num.push(c),
                _ => break
            }
            self.c_advance();
        }
        self.tokens.push(Token::Num(num,
                    span(start, self.c_index, line,column)));
        Ok(())
    }
    fn lexe_symbol(&mut self) -> Result<(), LexerErr>  {
        let start = self.c_index;
        let line = self.line;
        let column = self.column;
        // consume immediately
        let first = self.c_advance();

        let token = match (first,self.c_current()) {
            ('=', Some('=')) => {
                self.c_advance();
                Token::Symbol("==".into(),
                    span(start, self.c_index, line,column))
            },
            ('+', Some('=')) => {
                self.c_advance();
                Token::Symbol("+=".into(),
                span(start, self.c_index, line,column))
            },
            ('-', Some('=')) => {
                self.c_advance();
                Token::Symbol("-=".into(),
                    span(start, self.c_index, line,column))
            },
            ('*', Some('=')) => {
                self.c_advance();
                Token::Symbol("*=".into(),
                    span(start, self.c_index, line,column))
            },
            ('/', Some('=')) => {
                self.c_advance();
                Token::Symbol("/=".into(),
                    span(start, self.c_index, line,column))
            }
            ('!', Some('=')) => {
                self.c_advance();
                Token::Symbol("!=".into(),
                    span(start, self.c_index, line,column))
            }
            ('<', Some('=')) => {
                self.c_advance();
                Token::Symbol("<=".into(),
                    span(start, self.c_index, line,column))

            }
            ('>', Some('=')) => {
                self.c_advance();
                Token::Symbol(">=".into(),
                    span(start, self.c_index, line,column))
            }
            (':', Some('=')) => {
                self.c_advance();
                Token::Symbol(":=".into(),
                    span(start, self.c_index, line,column))
            }
            _ => Token::Symbol(first.to_string(),
                span(start, self.c_index, line,column)),
        };
        self.tokens.push(token);
        Ok(())
    }
    pub fn from_code(code: &String) -> Result<Self, LexerErr>  {
        let mut lexer = Lexer{
            tokens:Vec::new(), index: 0,
            chars: code.chars().collect(), c_index: 0,
            line: 1, column: 1,
        };
        while let Some(c) = lexer.c_current() {
            match c {
                'a'..='z' | 'A'..='Z' | '_' => {
                    lexer.lexe_ident()?;
                },
                '0'..='9' => {
                    lexer.lexe_number()?;
                },
                '(' | ')' | '{' | '}' | '[' | ']' | ':' | ','
                    | '+' | '-' | '*' | '/' | '=' | '!' | '<' | '>' | ';' => {
                        lexer.lexe_symbol()?;
                    }
                c =>  {
                    if c.is_whitespace() {
                        let c = lexer.c_advance();
                        if c == '\n' {
                            lexer.line += 1;
                            lexer.column = 1;
                        }
                    } else {
                        return Err(LexerErr::
                            Invalid(format!("Invalid char {}",  c)));
                    }
                },
            }
        }
        lexer.tokens.push(Token::EOF);

        Ok(lexer)
    }
}
impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Ident(i,span) => { return write!(f, "ident {:?} at {:?}", i,span);}
            Token::Num(n,span) => { return write!(f, "number {:?} at {:?}", n,span);}
            Token::Keyword(k,span) => { return write!(f, "kw {:?} at {:?}", k,span);}
            Token::Symbol(s,span) => { return write!(f, "symbol {:?} at {:?}", s,span);}
            Token::EOF => { return write!(f, "eof");}
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Ident(i,span) => { return write!(f, "ident {:?} at {:?}", i,span);}
            Token::Num(n,span) => { return write!(f, "number {:?} at {:?}", n,span);}
            Token::Keyword(k,span) => { return write!(f, "kw {:?} at {:?}", k,span);}
            Token::Symbol(s,span) => { return write!(f, "symbol {:?} at {:?}", s,span);}
            Token::EOF => { return write!(f, "eof");}
        }
    }
}
impl Token {
    pub fn is_kw(&self, kw: Keyword) -> bool {
        if let Token::Keyword(k, _) = self && *k  == kw {
            true
        } else {
            false
        }
    }
    pub fn is_symbol(&self, symbol: &'static str) -> bool {
        if let Token::Symbol(s, _) = self && s  == symbol {
            true
        } else {
            false
        }
    }
    pub fn is_ident(&self) -> bool {
        if let Token::Ident(_, _) = self {
            true
        } else {
            false
        }
    }
    pub fn is_assingment_symbol(&self) -> bool {
        if let Token::Symbol(s, _) = self {
            match s.as_str() {
                "=" | "+=" | "-=" | "*=" | "/=" => true,
                _ => false
            }
        } else {
            false
        }
    }
    pub fn is_vardec_symbol(&self) -> bool {
        if let Token::Symbol(s, _) = self {
            match s.as_str() {
                ":=" | ":" | "::" => true,
                _ => false
            }
        } else {
            false
        }
    }
}
