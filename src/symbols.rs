use crate::parser::{self};
use std::{collections::HashMap, fmt::Display};

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Object(o) => write!(f, "{}", o),
            Self::Type(t) => write!(f, "{}", t),
        }
    }
}

#[derive(Clone,PartialEq, Eq)]
pub struct Object {
    pub is_const: bool,
    pub name: String,
    pub ty: Option<Type>,
    pub id: parser::NodeId, // node id for types and what not
}
impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.ty {
            Some(t) => {
                write!(f, "{} type: {}", self.name, t)
            },
            None => {
                write!(f, "{} type: {}", self.name, "no type")
            },
        }
    }
}
#[derive(Copy,Clone,PartialEq, Eq)]
pub struct Arg {
}
#[derive(Clone,PartialEq, Eq)]
pub struct Function {
    pub args: Vec<Arg>,
    pub return_type: Box<Type>,
}
#[derive(Clone,PartialEq, Eq)]
pub enum Type {
    UntypedUnsignedInteger,
    UntypedSignedInteger,
    UntypedFloat,
    Uint, Int, Float,
    Function(Function),
}
impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int => write!(f, "int"),
            Self::Float => write!(f, "flt"),
            Self::UntypedUnsignedInteger => write!(f, "untyped uint"),
            Self::UntypedSignedInteger => write!(f, "untyped int"),
            Self::UntypedFloat => write!(f, "untyped flt"),
            _ => panic!("Handle"),
        }
    }
}
impl Type {
    pub fn get_default_from_untyped(&self) -> Self {
        match self {
            Self::UntypedFloat => Self::Float,
            Self::UntypedSignedInteger =>Self::Int,
            Self::UntypedUnsignedInteger => Self::Int,
            _ => panic!("Shouldn't happen"),
        }
    }
    pub fn is_untyped(&self) -> bool {
        match self {
            Self::UntypedFloat |
            Self::UntypedSignedInteger |
            Self::UntypedUnsignedInteger => return true,
            _ => return false
        }
    }
    pub fn is_numeric(&self) -> bool {
        match self {
            Self::Int |
            Self::Float |
            Self::UntypedFloat |
            Self::UntypedSignedInteger |
            Self::UntypedUnsignedInteger => return true,
            _ => return false
        }
    }
    pub fn is_float(&self) -> bool {
        match self {
            Self::Float |
            Self::UntypedFloat => return true,
            _ => return false
        }
    }
    pub fn is_unsigned(&self) -> bool {
        match self {
            Self::UntypedUnsignedInteger |
            Self::Uint => return true,
            _ => return false
        }
    }
    pub fn is_signed(&self) -> bool {
        match self {
            Self::UntypedSignedInteger |
            Self::Int => return true,
            _ => return false
        }
    }
    pub fn is_integer(&self) -> bool {
        match self {
            Self::UntypedUnsignedInteger |
            Self::UntypedSignedInteger |
            Self::Uint |
            Self::Int => return true,
            _ => return false
        }
    }
}
#[derive(Clone,PartialEq, Eq)]
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
    pub fn dump(&self) {
        for (k, v) in &self.resolved {
            println!("\t{} {}", k, v);
        }
    }
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
                    id:v.s.id,
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
        let root = &p.root;
        for n in root {
            s.resolve_stmt(&n)?;
        }
        Ok(s)
    }
    fn new_obj(&mut self, id: parser::NodeId, s: Object) -> 
        Result<parser::NodeId, String>{
            // check if it exists
        if let Some(_) = self.scope_exists(&s.name) {
            return Err(String::from("Value already exists"));
        }
        let name = s.name.clone();
        // println!("{} {}", self.base_scope, self.scopes.len());
        // else add
        // add to scope
        self.scopes[self.base_scope..]
            .last_mut().unwrap().add(s.name.clone(), id);
        // add to resolved
        self.resolved.insert(id, Symbol::Object(s));
        // add ref to itself
        self.add_ref(id, id);

        println!("New var {}", name);
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
        if let Some(v) = self.symbols.get(&id) {
            return Some(self.resolved.get(v)?);
        }
        println!("Symbol ({}) doesn't exist", id);
        for (id, v) in &self.resolved {
            println!("Symbol {} {}",id, v);
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
    pub fn set_obj_type(&mut self, id: parser::NodeId, t: Type)
        -> Result<(), String> {
        for (k,v) in &self.symbols {
            println!("k {} v {}", k, v);
        }
        for (k,v) in &self.resolved {
            println!("res\tk {} v {}", k, v.name());
        }
        let resolved_id = self.symbols.get(&id)
            .ok_or(String::from(format!(
                        "Symbol {} doesn't exist as a symbol?.", id)))?;
        let mut s = self.resolved.get(resolved_id)
            .ok_or(String::from(format!(
                        "Symbol {} doesn't exist/not resolved.", id)))?;
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
