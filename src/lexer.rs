use std::{fmt};
#[derive(Clone)]
pub enum Token {
    Ident(String),
    Num(String),
    Keyword(String),
    Symbol(String), // can be double like "+="
    EOF,
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
        while let Some(c) = chars.peek() {
            match c {
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut name = String::new();
                    while let Some(c) = chars.peek() {
                        match c {
                            'a'..='z' | 'A'..='Z' | '_' =>
                                name.push(*c),
                            _ => break
                        }
                        chars.next(); // advance
                    }
                    match name.as_str() {
                        "fn"| "if" | "else" =>
                            tokens.push(Token::Keyword(name)),
                        _ => 
                            tokens.push(Token::Ident(name)),
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
                        chars.next(); // advance
                    }
                    tokens.push(Token::Num(num));
                },
                '(' | ')' | '{' | '}' |'[' | ']'
                    | '+' | '-' | '*' | '/' |'='
                    | ';' => {
                        tokens.push(Token::Symbol(String::from(*c)));
                        chars.next();
                }
                c =>  {
                    if c.is_whitespace() {
                        chars.next();
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
            Token::Ident(i) => { return write!(f, "ident {:?}", i);}
            Token::Num(n) => { return write!(f, "number {:?}", n);}
            Token::Keyword(k) => { return write!(f, "kw {:?}", k);}
            Token::Symbol(s) => { return write!(f, "symbol {:?}", s);}
            Token::EOF => { return write!(f, "eof");}
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Ident(i) => { return write!(f, "ident {:?}", i);}
            Token::Num(n) => { return write!(f, "number {:?}", n);}
            Token::Keyword(k) => { return write!(f, "kw {:?}", k);}
            Token::Symbol(s) => { return write!(f, "symbol {:?}", s);}
            Token::EOF => { return write!(f, "eof");}
        }
    }
}
