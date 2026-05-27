use crate::parser;
use crate::{resolver, symbols};
use crate::{debugln};

pub struct TypeChecker<'ctx> {
    ctx: &'ctx mut resolver::Context,
    current_fn_ret_type: Option<symbols::TypeId>,
}

impl<'ctx> TypeChecker<'ctx> {
    pub fn type_check(ctx: &'ctx mut resolver::Context, moduleid: parser::ModId) -> Result<(), String> {
        let mut tc = TypeChecker{ctx, current_fn_ret_type:None};
        for i in tc.ctx.get_module(moduleid).unwrap().items.clone() {
            debugln!("{:?}", i);
            tc.tc_item(i)?;
        }
        panic!("tc complete");
    }
    fn tc_binop(&mut self, b: &parser::Binop) -> Result<symbols::TypeId, String> {
        let l = self.tc_expr(b.left)?;
        let r = self.tc_expr(b.right)?;
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
        let res_id = self.compare_and_reduce_types(l, r)?;
        return Ok(res_id);
    }
    fn tc_expr(&mut self, id: parser::ExprId) -> Result<symbols::TypeId, String> {
        let expr = self.ctx.get_expr(id).unwrap().clone();
        match expr {
            parser::Expr::Binop(b) => {
                let type_id = self.tc_binop(&b)?;
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
            _ => panic!("Impl expr {:?}.", expr),
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
            // _ => panic!("Impl {:?} {:?}", l, r)
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
            Ok(_) => Ok(()),
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
