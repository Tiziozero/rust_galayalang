use std::collections::HashMap;

use crate::parser::{self, Item, ItemId, ModId};

use parser::{ExprId, StmtId};

pub struct Object {
}
pub struct FnArg {
    name: String,
    ty: Box<Type>,
}
pub struct Function {
    args: Vec<FnArg>,
    ret_ty: Option<Box<Type>>,
}
pub enum Type {
    I32, F32,
    Function(Function),
}
#[derive(Eq,PartialEq,Hash,Debug,Clone,Copy)]
pub struct ScopeId(usize);
#[derive(Debug)] // optional reference to parent
struct Scope {
    parent: Option<ScopeId>,
    symbols: HashMap<String, SymbolId>,
}

#[derive(Debug,Clone)]
struct FnPreDecArg {
    name:String,
    ty: parser::TypeSpecifier, // parser type
}
#[derive(Debug,Clone)]
struct FnPreDec {
    name: String,
    args: Vec<FnPreDecArg>,
    ret_ty: Option<parser::TypeSpecifier>,
}
#[derive(Debug,Clone)]
enum PreDec {
    Fn(FnPreDec),
}
struct Module {
    pre_decs: HashMap<String, PreDec>,
    scope: ScopeId,
}
#[derive(Eq,PartialEq,Hash,Debug,Clone,Copy)]
pub struct SymbolId(usize);
pub struct SymbolTable {
    refs:HashMap<ExprId,SymbolId>,
    objects: HashMap<SymbolId,Object>,
    types: HashMap<SymbolId,Type>,
    scopes: Vec<Scope>,
    global_scope: ScopeId,
    p: parser::Parser,
}
impl SymbolTable {
    fn new(p: parser::Parser) -> Self {
        let mut scopes = Vec::new();
        scopes.push(Scope{parent:None, symbols:HashMap::new()});
        let sid = ScopeId(scopes.len() - 1);
        Self {
            refs: HashMap::new(),
            objects: HashMap::new(),
            types: HashMap::new(),
            p,
            scopes: scopes,
            global_scope: sid,
        }
    }
    fn get_scope(&mut self, id: ScopeId) -> Option<&Scope> {
        self.scopes.get(id.0)
    }
    fn new_scope(&mut self, parent: Option<ScopeId>) -> ScopeId {
        self.scopes.push(Scope { parent: parent, symbols: HashMap::new()});
        ScopeId(self.scopes.len() - 1)
    }
    pub fn get_object(&mut self, id: SymbolId) -> Option<&Object> {
        self.objects.get(&id)
    }
    pub fn get_type(&mut self, id: SymbolId) -> Option<&Type> {
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
    // create fn dec
    fn resolve_fndec(&mut self, fn_dec: parser::FnDec) -> Result<(), String> {
        // create type first, resolve that, then create object
        // resolve args:

        for a in fn_dec.args {
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
