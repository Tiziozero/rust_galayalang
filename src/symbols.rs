use crate::parser::{self};
use std::{collections::HashMap};


#[derive(Clone,PartialEq, Eq)]
pub struct Object {
    is_const: bool,
    name: String,
    ty: Option<Type>,
}
#[derive(Copy,Clone,PartialEq, Eq)]
pub struct Arg {
}
#[derive(Clone,PartialEq, Eq)]
pub struct Function {
    args: Vec<Arg>,
    return_type: Box<Type>,
}
#[derive(Clone,PartialEq, Eq)]
pub enum Type {
    Integer,
    Float,
    Function(Function),
}
pub enum Symbol {
    Object(Object),
    Type(Type),
}
impl Symbol {
    fn name(&self) -> String {
        match self {
            Symbol::Object(v) => v.name.clone(),
            _ => panic!("Handle"),
        }
    }
}

pub struct Scope {
    symbols: HashMap<String, parser::NodeId>,
}
impl Scope {
    fn new() ->Self {
        Self{ symbols: HashMap::new() }
    }
    fn add(&mut self, name: String, id: parser::NodeId) {
        self.symbols.insert(name, id);
    }
    fn get(&self, name: &String) -> Option<parser::NodeId> {
        if let Some(v) = self.symbols.get(name) {
            return Some(*v);
        }
        None
    }
}

pub struct Resolver {
    scopes: Vec<Scope>,
    base_scope: usize,
    globals: Scope,
    // value points to resolved
    symbols: HashMap<parser::NodeId, parser::NodeId>,
    // stores resolved symbols with NodeId being declaration node id
    // stores resolved symbols with NodeId being declaration node id
    resolved: HashMap<parser::NodeId, Symbol>,
    errors: Vec<String>,
}

impl Resolver {
    fn resolve_stmt(&mut self, stmt: &parser::Stmt) -> Result<(), String> {
        match stmt {
            parser::Stmt::Expr(expr) => {
                match self.resolve_expr(expr) {
                    Ok(()) => return Ok(()),
                    Err(s) => {
                        self.errors.push(s.clone());
                        return Err(s);
                    }
                };
            },
            // _ => panic!("Handle {}", n),
        }
    }
    fn resolve_expr(&mut self, expr: &parser::Expr) -> Result<(), String> {
        match &expr {
            parser::Expr::Symbol(s) => {
                if let Some(id) = self.get_obj_ref(&s.symbol) {
                    println!("symbol {:?} exists with id {:?}", s, id);
                    self.add_ref(s.id, id); // create reference to id
                    return Ok(());
                }
            },
            parser::Expr::Binop(b) => {
                self.resolve_expr(&b.left)?; // check exprs are ok
                self.resolve_expr(&b.right)?;
            }
            parser::Expr::VarDec(v) => {
                if let Some(val) = &v.val {
                    self.resolve_expr(&val)?; // check val is ok
                }
                if let Some(_) = &v.ty {
                    panic!("handle");
                }
                let o = Object{
                    name: v.s.symbol.clone(),
                    ty: None,
                    is_const:false,
                };
                // add
                self.new_obj(v.s.id, o)?;
            },
            parser::Expr::Number(_) => {},
            // _ => panic!("Handle {:?}", expr),
        }
        Ok(())
    }
    pub fn new(p: &parser::Parser) -> Result<Self, String> {
        let mut scopes = Vec::new();
        scopes.push(Scope::new());
        let mut s = Self{
            symbols: HashMap::new(),
            base_scope: 0,
            globals: Scope::new(),
            resolved: HashMap::new(),
            scopes: scopes,
            errors: Vec::new(),
        };
        if let Some(root) = &p.root {
            for n in root {
                s.resolve_stmt(&n)?;
            }
        }
        Ok(s)
    }
    fn new_obj(&mut self, id: parser::NodeId, s: Object) -> 
        Result<parser::NodeId, String>{
            // check if it exists
        if let Some(_) = self.scope_exists(&s.name) {
            return Err(String::from("Value already exists"));
        }
        println!("{} {}", self.base_scope, self.scopes.len());
        // else add
        // add to scope
        self.scopes[self.base_scope..]
            .last_mut().unwrap().add(s.name.clone(), id);
        // add to resolved
        self.resolved.insert(id, Symbol::Object(s));
        // add ref to itself
        self.add_ref(id, id);

        Ok(id)
    }
    fn add_ref(&mut self, id: parser::NodeId, to: parser::NodeId) {
        self.symbols.insert(id, to);
    }
    fn scope_exists(&self, name: &String) -> Option<&Symbol> {
        // check if name exists in scope
        if let Some(sym) = self.scopes[self.base_scope..].last()?.get(name) {
            return self.get(sym);
        }
        None
    }
    pub fn get(& self, id: parser::NodeId) -> Option<&Symbol> {
        if let Some(v) = self.resolved.get(&id) {
            return Some(v);
        }
        None
    }
    fn get_obj_ref(&self, name: &String) -> Option<parser::NodeId> {
        // try all scopes from last to first
        for s in self.scopes[self.base_scope..].iter().rev() {
            let id = s.get(name)?;
            // if it exists and is an object
            if let Some(Symbol::Object(_)) = self.get(id) {
                return Some(id);
            }
        }
        // try globals next ig?
        let id = self.globals.get(name)?;
        if let Some(Symbol::Object(_)) = self.get(id) {
            return Some(id);
        }
        None
    }
    pub fn set_onj_type(&mut self, id: parser::NodeId, t: Type)
        -> Result<(), String> {
        let mut s = self.resolved.get(&id).unwrap().clone();
        if let Symbol::Object(obj) = &mut s {
            // fail if it has a type
            if let Some(_) = obj.ty {
                return Err(String::from("Object already has a type."));
            }
            let mut new = obj.clone();
            new.ty = Some(t);
            self.resolved.insert(id, Symbol::Object(new));
            Ok(())
        } else {
            Err(String::from("symbol not a object."))
        }
            
    }
}
