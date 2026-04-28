use std::collections::HashMap;
use crate::{parser, symbols};
#[derive(Clone)]
struct Fn {
    body: Vec<parser::Stmt>,
    return_values: Option<Box<Value>>,
}
#[derive(Clone)]
enum Value {
    Int(i32),
    Float(f32),
    Fn(Fn),
}
struct Symbol {
    ty: symbols::Type,
    value: Value,
}
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Int(i) => {
                if let Self::Int(o) = other {
                    return i == o;
                } else {
                    return false;
                }
            }
            Self::Float(f) => {
                if let Self::Float(o) = other {
                    return f == o;
                } else {
                    return false;
                }
            }
            _=>panic!("handle")
        }
    }
}
pub struct Interpreter {
    values: HashMap<parser::NodeId, Value>,
    symbols: symbols::Resolver,
    types: HashMap<parser::NodeId, symbols::Type>,
}
fn add_vals(a: &Value, b: &Value) -> Result<Value, String> {
    match a {
        Value::Int(i) => {
            if let Value::Int(other) = b { 
                return Ok(Value::Int(i + other));
            }
        }
        _=> panic!("handle"),
    }
    Err(String::from("handle"))
}
impl Interpreter {
    fn eval_expr(&mut self, expr: &parser::Expr) -> Result<Value, String> {
        match expr {
            parser::Expr::Binop(b) => {
                let lv = self.eval_expr(&b.left)?;
                let rv = self.eval_expr(&b.right)?;
                match b.kind {
                    parser::BinopKind::Add  => {
                        return add_vals(&lv, &rv);
                    }
                    _=>panic!("Handle"),
                }
            }
            parser::Expr::Symbol(s) => {
                let id = s.id;
                let v = self.values.get(&id).
                    ok_or(String::from("Value doesn't exist"))?.clone();
                return Ok(v);
            }
            parser::Expr::Number(n) => {
                let t = self.types.get(&n.id)
                    .ok_or(String::from("no type for number literal"))?;
                match t {
                    symbols::Type::Float => {
                        return Ok(Value::Float(n.str.parse::<f32>().unwrap()));
                    }
                    symbols::Type::Int => {
                        return Ok(Value::Int(n.str.parse::<i32>().unwrap()));
                    }
                    _ => return Err(String::from("Trying to convert num lit to some other type that is not numeric")),
                }
            },
            parser::Expr::VarDec(v) => {
                let id = v.s.id;
                let t = self.types.get(&id)
                    .ok_or(String::from(format!("vardec {:?} has no type", v.s)))?;
                let value: Value;
                if let Some(val) = &v.val {
                    value = self.eval_expr(val)?;
                } else { // get base type null value
                    match t {
                        symbols::Type::Float => {
                            value = Value::Float(0.0);
                        }
                        symbols::Type::Int => {
                            value = Value::Int(0);
                        }
                        _ => panic!("handle"),
                    }
                }
                self.values.insert(id, value.clone());
                return Ok(value);
            },
            // _ => panic!("handle"),
        }
    }
    fn run_stmt(&mut self, stmt: &parser::Stmt) -> Result<(), String> {
        match &stmt {
            parser::Stmt::Expr(expr) => {
                self.eval_expr(expr)?;
            }
            // _ => panic!("handle"),
        }
        Ok(())
    }
    pub fn run(p: parser::Parser, symbols: symbols::Resolver,
        types: HashMap<parser::NodeId, symbols::Type>) -> Result<(), String>{
        let mut interpreter = Interpreter{ values: HashMap::new(), symbols, types };
        for s in &p.root {
            interpreter.run_stmt(s)?;
        }
        Ok(())
    }
}
