use std::collections::HashMap;

use crate::{parser::{self, Expr, Parser}, symbols};

pub struct TypeChecker<'a> {
    symbols: &'a mut symbols::Resolver,
    resolved: HashMap<parser::NodeId,symbols::Type>,
    fn_data: Vec<symbols::Function>, // if it's in a function
                                     // vec for nested functions and what not
}
impl<'a> TypeChecker<'a> {
    fn dump(&self) {
        for (k, v) in &self.resolved {
            println!("\t{:?} {}", k, v);
        }
    }
    fn propagate_type(&mut self, t: &symbols::Type, expr: &parser::Expr) {
        match expr {
            parser::Expr::Binop(b) => {
                self.propagate_type(t, &b.left);
                self.propagate_type(t, &b.right);
            },
            parser::Expr::Number(n) => {
                println!("updating type for id {:?} to {}", n.id, t);
                self.resolved.insert(n.id, t.clone());
            }
            parser::Expr::Symbol(_) => {
                // nvm ignore ig
                // panic!("What. shoudn't be propagating to symbol.");
            }
            parser::Expr::VarDec(_) => {
                // nvm ignore ig
                // panic!("What. shoudn't be propagating to vardec.");
            }
        }
    }
    fn handle_vardec(&mut self, vardec: &parser::VarDec) -> Result<(),String> {
        let id = vardec.s.id;
        let obj: symbols::Object;
        if let symbols::Symbol::Object(symbol_var) =
                                                self.symbols.get(id).unwrap() {
            obj = symbol_var.clone(); // get object
        } else {
            return Err(String::from("vardec not resolved?"));
        }
        println!("Checking vardec {}", &obj.name);
        if let Some(val) = &vardec.val {
            self.resolve_expr(val)?;
            // get value type
            let mut val_t = self.resolved.get(&val.id()).unwrap().clone();
            // if obj has a type compare
            if let Some(var_t) = obj.ty.clone() {
                // var_t first arg cus i'd return that
                let t3 = Self::handle_untyped(&var_t, &val_t)?;
                self.propagate_type(&t3, val); // only value
                // set to type
                self.resolved.insert(vardec.s.id, t3.clone()); // add to resolved. ofc
            } else { // else set obj type
                println!("obj {} has no type", &obj.name);
                // if it's untyped then set both val and s
                if val_t.is_untyped() {
                    val_t = val_t.get_default_from_untyped();
                    // set value to resolved types
                    self.resolved.insert(vardec.val
                        .as_ref().unwrap().id(), val_t.clone());
                }
                println!("setting obj {} type to {}", &obj.name, val_t);
                // set in symbol table
                self.symbols.set_obj_type(vardec.s.id, val_t.clone()).unwrap();
                self.resolved.insert(vardec.s.id, val_t.clone()); // add to resolved. ofc
            }
            return Ok(());
        } else { // no value so must have type
            if let Some(t) = obj.ty {
                self.resolved.insert(id, t);
                return Ok(());
            } else {
                return Err(String::from(
                        "object has no type AND no value. Can't have that"));
            }
        }
    }
    fn handle_untyped(l: &symbols::Type, r: &symbols::Type)
        -> Result<symbols::Type, String> {
        if l.is_untyped() && r.is_untyped() {
            if l.is_numeric() && r.is_numeric() {
                if l.is_float() || r.is_float()  { // if either is flaot
                    return Ok(symbols::Type::UntypedFloat); // untyped float
                } else if l.is_signed() || r.is_signed()  { // signed int
                    return Ok(symbols::Type::UntypedSignedInteger);
                } else { // else uint
                    return Ok(symbols::Type::UntypedUnsignedInteger);
                }
            } else {
                panic!("Handle untyped non-numeric");
            }
        } else if l.is_untyped() { // if only left is typed
            if l.is_numeric() && r.is_numeric() {
                if l.is_float()  { // if left is float cast to float
                    return Ok(symbols::Type::Float);
                } else if l.is_signed() || r.is_integer()  { // signed int
                    return Ok(symbols::Type::Int);
                } else { // else uint
                    return Ok(symbols::Type::Uint);
                }
            } else {
                panic!("Handle typed/untyped non-numeric");
            }
        } else if r.is_untyped() { // if only right is typed
            if r.is_numeric() && l.is_numeric() {
                if r.is_float()  { // if right is float cast to float
                    return Ok(symbols::Type::Float);
                } else if r.is_signed() || l.is_integer()  { // signed int
                    return Ok(symbols::Type::Int);
                } else { // else uint
                    return Ok(symbols::Type::Uint);
                }
            } else {
                panic!("Handle untyped/typed non-numeric");
            }
        } else if !l.is_untyped() && !r.is_untyped() {
            if l == r {
                return Ok(l.clone());
            } else {
                return Err(String::from("Types don't match"));
            }
        }
        panic!("Handle");
    }
    fn resolve_expr(&mut self, e: &parser::Expr) ->Result<(),String> {
        match e {
            Expr::VarDec(v) => {
                return self.handle_vardec(v);
            }
            Expr::Number(n) => {
                // set to untyped int
                self.resolved.insert(n.id,
                    symbols::Type::UntypedUnsignedInteger);
            }
            Expr::Symbol(s) => {
                // get symbol in symbol table and make sure it's an object
                let symbol = 
                    self.symbols.get(s.id).unwrap();//.ok_or(String::from(format!("Symbol {}({:?}) doesn't exist.", s.symbol, s.id)))?;
                if let symbols::Symbol::Object(symbol_var) = symbol {
                    if let Some(t) = &symbol_var.ty {
                        self.resolved.insert(s.id, t.clone());
                    } else {
                        return Err(String::from(format!(
                                    "no type in symbol {}.", s.symbol)));
                    }
                } else {
                    return Err(String::from(format!(
                        "Symbol is not an object and {}",
                        "can not be used in expressions.")));
                }
            },
            Expr::Binop(b) => {
                println!("binop left: {:?}", b.left);
                self.resolve_expr(&b.left)?;
                self.resolve_expr(&b.right)?;
                let mut lt = self.resolved.get(&b.left.id()).unwrap().clone();
                let mut rt = self.resolved.get(&b.right.id()).unwrap().clone();
                let t = TypeChecker::handle_untyped(&lt, &rt)?;
                self.propagate_type(&t, &parser::Expr::Binop(b.clone()));
                // get new types
                lt = self.resolved.get(&b.left.id()).unwrap().clone();
                rt = self.resolved.get(&b.right.id()).unwrap().clone();
                // check if untyped
                let t3 = TypeChecker::handle_untyped(&lt, &rt)?;
                self.propagate_type(&t3, &b.left);
                self.propagate_type(&t3, &b.right);
                self.resolved.insert(b.id, lt.clone());
            }
            // _ => panic!("handle"),
        }
        Ok(())
    }
    fn resolve_stmt(&mut self, s: &parser::Stmt) ->Result<(),String> {
        match s {
            parser::Stmt::Expr(expr) => self.resolve_expr(expr),
            // _ => panic!("Handle {}", s),
        } 
    }
    fn resolve_scope(&mut self, s: &parser::Scope) ->Result<(),String> {
        for stmt in s {
            self.resolve_stmt(&stmt)?;
        }
        Ok(())
    }
    pub fn resolve(symbols: &'a mut symbols::Resolver, p: &Parser)
        ->Result<HashMap<parser::NodeId, symbols::Type>,String> {
        let mut c = TypeChecker{
            symbols,
            fn_data: Vec::new(),
            resolved: HashMap::new()};

        let scope = &p.root;
        c.resolve_scope(scope)?;
        if c.fn_data.len() != 0 { // make sure to exit all scopes before rreturning
            return Err(String::from(format!("Haven't exited all scopes? {}",
                        c.fn_data.len())));
        }
        Ok(c.resolved)
    }
}
