use std::{collections::HashMap};

use crate::parser::{self, Item, ItemId, ModId, TypeSpecifier};

use parser::{ExprId, StmtId};
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct TypeId(pub usize);

#[derive(Debug,Clone)]
pub struct Object {
    name: String,
    ty: Option<Type>,
}
#[derive(Debug,Clone,PartialEq)]
pub struct FnArg {
    name: String,
    ty: TypeId,
}
#[derive(Debug,Clone)]
pub struct Function {
    args: Vec<FnArg>,
    ret_ty: Option<TypeId>,
}
impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        if self.args.len() != other.args.len() {
            return false;
        }
        for (i, a) in self.args.iter().enumerate() {
            if *a != other.args[i] {
                return false;
            }
        }
        if self.ret_ty != other.ret_ty {
            return false;
        }
        true
    }
}
#[derive(Debug,Clone,PartialEq)]
pub enum Type {
    I32, F32,
    Function(Function),
    Pointer(TypeId),
}
#[derive(Eq,PartialEq,Hash,Debug,Clone,Copy)]
pub struct ScopeId(usize);
#[derive(Debug)] // optional reference to parent
struct Scope {
    parent: Option<ScopeId>,
    objects: HashMap<String, ObjectId>,
    types: HashMap<String, TypeId>,
}
pub struct Module {
    objects: Vec<Object>,
    types: Vec<Type>,
    p: parser::Parser,
}
#[derive(Eq,PartialEq,Hash,Debug,Clone,Copy)]
pub struct ObjectId(pub usize);
use crate::resolver;
pub struct SymbolResolver<'ctx> {
    ctx: &'ctx mut resolver::Context,
    refs: HashMap<ExprId,ObjectId>,
    scopes: Vec<Scope>,
    global_scope: ScopeId,
    current_scope: ScopeId,
    current_p: Option<parser::Parser>,
}
impl<'ctx> SymbolResolver<'ctx> {
    pub fn new(ctx: &'ctx mut resolver::Context) -> Self {
        let mut scopes = Vec::new();
        scopes.push(Scope{
            parent:None,
            objects:HashMap::new(),
            types:HashMap::new(),
        });
        let sid = ScopeId(scopes.len() - 1);
        Self {
            ctx,
            refs: HashMap::new(),
            current_p: None,
            scopes: scopes,
            global_scope: sid,
            current_scope: sid,
        }
    }
    fn get_scope(&mut self, id: ScopeId) -> Option<&Scope> {
        self.scopes.get(id.0)
    }
    fn new_scope(&mut self, parent: Option<ScopeId>) -> ScopeId {
        self.scopes.push(Scope {
            parent: parent,
            objects: HashMap::new(),
            types: HashMap::new(),
        });
        ScopeId(self.scopes.len() - 1)
    }
    fn enter_scope(&mut self) {
        let s = self.new_scope(Some(self.current_scope));
        self.current_scope = s;
    }
    fn exit_scope(&mut self) {
        match self.get_scope(self.current_scope).unwrap().parent {
            Some(s) => self.current_scope = s,
            None => panic!("Can't exit scope. has no parent"),
        }
    }
    pub fn get_object(&mut self, id: ObjectId) -> Option<&Object> {
        self.ctx.get_object(id)
    }
    pub fn get_type(&mut self, id: TypeId) -> Option<&Type> {
        self.ctx.get_type(id)
    }
    fn resolve_expr(&mut self, exprid: ExprId) -> Result<(), String> {
        let p = self.get_current_ast()?;
        match p.get_expr(exprid).unwrap() {
            e => panic!("Impl expr check for {:?}", e),
        }
        // Ok(())
    }
    fn new_object(&mut self, name: String, ty: Option<Type>) -> Result<ObjectId, String> {
        let o = Object { name, ty };
        Ok(self.ctx.new_object(o))
    }
    fn scope_get_object(&mut self, name: &String) -> Result<ObjectId,String> {
        let mut id = self.current_scope;
        loop {
            match self.get_scope(id) {
                Some(s) => {
                    match s.objects.get(name) {
                        Some(t) => return Ok(*t),
                        None => {
                            match s.parent {
                                Some(pid) => id = pid,
                                None =>
                                    return Err(String::from("Type does not exist"))
                            }
                        }
                    }
                },
                None => panic!("What"),
            }
        }
    }
    fn scope_get_type(&mut self, name: &String) -> Result<TypeId,String> {
        let mut id = self.current_scope;
        loop {
            match self.get_scope(id) {
                Some(s) => {
                    match s.types.get(name) {
                        Some(t) => return Ok(*t),
                        None => {
                            match s.parent {
                                Some(pid) => id = pid,
                                None =>
                                    return Err(String::from("Type does not exist"))
                            }
                        }
                    }
                },
                None => panic!("What"),
            }
        }
    }
    fn intern_type(&mut self, ty: Type) -> TypeId {
        self.ctx.intern_type(ty)
    }
    fn resolve_type_specifier(&mut self, ty: &TypeSpecifier) -> Result<TypeId, String> {
        match ty {
            TypeSpecifier::Base(name) => self.scope_get_type(name),
            TypeSpecifier::Pointer(b) => {
                let base_id = self.resolve_type_specifier(b)?;
                Ok(self.intern_type(Type::Pointer(base_id)))
            }
        }
    }
    fn resolve_type(&mut self, ty: &TypeSpecifier) -> Result<TypeId, String> {
        self.resolve_type_specifier(ty)
    }
    fn resolve_fndec_body(&mut self, block: parser::Block) -> Result<(), String> {
        let stmts = block.stmts.clone();
        for stmt in stmts {
            self.resolve_stmt(stmt)?;
        }
        Ok(())
    }
    // create fn dec
    fn resolve_fndec(&mut self, fn_dec: parser::FnDec) -> Result<(), String> {
        // create type first, resolve that, then create object
        // resolve args:

        self.enter_scope(); // for fn recursion + args
        let argdecs = Vec::<FnArg>::new();
        for a in fn_dec.args {
            panic!("Impl arg res");
        }
        // check type
        let ret_ty = match fn_dec.ret_ty {
            Some(t) => Some(self.resolve_type(&t)?),
            None => None,
        };
        let fn_ty = Type::Function(Function {
            args: argdecs,
            ret_ty,
        });
        // define self for recursion
        self.new_object(fn_dec.name.clone(), Some(fn_ty.clone()))?;
        // make sure body's alright
        if let Some(b) = fn_dec.body {
            self.resolve_fndec_body(b)?;
        } else {
            panic!("Fn must have body");
        }
        self.exit_scope();
        self.new_object(fn_dec.name.clone(), Some(fn_ty.clone()))?;
        Ok(())
    }
    fn resolve_item(&mut self, itemid: ItemId) -> Result<(), String> {
        let p = self.get_current_ast()?;
        let i = p.get_item(itemid).unwrap();
        match i {
            Item::FnDec(fn_dec) => {
                let f = fn_dec.clone();
                self.resolve_fndec(f)
            },
            // _ => panic!("Impl resolve item"),
        }
    }
    fn resolve_mod_decs(&mut self, modid: ModId) -> Result<(), String> {
        let p = self.get_current_ast()?;
        let m = p.get_module(modid).unwrap().items.clone(); // clone atp bro
        for i in m {
            self.resolve_item(i)?;
        }
        Ok(())
    }
    fn resolve_stmt(&mut self, stmtid: StmtId) -> Result<(), String> {
        let p = self.current_p.as_ref().ok_or(String::from("No cuurent parser in st"))?;
        match p.get_stmt(stmtid).unwrap() {
            parser::Stmt::Expr(exprid) => {
                self.resolve_expr(*exprid)?;
            },
            s => panic!("Impl stmt check for {:?}", s),
        }
        Ok(())
    }
    fn resolve_module(&mut self, modid: ModId) -> Result<(), String> {
        self.resolve_mod_decs(modid)?;
        Ok(())
    }
    fn get_current_ast(&mut self) -> Result<&parser::Parser,String> {
        let p = self.current_p.as_ref().ok_or(String::from("No cuurent parser in st"))?;
        Ok(p)
    }
    pub fn resolve(&mut self, p: parser::Parser) -> Result<(), String> {
        let m = p.root.clone();
        self.current_p = Some(p);
        self.resolve_module(m)?;
        Ok(())
    }
}
