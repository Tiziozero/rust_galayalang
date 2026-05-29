use crate::parser;
use crate::{resolver, symbols};
use crate::{debugln};

pub struct TypeChecker<'ctx> {
    ctx: &'ctx mut resolver::Context,
    current_fn_ret_type: Option<symbols::TypeId>,
}

impl<'ctx> TypeChecker<'ctx> {
    pub fn type_check(ctx: &'ctx mut resolver::Context, moduleid: parser::ModId)
                                                        -> Result<(), String> {
        let mut tc = TypeChecker{ctx, current_fn_ret_type:None};
        for i in tc.ctx.get_module(moduleid).unwrap().items.clone() {
            debugln!("{:?}", i);
            tc.tc_item(i)?;
        }
        Ok(())
    }
    fn propagate_type(&mut self, expr: parser::ExprId, ty: symbols::TypeId)
                                                        -> Result<(), String> {
        let t = self.ctx.get_type(ty).unwrap().clone();
        let e = self.ctx.get_expr(expr).unwrap().clone();
        debugln!("Propagating: {:?} to {:?}", t, e);
        match e {
            parser::Expr::Number(_) => {
                // overwrite the literal's type with the concrete one
                self.ctx.new_expr_ty_ref(expr, ty);
                Ok(())
            },
            parser::Expr::Symbol(_) => {
                // symbols have a declared type, don't overwrite
                Ok(())
            },
            parser::Expr::Binop(b) => {
                let current = self.ctx.get_expr_ty_ref(expr).unwrap();
                let current_ty = self.ctx.get_type(current).unwrap().clone();
                if current_ty.is_untyped() {
                    self.ctx.new_expr_ty_ref(expr, ty);
                }
                self.propagate_type(b.left, ty)?;
                self.propagate_type(b.right, ty)?;
                Ok(())
            },
            parser::Expr::FnCall(_) => {
                // fn call has its own return type, don't touch it
                Ok(())
            },
        }
    }
    fn tc_binop(&mut self, id: parser::ExprId) -> Result<symbols::TypeId, String> {
        let b = match self.ctx.get_expr(id).unwrap() {
            parser::Expr::Binop(b) => b.clone(),
            _ => panic!("Expected binop"),
        };
        // check left and right
        let l = self.tc_expr(b.left)?;
        let r = self.tc_expr(b.right)?;
        // make sure they can binop
        let lexpr_ty = self.ctx.get_type(l).unwrap().clone();
        if !lexpr_ty.can_binop() {
            return Err(String::from(format!(
            "Can not binop expr (left) {:?}.", lexpr_ty)));
        }
        let rexpr_ty = self.ctx.get_type(r).unwrap().clone();
        if !rexpr_ty.can_binop() {
            return Err(String::from(format!(
            "Can not binop expr (right) {:?}.", lexpr_ty)));
        }
        // compare resulting ids
        let res_id = self.compare_and_reduce_types(l, r)?;
        // add type ref
        self.ctx.new_expr_ty_ref(id, res_id);
        // propagate to children
        self.propagate_type(b.left, res_id).unwrap();
        self.propagate_type(b.right, res_id).unwrap();

        return Ok(res_id);
    }
    fn tc_expr(&mut self, id: parser::ExprId) -> Result<symbols::TypeId, String> {
        let expr = self.ctx.get_expr(id).unwrap().clone();
        match expr {
            parser::Expr::Binop(_) => {
                let type_id = self.tc_binop(id)?;
                self.ctx.new_expr_ty_ref(id, type_id);
                Ok(type_id)
            }
            parser::Expr::Number(n) => {
                // it's a float
                if n.str.contains('.') {
                    // intern or ger literal
                    let type_id =self.ctx.intern_type(symbols::Type::FloatLiteral); 
                    self.ctx.new_expr_ty_ref(id, type_id);
                    Ok(type_id)
                } else {
                    let type_id =self.ctx.intern_type(symbols::Type::IntLiteral); 
                    self.ctx.new_expr_ty_ref(id, type_id);
                    Ok(type_id)
                }
            },
            parser::Expr::Symbol(_) => {
                let symid = self.ctx.get_expr_ref(id).unwrap();
                let sym = self.ctx.get_object(symid).unwrap().clone();
                let ty = sym.ty.ok_or(String::from("No type in object"))?;
                self.ctx.new_expr_ty_ref(id, ty);
                return Ok(ty);
            },
            parser::Expr::FnCall(fncall)=> {
                let target_id = self.tc_expr(fncall.target)?;
                // get function type
                let f = self.ctx.get_type(target_id).unwrap().get_fn()?.clone();
                debugln!("ty/call: {}/{}", f.args.len(), fncall.args.len());

                if f.args.len() != fncall.args.len() {
                    return Err(format!("fn call args don't match expected args: {}:{}",
                            f.args.len(), fncall.args.len()));
                }
                for i in 0..f.args.len() {
                    let farg = f.args[i].clone();
                    let carg = fncall.args[i].clone();
                    let farg_ty = farg.ty;
                    let carg_ty = self.tc_expr(carg)?;
                    // compare
                    let r = self.compare_and_reduce_types(farg_ty, carg_ty)?;
                    // propagate to call arg
                    self.propagate_type(carg, r).unwrap();
                }
                if let Some(r) = f.ret_ty {
                    self.ctx.new_expr_ty_ref(id, r);
                    Ok(r)
                } else {
                    panic!("Impl");
                }
            },
            // _ => panic!("Impl expr {:?}.", expr),
        }
    }
    fn compare_and_reduce_numerics_untyped(&mut self, untyped: symbols::TypeId,
        typed: symbols::TypeId) -> Result<symbols::TypeId,String> {
        let unty = self.ctx.get_type(untyped).unwrap().clone();
        let ty = self.ctx.get_type(typed).unwrap().clone();
        if ty.is_float() {
            return Ok(typed);
        }
        if ty.is_integer() {
            // if untyped is a float literal than no
            return if unty.is_float() {
                Err(String::from(format!(
                            "Right type is an int but left is a float lit"
                )))
            } else {
                Ok(typed)
            }
        }
        panic!("Impl");
    }
    fn compare_and_reduce_numerics(&mut self, left: symbols::TypeId, right: symbols::TypeId) 
                                    -> Result<symbols::TypeId,String> {
        let l = self.ctx.get_type(left).unwrap().clone();
        let r = self.ctx.get_type(right).unwrap().clone();
        if !l.is_numeric() || !r.is_numeric() {
            panic!("expected numerics");
        }
        match (l.is_untyped(), r.is_untyped()) {
            (true, false) => self.compare_and_reduce_numerics_untyped(left, right),
            (false, true) => self.compare_and_reduce_numerics_untyped(right, left),
            (true, true) => { // if both are untyped return the flaot one if there's one
                if l.is_float() {
                    Ok(left)
                } else {
                    Ok(right)
                }
            }
            (false,false) => if left == right {
                    Ok(left)
                } else {
                    Err("types don't match".into())
                }
        }
    }
    fn compare_and_reduce_types(&mut self, left: symbols::TypeId, right: symbols::TypeId)
                            -> Result<symbols::TypeId,String> {
        if left == right {
            return  Ok(left);
        }
        let l = self.ctx.get_type(left).unwrap().clone();
        let r = self.ctx.get_type(right).unwrap().clone();
        if l.is_numeric() && r.is_numeric() {
            return self.compare_and_reduce_numerics(left, right);
        }
        panic!("Impl rest")
    }
    fn tc_ret(&mut self, ret: &parser::ExprId) -> Result<(),String> {
        let ty = self.tc_expr(ret.clone())?;
        if !self.current_fn_ret_type.is_some() {
            return Err(String::from(format!(
                        "return expr for no expeced return type")));
        }
        let ret_ty = self.current_fn_ret_type.unwrap();
        match self.compare_and_reduce_types(ty, ret_ty) {
            Ok(r) => {
                self.propagate_type(*ret, r).unwrap();
                Ok(())
            } ,
            Err(s) =>
                Err(String::from(format!(
                    "return expr type ({:?}) isn't the same as expected return type ({:?}) ({}).",
                    ty, ret_ty, s)))
        }
    }
    fn tc_stmt(&mut self, id: parser::StmtId) -> Result<(), String> {
        let stmt = self.ctx.get_stmt(id).unwrap().clone();
        match &stmt {
            parser::Stmt::Return(ret) => {
                self.tc_ret(ret)
            },
            parser::Stmt::Expr(id) => {
                self.tc_expr(*id)?;
                Ok(())
            }
            parser::Stmt::IfStmt(s) => {
                let cond_id = self.tc_expr(s.cond)?;
                let cond_ty = self.ctx.get_type(cond_id).unwrap();
                if !cond_ty.is_cond() {
                    panic!("Type is not able to condition.");
                }
                self.tc_block(&s.block)?;
                for alt in s.alt.clone() {
                    let cond_id = self.tc_expr(alt.cond)?;
                    let cond_ty = self.ctx.get_type(cond_id).unwrap();
                    if !cond_ty.is_cond() {
                        panic!("Type is not able to condition.");
                    }
                    self.tc_block(&alt.block)?;
                }
                if let Some(b) = s.else_block.clone() {
                    self.tc_block(&b)?;
                }
                Ok(())
            },
            parser::Stmt::Assignment(a) => {
                let target_ty = self.tc_expr(a.left)?;
                let e_ty = self.tc_expr(a.right)?;
                let res_ty = self.compare_and_reduce_types(target_ty, e_ty)?;
                self.propagate_type(a.left, res_ty).unwrap();
                self.propagate_type(a.right, res_ty).unwrap();
                Ok(())
            },
            parser::Stmt::VarDec(vardec) => {
                let obj_id = self.ctx.get_vardec_ref(id).unwrap();
                let obj = self.ctx.get_object(obj_id).unwrap().clone();
                if let Some(expr) = vardec.val {
                    let expr_ty = self.tc_expr(expr)?;
                    match obj.ty {
                        Some(ty) => {
                            let rt = self.compare_and_reduce_types(ty, expr_ty)?;
                            self.propagate_type(expr, rt)?;
                        },
                        None => {
                            let mut new_obj = obj.clone();
                            new_obj.ty = Some(expr_ty);
                            self.ctx.update_object(obj_id, new_obj);
                        },
                    }
                }
                Ok(())
            }
            _ => panic!("Impl stmt {:?}", stmt),
        }
    }
    fn tc_block(&mut self, block: &parser::Block) -> Result<(), String> {
        for id in block.stmts.clone() {
            self.tc_stmt(id)?;
        }
        Ok(())
    }
    fn tc_fndec(&mut self, id: parser::ItemId) -> Result<(), String> {
        let fndec = match self.ctx.get_item(id).unwrap() {
            parser::Item::FnDec(fndec) => fndec.clone(),
            // _ => panic!("other item when fndec expected"),
        };
        let prev_fn_ctx = self.current_fn_ret_type.to_owned();

        // get fn type ret ty
        let sym_id = self.ctx.get_item_ref(id).unwrap().clone();
        let fn_sym = self.ctx.get_object(sym_id).unwrap().clone();
        let fn_ty = self.ctx.get_type(fn_sym.ty.unwrap()).unwrap().clone();
        // set it to ret ty
        self.current_fn_ret_type = fn_ty.get_fn().unwrap().ret_ty;

        // check body
        let body = fndec.body.clone().unwrap();
        self.tc_block(&body)?;

        // set to prev
        self.current_fn_ret_type = prev_fn_ctx;
        Ok(())
    }
    fn tc_item(&mut self, id: parser::ItemId) -> Result<(), String> {
        let item = self.ctx.get_item(id).unwrap();
        match item {
            parser::Item::FnDec(_) => self.tc_fndec(id),
            // _ => panic!("Handle item {:?}.", item),
        }
    }
}
