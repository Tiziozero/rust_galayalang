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
    Fn, If, Else,
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
#[derive(Debug)]
pub struct Lexer {
    tokens: Vec<Token>,
    index: u32,
}
impl Lexer {
    pub fn peek(&mut self) -> Option<&Token> {
        if self.index < self.tokens.len() as u32 -1 {
            return Some(&self.tokens[self.index as usize +1]);
        } else {
            return None;
        }
    }
    pub fn current(&mut self) -> Option<&Token> {
        if self.index > self.tokens.len() as u32 - 1 {
            return None;
        }
        let t = &self.tokens[self.index as usize];
        return Some(t);
    }
    pub fn next(&mut self) -> Option<&Token> {
        if self.index > self.tokens.len() as u32 - 1 {
            return None;
        }
        let t = &self.tokens[self.index as usize];
        self.index += 1;
        return Some(t);
    }
    pub fn back(&mut self) {
        if self.index > 0 { self.index -= 1; }
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
    pub fn from_code(code: &String) -> Result<Self, LexerErr>  {
        let mut tokens: Vec<Token> = vec![];
        let mut chars = code.chars().peekable();
        let mut i = 0;
        let mut line = 1;
        let mut column = 1;
        while let Some(c) = chars.peek() {
            let start = i;
            match c {
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut name = String::new();
                    while let Some(c) = chars.peek() {
                        match c {
                            'a'..='z' | 'A'..='Z' | '_' | '0'..='9' =>
                                name.push(*c),
                            _ => break
                        }
                        let s = chars.next().unwrap().len_utf8(); // advance
                        i+=s;
                        column += 1;
                    }
                    match name.as_str() {
                        "fn" =>
                            tokens.push(
                                Token::Keyword(Keyword::Fn,span(start, i, line, column))),
                        "if" =>
                            tokens.push(
                                Token::Keyword(Keyword::If,span(start, i, line, column))),
                        "else" =>
                            tokens.push(
                                Token::Keyword(Keyword::Else,span(start, i, line, column))),
                        _ => 
                            tokens.push(Token::Ident(name,span(start, i, line, column))),
                    }
                },
                '0'..='9' => {
                    let mut num = String::new();
                    while let Some(c) = chars.peek() {
                        match c {
                            '0'..='9'=>
                                num.push(*c),
                            _ => break
                        }
                        let s = chars.next().unwrap().len_utf8(); // advance
                        i+=s;
                        column += 1;
                    }
                    tokens.push(Token::Num(num,span(start, i, line, column)));
                },
                '(' | ')' | '{' | '}' | '[' | ']' | ':'
                    | '+' | '-' | '*' | '/' | '=' | '!' | '<' | '>' | ';' => {
                        let first = chars.next().unwrap(); // consume immediately

                        let token = match (first, chars.peek()) {
                            ('=', Some('=')) => {
                                let s = chars.next().unwrap().len_utf8();
                                i+=s; column += 1;
                                Token::Symbol("==".into(),span(start, i, line, column))
                        },
                            ('+', Some('=')) => {
                                let s = chars.next().unwrap().len_utf8();
                                i+=s; column += 1;
                                Token::Symbol("+=".into(),span(start, i, line, column))
                        },
                            ('-', Some('=')) => {
                                let s = chars.next().unwrap().len_utf8();
                                i+=s; column += 1;
                                Token::Symbol("-=".into(),span(start, i, line, column))
                        },
                            ('*', Some('=')) => {
                                let s = chars.next().unwrap().len_utf8();
                                i+=s; column += 1;
                                Token::Symbol("*=".into(),span(start, i, line, column))
                        },
                            ('/', Some('=')) => {
                                let s = chars.next().unwrap().len_utf8();
                                i+=s; column += 1;
                                Token::Symbol("/=".into(),span(start, i, line, column))
                        }
                            ('!', Some('=')) => {
                                let s = chars.next().unwrap().len_utf8();
                                i+=s; column += 1;
                                Token::Symbol("!=".into(),span(start, i, line, column))
                        }
                            ('<', Some('=')) => {
                                let s = chars.next().unwrap().len_utf8();
                                i+=s; column += 1;
                                Token::Symbol("<=".into(),span(start, i, line, column))

                        }
                            ('>', Some('=')) => {
                                let s = chars.next().unwrap().len_utf8();
                                i+=s; column += 1;
                                Token::Symbol(">=".into(),span(start, i, line, column))
                        }
                            (':', Some('=')) => {
                                let s = chars.next().unwrap().len_utf8();
                                i+=s; column += 1;
                                Token::Symbol(":=".into(),span(start, i, line, column))
                        }
                            _ => Token::Symbol(first.to_string(),span(start, i, line, column)),

                        };

                        tokens.push(token);
                    }
                c =>  {
                    if c.is_whitespace() {
                        if let Some(c) = chars.next() {
                            if c == '\n' {
                                line += 1;
                                column = 1;
                            }
                            let s = c.len_utf8();
                            i += s;
                        }
                    } else {
                        println!("Invalid char {}", *c as u32);
                        return Err(LexerErr::
                            Invalid(format!("Invalid char {}",  c)));
                    }
                },
            }
        }
        tokens.push(Token::EOF);

        println!("returning {} tokens", tokens.len());
        Ok(Lexer{tokens, index:0})
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
}
