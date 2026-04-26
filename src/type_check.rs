use std::collections::HashMap;

use crate::{parser::{self, Parser}, symbols};

pub struct TypeChecker<'a> {
    symbols: &'a mut symbols::Resolver,
    resolved: HashMap<parser::NodeId,symbols::Type>,
    fn_data: Vec<symbols::Function>, // if it's in a function
                                     // vec for nested functions and what not
}
impl<'a> TypeChecker<'a> {
    fn resolve_expr(&self, e: &parser::Expr) ->Result<(),String> {
        match e {
            _ => {},
        }
        Ok(())
    }
    fn resolve_stmt(&self, s: &parser::Stmt) ->Result<(),String> {
        match s {
            parser::Stmt::Expr(expr) => self.resolve_expr(expr),
            // _ => panic!("Handle {}", s),
        } 
    }
    fn resolve_scope(&self, s: &parser::Scope) ->Result<(),String> {
        for stmt in s {
            self.resolve_stmt(&stmt)?;
        }
        Ok(())
    }
    pub fn resolve(symbols: &'a mut symbols::Resolver, p: &Parser)
        ->Result<(),String> {
        let c = TypeChecker{
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
