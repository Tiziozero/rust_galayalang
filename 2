use crate::parser::{self, ExprId, StmtId};
use std::{collections::HashMap, fmt::Display};

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Object(o) => write!(f, "obj {}", o),
            Self::Type(t) => write!(f, "type {}", t),
        }
    }
}

#[derive(Clone,PartialEq, Eq)]
pub struct Object {
    pub is_const: bool,
    pub name: String,
    pub ty: Option<Type>,
    pub id: SymbolId, // node id for types and what not
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
            Symbol::Type(t) => t.to_string(),
            // _ => panic!("Handle"),
        }
    }
}


pub struct Scope {
    symbols: HashMap<String, SymbolId>, // points to resolved
}
impl Scope {
    fn new() ->Self {
        Self{ symbols: HashMap::new() }
    }
    fn add(&mut self, name: String, id: SymbolId) {
        self.symbols.insert(name, id);
    }
    fn get(&self, name: &String) -> Option<SymbolId> {
        if let Some(v) = self.symbols.get(name) {
            return Some(*v);
        }
        None
    }
}

pub struct Resolver {
    parser: parser::Parser,
    scopes: Vec<Scope>,
    base_scope: usize,
    globals: Scope,
    // value points to resolved
    symbols: HashMap<parser::ExprId, SymbolId>,
    typed: HashMap<parser::ExprId,Type>, // again exprid cus only applies to exprs
    // stores resolved symbols with ExprId being declaration node id
    // stores resolved symbols with ExprId being declaration node id
    resolved: HashMap<SymbolId, Symbol>,
    errors: Vec<String>,
    current_id: SymbolId,
}

pub type SymbolId = usize;
impl Resolver {
    fn next_id(&mut self) -> SymbolId {
        self.current_id += 1;
        self.current_id
    }
    pub fn dump(&self) {
        for (k, sk) in &self.symbols {
            let v = self.resolved.get(sk).unwrap();
            println!("\t{:?} {}", k, v);
        }
    }
    fn resolve_stmt(&mut self, id: StmtId) -> Result<(), String> {
        let stmt = self.parser.get_stmt(id).unwrap();
        match stmt {
            parser::Stmt::Expr(expr) => {
                match self.resolve_expr(expr.clone()) {
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
    fn resolve_type(&mut self, t: &parser::Type) -> Result<&Type, String> {
        match t {
            parser::Type::Base(base) => self.get_type_from_name(base),
            _ => panic!("Handle"),
            
        }
    }
    fn resolve_expr(&mut self, expr_id: ExprId) -> Result<(), String> {
        let expr = self.parser.get_expr(expr_id).unwrap().clone();
        match &expr {
            parser::Expr::Symbol(s) => {
                let id = self.get_obj_ref(&s.symbol)
                    .ok_or(String::from("var doesn't exist."))?;
                println!("symbol {:?} exists with id {:?}", s, id);
                self.add_ref(expr_id, id); // create reference to expr_id
                                           // cuz the expr is the symbol
                return Ok(());
            },
            parser::Expr::Binop(b) => {
                self.resolve_expr(b.left)?; // check exprs are ok
                self.resolve_expr(b.right)?;
                return Ok(())
            }
            parser::Expr::VarDec(v) => {
                if let Some(val) = v.val {
                    self.resolve_expr(val)?; // check val is ok
                }
                let id = self.next_id();
                let mut o = Object{
                    name: v.s.symbol.clone(),
                    ty: None,
                    is_const:false,
                    id:id,
                };
                if let Some(vt) = &v.ty {
                    let t = self.resolve_type(&vt)?;
                    o.ty = Some(t.clone());
                }
                // add
                // new object exprid cus that's the vardec
                self.new_obj(expr_id, id, o)?;
                return Ok(())
            },
            parser::Expr::Number(_) => {
                return Ok(());
            },
            // _ => panic!("Handle {:?}", expr),
        }
    }
    fn add_base_types(&mut self) {
        let t1 = Symbol::Type(Type::Int);
        let t1id = self.next_id();
        self.resolved.insert(t1id, t1);
        self.globals.add(String::from("int"), t1id);
        let t2 = Symbol::Type(Type::Float);
        let t2id = self.next_id();
        self.resolved.insert(t2id, t2);
        self.globals.add(String::from("float"), t2id);
    }
    pub fn new(p: parser::Parser) -> Result<Self, String> {
        let mut scopes = Vec::new();
        scopes.push(Scope::new());
        let mut s = Self{
            parser: p,
            symbols: HashMap::new(),
            typed: HashMap::new(),
            base_scope: 0,
            globals: Scope::new(),
            resolved: HashMap::new(),
            scopes: scopes,
            errors: Vec::new(),
            current_id: 0,
        };
        s.add_base_types();

        for n in &s.parser.root.stmts.clone() {
            s.resolve_stmt(n.clone())?;
        }
        Ok(s)
    }
    fn new_obj(&mut self, node_id: parser::ExprId, id: SymbolId, s: Object) -> 
        Result<SymbolId, String>{
            // check if it exists
        if let Some(_) = self.get_from_name(&s.name) {
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
        self.add_ref(node_id, id);

        println!("New var {}", name);
        Ok(id)
    }
    fn add_ref(&mut self, id: ExprId, to: SymbolId) {
        self.symbols.insert(id, to);
    }
    fn get_obj_from_name(&self, name: &String) -> Result<&Object,String> {
        if let Some(sym) = self.get_from_name(name) {
            match sym {
                Symbol::Object(o) => return Ok(o),
                _=> return Err(String::from(format!("symbol {} not an objext", name))),
            }
        }
        Err(String::from(format!("symbol {} does not exist", name)))
    }
    fn get_type_from_name(&self, name: &String) -> Result<&Type,String> {
        self.dump();
        if let Some(sym) = self.get_from_name(name) {
            match sym {
                Symbol::Type(t) => return Ok(t),
                _=> return Err(String::from(format!("symbol {} not a type", name))),
            }
        }
        Err(String::from(format!("symbol {} does not exist", name)))
    }
    fn get_from_name(&self, name: &String) -> Option<&Symbol> {
        // check if name exists in scope
        if let Some(sym) = self.scopes[self.base_scope..].last()?.get(name) {
            return self.resolved.get(&sym);
        }
        if let Some(sym) = self.globals.get(name) {
            return self.resolved.get(&sym);
        }
        None
    }

    pub fn get_resolved(& self, id: SymbolId) -> Option<&Symbol> {
        return self.resolved.get(&id);
    }
    pub fn get(& self, id: parser::ExprId) -> Option<&Symbol> {
        if let Some(v) = self.symbols.get(&id) {
            return Some(self.resolved.get(v)?);
        }
        println!("Symbol ({:?}) doesn't exist", id);
        self.dump();
        None
    }
    fn get_obj_ref(&self, name: &String) -> Option<SymbolId> {
        // try all scopes from last to first
        for s in self.scopes[self.base_scope..].iter().rev() {
            let id = s.get(name)?;
            // if it exists and is an object
            if let Some(Symbol::Object(_)) = self.resolved.get(&id) {
                return Some(id);
            }
        }
        // try globals next ig?
        let id = self.globals.get(name)?;
        if let Some(Symbol::Object(_)) = self.resolved.get(&id) {
            return Some(id);
        }
        None
    }
    pub fn set_obj_type(&mut self, id: parser::ExprId, t: Type)
        -> Result<(), String> {
        for (k,v) in &self.symbols {
            println!("k {:?} v {}", k, v);
        }
        for (k,v) in &self.resolved {
            println!("res\tk {} v {}", k, v.name());
        }
        let resolved_id = self.symbols.get(&id)
            .ok_or(String::from(format!(
                        "Symbol {:?} doesn't exist as a symbol?.", id)))?;
        let mut s = self.resolved.get(resolved_id)
            .ok_or(String::from(format!(
                        "Symbol {:?} doesn't exist/not resolved.", id)))?;
        if let Symbol::Object(obj) = &mut s {
            // fail if it has a type
            if let Some(_) = obj.ty {
                return Err(String::from("Object already has a type."));
            }
            let mut new = obj.clone();
            new.ty = Some(t);
            let res_id = self.symbols.get(&id).unwrap();
            self.resolved.insert(*res_id, Symbol::Object(new));
            Ok(())
        } else {
            Err(String::from("symbol not a object."))
        }
            
    }
}
