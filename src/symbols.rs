use std::collections::HashMap;

use crate::parser::{self, Item, ItemId, ModId, TypeSpecifier};

use parser::{ExprId, StmtId};
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct TypeId(usize);

pub struct Object {
}
pub struct FnArg {
    name: String,
    ty: TypeId,
}
pub struct Function {
    args: Vec<FnArg>,
    ret_ty: Option<TypeId>,
}
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
    types: HashMap<String, ObjectId>,
}
struct Module {
    scope: ScopeId,
}
#[derive(Eq,PartialEq,Hash,Debug,Clone,Copy)]
pub struct ObjectId(usize);
pub struct SymbolTable {
    refs:HashMap<ExprId,ObjectId>,
    objects: HashMap<ObjectId,Object>,
    types: HashMap<TypeId,Type>,
    scopes: Vec<Scope>,
    global_scope: ScopeId,
    current_scope: ScopeId,
    p: parser::Parser,
    files: HashMap<String,parser::Parser>,
}
impl SymbolTable {
    fn new(p: parser::Parser) -> Self {
        let mut scopes = Vec::new();
        scopes.push(Scope{
            parent:None,
            objects:HashMap::new(),
            types:HashMap::new(),
        });
        let sid = ScopeId(scopes.len() - 1);
        Self {
            refs: HashMap::new(),
            objects: HashMap::new(),
            types: HashMap::new(),
            p,
            scopes: scopes,
            global_scope: sid,
            current_scope: sid,
            files: HashMap::new(),
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
    pub fn get_object(&mut self, id: ObjectId) -> Option<&Object> {
        self.objects.get(&id)
    }
    pub fn get_type(&mut self, id: TypeId) -> Option<&Type> {
        self.types.get(&id)
    }
    fn resolve_expr(&mut self, exprid: ExprId) -> Result<(), String> {
        match self.p.get_expr(exprid).unwrap() {
            e => panic!("Impl expr check for {:?}", e),
        }
        // Ok(())
    }
    fn new_object(&mut self, o: Object) -> Result<(), String> {
        panic!("Implement");
    }
    fn scope_get_type(&mut self, name: &String) -> Result<TypeId,String> {
        panic!("impl");
    }
    fn scope_get_object(&mut self, name: &String) -> Result<ObjectId,String> {
        panic!("impl");
    }
    fn resolve_type(&mut self, ty: &TypeSpecifier) -> Result<TypeId, String> {
        let name = ty.name();

        panic!("Handle");
    }
    // create fn dec
    fn resolve_fndec(&mut self, fn_dec: parser::FnDec) -> Result<(), String> {
        // create type first, resolve that, then create object
        // resolve args:

        let mut argdecs = Vec::<FnArg>::new();
        for a in fn_dec.args {
            let arg = FnArg{
                name: a.name,
                ty: B
            };
        }
        panic!("Implement");
    }
    fn resolve_item(&mut self, itemid: ItemId) -> Result<(), String> {
        match self.p.get_item(itemid).unwrap() {
            Item::FnDec(fn_dec) => {
                self.resolve_fndec(fn_dec.clone())
            },
            // _ => panic!("Impl resolve item"),
        }
    }
    fn resolve_mod_decs(&mut self, modid: ModId) -> Result<(), String> {
        let m = self.p.get_module(modid).unwrap().items.clone(); // clone atp bro
        for i in m {
            self.resolve_item(i)?;
        }
        Ok(())
    }
    fn resolve_stmt(&mut self, stmtid: StmtId) -> Result<(), String> {
        match self.p.get_stmt(stmtid).unwrap() {
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
    pub fn resolve(p: parser::Parser) -> Result<Self, String> {
        let mut st = Self::new(p);
        st.resolve_module(st.p.root)?;
        Ok(st)
    }
}
