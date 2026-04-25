use crate::parser;
use std::collections::HashMap;


pub struct Object {
    is_const: bool,
    name: String,
    ty: Type,
}
#[derive(Copy,Clone,PartialEq, Eq)]
struct Arg {
}
#[derive(Clone,PartialEq, Eq)]
struct Function {
    args: Vec<Arg>,
    return_type: Box<Type>,
}
#[derive(Clone,PartialEq, Eq)]
enum Type {
    Integer,
    Float,
    Function(Function),
}
pub type Symbol = Object;

pub struct Scope {
    symbols: HashMap<parser::NodeId, Symbol>,
}
impl Scope {
    fn new() ->Self {
        Self{ symbols: HashMap::new() }
    }
    fn add(&mut self, id: parser::NodeId, s: Symbol) {
        self.symbols.insert(id, s);
    }
    fn get(&mut self, id: parser::NodeId) -> Option<&Object> {
        if let Some(v) = self.symbols.get(&id) {
            return Some(v);
        }
        None
    }
}

pub struct Resolver {
    symbols: HashMap<parser::NodeId, Symbol>,
    errors: Vec<String>,
}

impl Resolver {
    fn resolve_stmt(&mut self, stmt: &parser::Stmt) -> Result<(), String> {
        match stmt {
            parser::Stmt::Expr(expr) => {
                return self.resolve_expr(expr);
            },
            // _ => panic!("Handle {}", n),
        }
    }
    fn resolve_expr(&mut self, expr: &parser::Expr) -> Result<(), String> {
        match &expr {
            parser::Expr::Binop(b) => {
                self.resolve_expr(&b.left)?;
                self.resolve_expr(&b.right)?;
            }
            _ => panic!("Handle {:?}", expr),
        }
        Ok(())
    }
    pub fn new(p: parser::Parser) -> Result<Self, String> {
        let mut s = Self{
            symbols: HashMap::new(), errors: Vec::new()
        };
        if let Some(parser::Root::Block(root)) = p.root {
            for n in &root {
                s.resolve_stmt(&n)?;
            }
        }
        Ok(s)
    }
    fn add(&mut self, id: parser::NodeId, s: Symbol) {
        self.symbols.insert(id, s);
    }
    fn get(&mut self, id: parser::NodeId) -> Option<&Object> {
        if let Some(v) = self.symbols.get(&id) {
            return Some(v);
        }
        None
    }
}
