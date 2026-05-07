use std::{fmt};
#[derive(Clone,Debug, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}
fn span(s:usize,e:usize)->Span {
    Span{start:s, end:e}
}
#[derive(Debug,Clone, PartialEq)]
pub enum Keyword {
    Fn,
}
#[derive(Clone, PartialEq)]
pub enum Token {
    Ident(String,Span),
    Num(String,Span),
    Keyword(Keyword,Span),
    Symbol(String,Span), // can be double like "+="
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
        let mut i = 0;
        while let Some(c) = chars.peek() {
            let start = i;
            match c {
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut name = String::new();
                    while let Some(c) = chars.peek() {
                        match c {
                            'a'..='z' | 'A'..='Z' | '_' =>
                                name.push(*c),
                            _ => break
                        }
                        i += chars.next().unwrap().len_utf8(); // advance
                    }
                    match name.as_str() {
                        "fn" =>
                            tokens.push(
                                Token::Keyword(Keyword::Fn,span(start, i))),
                        _ => 
                            tokens.push(Token::Ident(name,span(start,i))),
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
                        i+=chars.next().unwrap().len_utf8(); // advance
                    }
                    tokens.push(Token::Num(num,span(start,i)));
                },
                '(' | ')' | '{' | '}' | '[' | ']' | ':'
                    | '+' | '-' | '*' | '/' | '=' | '!' | '<' | '>' | ';' => {
                        let first = chars.next().unwrap(); // consume immediately

                        let token = match (first, chars.peek()) {
                            ('=', Some('=')) => {
                                i+=chars.next().unwrap().len_utf8(); // advance
                                Token::Symbol("==".into(),span(start,i))
                        }
                            ('!', Some('=')) => {
                                i+=chars.next().unwrap().len_utf8(); // advance
                                Token::Symbol("!=".into(),span(start,i))
                        }
                            ('<', Some('=')) => {
                                i+=chars.next().unwrap().len_utf8(); // advance
                                Token::Symbol("<=".into(),span(start,i))

                        }
                            ('>', Some('=')) => {
                                i+=chars.next().unwrap().len_utf8(); // advance
                                Token::Symbol(">=".into(),span(start,i))
                        }
                            (':', Some('=')) => {
                                i+=chars.next().unwrap().len_utf8(); // advance
                                Token::Symbol(":=".into(),span(start,i))
                        }
                            _ => Token::Symbol(first.to_string(),span(start,i)),

                        };

                        tokens.push(token);
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
            Token::Ident(i,_) => { return write!(f, "ident {:?}", i);}
            Token::Num(n,_) => { return write!(f, "number {:?}", n);}
            Token::Keyword(k,_) => { return write!(f, "kw {:?}", k);}
            Token::Symbol(s,_) => { return write!(f, "symbol {:?}", s);}
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
