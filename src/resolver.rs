use std::fs::read_to_string;
pub struct ModuleContext {

}
use crate::parser;
use crate::lexer;
use crate::symbols;
impl ModuleContext {
    pub fn new() -> Self {
        let f = read_to_string("main.gala").unwrap();
        let l = lexer::Lexer::from_code(&f).unwrap();
        let p = parser::Parser::parse(l);
        let _ = symbols::SymbolTable::resolve(p).unwrap();
        Self{}
    }
}
