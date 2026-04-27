use std::collections::HashMap;

use crate::{parser::{self, Expr, Parser}, symbols};

pub struct TypeChecker<'a> {
    symbols: &'a mut symbols::Resolver,
    resolved: HashMap<parser::NodeId,symbols::Type>,
    fn_data: Vec<symbols::Function>, // if it's in a function
                                     // vec for nested functions and what not
}
impl<'a> TypeChecker<'a> {
    fn handle_vardec(&mut self, vardec: &parser::VarDec) -> Result<(),String> {
        let obj: symbols::Object;
        if let symbols::Symbol::Object(symbol_var) = self.symbols.get(vardec.s.id).unwrap() {
            obj = symbol_var.clone();
        } else {
            return Err(String::from("vardec not resolved?"));
        }
        if let Some(val) = &vardec.val {
            self.resolve_expr(val)?;
            // get value type
            let val_t = self.resolved.get(&val.id()).unwrap().clone();
            // if obj has a type compare
            if let Some(var_t) = obj.ty.clone() {
                if val_t != var_t {
                    return Err(String::from(format!(
                                "vardec value type doesn't match vardec type {}:{}",
                                var_t, val_t)));
                }
            } else { // else set obj type
                self.symbols.set_onj_type(vardec.s.id, val_t.clone())?;
            }

            return Ok(());
        }
        Err(String::from("Handle"))
    }
    fn resolve_expr(&mut self, e: &parser::Expr) ->Result<(),String> {
        match e {
            Expr::VarDec(v) => {
                return self.handle_vardec(v);
            }
            Expr::Number(n) => {
                // set to untyped int
                self.resolved.insert(n.id, symbols::Type::UntypedUnsignedInteger);
            }
            Expr::Symbol(s) => {
                // get symbol in symbol table and make sure it's an object
                let symbol = 
                    self.symbols.get(s.id) .ok_or(String::from(format!(
                                "Symbol {} doesn't exist.", s.symbol)))?;
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
                self.resolve_expr(&b.left)?;
                self.resolve_expr(&b.right)?;
                let lt = self.resolved.get(&b.left.id()).unwrap();
                let rt = self.resolved.get(&b.right.id()).unwrap();
                if lt != rt {
                    return Err(String::from(format!(
                                "Binop types don't match {} {}.", lt, rt)));
                }
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
        ->Result<(),String> {
        let mut c = TypeChecker{
            symbols,
            fn_data: Vec::new(),
            resolved: HashMap::new()};

        if let Some(scope) = &p.root {
            c.resolve_scope(scope)?;
            if c.fn_data.len() != 0 { // make sure to exit all scopes before rreturning
                return Err(String::from(format!("Haven't exited all scopes? {}",
                        c.fn_data.len())));
            }
            Ok(())
        } else {
            Err(String::from("No root"))
        }
    }
}
