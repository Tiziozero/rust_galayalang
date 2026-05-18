use std::{collections::HashMap};

use crate::parser::{self, Item, ItemId, ModId, TypeSpecifier};
use crate::{debugln};

use parser::{ExprId, StmtId};
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct TypeId(pub usize);

#[derive(Debug,Clone)]
pub struct Object {
    name: String,
    ty: Option<TypeId>,
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
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScopeId(usize);
#[derive(Clone,Debug)] // optional reference to parent
pub struct Scope {
    parent: Option<ScopeId>,
    objects: HashMap<String, ObjectId>,
    types: HashMap<String, TypeId>,
    object_forward_decs: HashMap<String, ObjectId>,
    type_forward_decs: HashMap<String, TypeId>,
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
            object_forward_decs:HashMap::new(),
            type_forward_decs:HashMap::new(),
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
    fn get_scope_mut(&mut self, id: ScopeId) -> Option<&mut Scope> {
        self.scopes.get_mut(id.0)
    }
    fn new_scope(&mut self, parent: Option<ScopeId>) -> ScopeId {
        self.scopes.push(Scope {
            parent: parent,
            objects: HashMap::new(),
            types: HashMap::new(),
            object_forward_decs: HashMap::new(),
            type_forward_decs: HashMap::new(),
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
    fn resolve_fn_call(&mut self, fn_call: &parser::FnCall)
        -> Result<(), String> {

        // resolve target
        self.resolve_expr(fn_call.target)?;
        // resolve args
        for a in &fn_call.args {
            self.resolve_expr(*a)?;
        }
        Ok(())
    }
    fn resolve_expr(&mut self, exprid: ExprId) -> Result<(), String> {
        let expr = self.get_current_ast()?.get_expr(exprid).unwrap().clone();
        match expr {
            parser::Expr::Number(_) => Ok(()),
            parser::Expr::Symbol(s) => {
                let name = s.symbol.clone();
                let id = self.scope_get_object(&name)?;
                self.refs.insert(exprid, id);
                Ok(())
            },
            parser::Expr::Binop(binop) => {
                self.resolve_expr(binop.left)?;
                self.resolve_expr(binop.right)?;
                Ok(())
            }
            parser::Expr::FnCall(fn_call) => {
                self.resolve_fn_call(&fn_call)
            },
            // e => panic!("Impl expr check for {:?}", e),
        }
        // Ok(())
    }
    fn declare_object(&mut self, name: String, ty: Option<TypeId>) ->
        Result<ObjectId, String> {
        let o = Object { name: name.clone(), ty };
        // if it's a predec
        if let Some(id) = self.get_scope(self.current_scope).unwrap()
            .is_object_foreward_dec(&name) {
            self.ctx.update_object(id, o);
            self.get_scope_mut(self.current_scope).unwrap()
                .declare_object(name.clone(), id)?;
            return Ok(id);
        }
        let id = self.ctx.declare_object(o);
        self.get_scope_mut(self.current_scope).unwrap()
            .declare_object(name.clone(), id)?;
        debugln!("Adding object {} to scope {:?}", name, self.current_scope);
        Ok(id)
    }
    fn scope_get_object(&mut self, name: &String) -> Result<ObjectId,String> {
        let mut id = self.current_scope;
        loop {
            match self.get_scope(id) {
                Some(s) => {
                    match s.get_object(name) {
                        Some(t) => return Ok(*t),
                        None => {
                            match s.parent {
                                Some(pid) => id = pid,
                                None =>
                                    return Err(String::from(
                                            format!("object {} does not exist.",
                                                name)))
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
                    match s.get_type(name) {
                        Some(t) => return Ok(*t),
                        None => {
                            match s.parent {
                                Some(pid) => id = pid,
                                None => // check base 
                                    return self.ctx.base_scope_get_type(name)
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
    fn resolve_block(&mut self, block: &parser::Block) -> Result<(), String> {
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
        let interned = self.ctx.intern_type(fn_ty.clone());
        // define self for recursion
        self.declare_object(fn_dec.name.clone(),
                Some(interned))?;
        // make sure body's alright
        if let Some(b) = fn_dec.body {
            self.resolve_block(&b)?;
        } else {
            panic!("Fn must have body");
        }
        self.exit_scope();
        let interned = self.ctx.intern_type(fn_ty.clone());
        // define it in scope
        self.declare_object(fn_dec.name.clone(),
                Some(interned))?;
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
    fn resolve_vardec(&mut self, vardec: &parser::VarDec)  -> Result<(), String> {
        let t = if let Some(ty) = vardec.ty.clone() {
            let clone = ty.clone();
            let r = self.resolve_type(&clone)?; 
            Some(r)
        } else {
            None
        };
        let name = vardec.s.clone();
        if let Some(v) = vardec.val {
            debugln!("verdec has expression");
            self.resolve_expr(v)?;
        }
        self.declare_object(name, t)?; // create object
        Ok(())
    }
    fn resolve_if_stmt(&mut self, if_stmt: &parser::IfStmt)
        -> Result<(), String> {

        self.enter_scope(); // if cond/block
        self.resolve_expr(if_stmt.cond)?;
        self.resolve_block(&if_stmt.block)?;
        self.exit_scope(); // base con/block
        for alt in &if_stmt.alt {
            self.enter_scope(); // alt if cond/block
            self.resolve_expr(alt.cond)?;
            self.resolve_block(&alt.block)?;
            self.exit_scope(); // alt con/block
        }
        if let Some(b) = &if_stmt.else_block {
            self.resolve_block(b)?;
        }
        Ok(())
    }
    fn resolve_assignment(&mut self, a: &parser::Assignment) -> Result<(), String> {

        panic!("impl");
    }
    fn resolve_stmt(&mut self, stmtid: StmtId) -> Result<(), String> {
        let stmt = self.current_p.as_mut().unwrap().get_stmt(stmtid).unwrap();
        match stmt.clone() {
            parser::Stmt::Expr(exprid) => {
                self.resolve_expr(exprid)
            },
            parser::Stmt::VarDec(vardec) => {
                self.resolve_vardec(&vardec)
            },
            parser::Stmt::IfStmt(if_stmt) => {
                self.resolve_if_stmt(&if_stmt)
            },
            parser::Stmt::Assignment(a) => {
                self.resolve_assignment(&a)
            },
            s => panic!("Impl stmt check for {:?}", s),
        }
    }
    fn resolve_item_forward_dec(&mut self, itemid: ItemId) -> Result<(), String> {
        let p = self.get_current_ast()?;
        let i = p.get_item(itemid).unwrap();
        match i {
            Item::FnDec(fn_dec) => {
                // declare place holder
                let f = fn_dec.clone();
                let o = Object{name: f.name.clone(), ty: None};
                let id = self.ctx.declare_object(o.clone());
                // declare predec
                self.get_scope_mut(self.current_scope).unwrap()
                    .declare_object_forward_dec(f.name.clone(), id)?;
                Ok(())
            },
            // _ => panic!("Impl resolve item"),
        }
    }
    fn resolve_mod_forward_decs(&mut self, modid: ModId) -> Result<(), String> {
        let p = self.get_current_ast()?;
        let m = p.get_module(modid).unwrap().items.clone(); // clone atp bro
        for i in m {
            self.resolve_item_forward_dec(i)?;
        }
        Ok(())
    }
    fn resolve_module(&mut self, modid: ModId) -> Result<(), String> {
        self.resolve_mod_forward_decs(modid)?;
        self.resolve_mod_decs(modid)?;
        Ok(())
    }
    fn get_current_ast(&mut self) -> Result<&mut parser::Parser,String> {
        let p = self.current_p.as_mut()
            .ok_or(String::from("No cuurent parser in st"))?;
        Ok(p)
    }
    pub fn resolve(&mut self, p: parser::Parser) -> Result<(), String> {
        let m = p.root.clone();
        self.current_p = Some(p);
        self.resolve_module(m)?;
        let mut k = 0;
        for s in &self.scopes {
            let d = s.object_forward_decs.len() + s.type_forward_decs.len();
            println!(" scope {} has {} forward decs left.", k, d);
            k += 1;
        }
        Ok(())
    }
}
impl Scope {
    pub fn new(parent: Option<ScopeId>) -> Self {
        Scope {
            parent: parent,
            objects: HashMap::new(),
            types: HashMap::new(),
            object_forward_decs: HashMap::new(),
            type_forward_decs: HashMap::new(),
        }
    }
    pub fn get_type(&self, name: &String) -> Option<&TypeId> {
        match self.types.get(name) {
            Some(t) => Some(t),
            None => self.type_forward_decs.get(name),
        }
    }
    pub fn get_object(&self, name: &String) -> Option<&ObjectId> {
        match self.objects.get(name) {
            Some(o) => Some(o),
            None => self.object_forward_decs.get(name),
        }
    }
    pub fn declare_object(&mut self, name: String, id: ObjectId)
        -> Result<(), String>{
        // only one name per scope
        if let Some(_) = self.objects.get(&name) {
            return Err(String::from(
                    format!("Object {} already exists.", name)));
        }
        if let Some(_) = self.types.get(&name) {
            return Err(String::from(
                    format!("Object {} already exists as type.", name)));
        }
        // check forward_decs
        if let Some(_) = self.type_forward_decs.get(&name) {
            return Err(String::from(
                    format!("object {} is expected to be a type", name)));
        }
        if let Some(_) = self.object_forward_decs.get(&name) {
            debugln!("Objecr {} is a forward dec", name);
            self.object_forward_decs.remove(&name);
        }
        self.objects.insert(name, id);
        Ok(())
    }
    pub fn declare_type(&mut self, name: String, id: TypeId)
        -> Result<(), String>{
        // only one name per scope
        if let Some(_) = self.objects.get(&name) {
            return Err(String::from(
                    format!("Type {} already exists.", name)));
        }
        if let Some(_) = self.types.get(&name) {
            return Err(String::from(
                    format!("Type {} already exists as type.", name)));
        }
        // check forward_decs
        if let Some(_) = self.object_forward_decs.get(&name) {
            return Err(String::from(
                    format!("Type {} is expected to be a object", name)));
        }
        if let Some(_) = self.type_forward_decs.get(&name) {
            self.type_forward_decs.remove(&name);
        }
        self.types.insert(name, id);
        Ok(())
    }
    pub fn declare_type_forward_dec(&mut self, name: String, id: TypeId)
        -> Result<(), String>{
        // only one name per scope
        if let Some(_) = self.objects.get(&name) {
            return Err(String::from(
                    format!("Type forward_dec {} already exists as an object.",
                        name)));
        }
        if let Some(_) = self.types.get(&name) {
            return Err(String::from(
                    format!("Type forward_dec {} already exists.", name)));
        }
        // check forward_decs
        if let Some(_) = self.type_forward_decs.get(&name) {
            return Err(String::from(
                    format!("Type forward_dec {} already exists as a type forward dec.",
                        name)));
        }
        if let Some(_) = self.object_forward_decs.get(&name) {
            return Err(String::from(
                format!("Type forward_dec {} already exists as an object forward dec.",
                    name)));
        }
        self.type_forward_decs.insert(name, id);
        Ok(())
    }
    pub fn declare_object_forward_dec(&mut self, name: String, id: ObjectId)
        -> Result<(), String>{
        // only one name per scope
        if let Some(_) = self.objects.get(&name) {
            return Err(String::from(
                    format!("object forward_dec {} already exists.", name)));
        }
        if let Some(_) = self.types.get(&name) {
            return Err(String::from(
                    format!("object forward_dec {} already exists as a type.", name)));
        }
        // check forward_decs
        if let Some(_) = self.type_forward_decs.get(&name) {
            return Err(String::from(
                    format!("object forward_dec {} already exists as a type forward dec.",
                        name)));
        }
        if let Some(_) = self.object_forward_decs.get(&name) {
            return Err(String::from(
                format!("object forward_dec {} already exists as an object forward dec.",
                    name)));
        }
        self.object_forward_decs.insert(name, id);
        Ok(())
    }
    pub fn is_object_foreward_dec(&self, name: &String) -> Option<ObjectId> {
        match self.object_forward_decs.get(name) {
            Some(id) => Some(id.clone()),
            None => None,
        }
    }
    pub fn is_type_foreward_dec(&self, name: &String) -> Option<TypeId> {
        match self.type_forward_decs.get(name) {
            Some(id) => Some(id.clone()),
            None => None,
        }
    }
}
