use std::fs::read_to_string;
use std::collections::HashMap;
use crate::parser;
use crate::lexer;
use crate::symbols;
use crate::debugln;
use crate::type_checker;

pub struct Context {
    stmts:  Vec<parser::Stmt>,
    exprs:  Vec<parser::Expr>,
    items:  Vec<parser::Item>,
    mods:   Vec<parser::Module>,
    // spans for each
    pub stmts_span:  Vec<lexer::Span>,
    pub exprs_span:  Vec<lexer::Span>,
    pub items_span:  Vec<lexer::Span>,
    pub mods_span:   Vec<lexer::Span>,


    // references for objects
    vardec_refs: HashMap<parser::StmtId, symbols::ObjectId>, // vardecs
    expr_refs: HashMap<parser::ExprId, symbols::ObjectId>,
    item_fn_refs: HashMap<parser::ItemId, symbols::ObjectId>, // for functions and vars

    // struct decs item return a type
    item_ty_refs: HashMap<parser::ItemId, symbols::TypeId>, // for structs, alias
    // struct lit expr to it's resolved supposed type
    struct_lit_refs: HashMap<parser::ExprId, symbols::TypeId>,

    // expressions type
    expr_ty_refs: HashMap<parser::ExprId, symbols::TypeId>,

    // stores
    objects: Vec<symbols::Object>,
    types: Vec<symbols::Type>,
    base_scope: symbols::Scope,
}

impl Context {
    pub fn new() -> Self {
        let mut s = Self {
            stmts: Vec::new(),
            exprs: Vec::new(),
            items: Vec::new(),
            mods: Vec::new(),
            stmts_span: Vec::new(),
            exprs_span: Vec::new(),
            items_span: Vec::new(),
            mods_span: Vec::new(),

            // refs
            vardec_refs: HashMap::new(),
            expr_refs: HashMap::new(),
            item_fn_refs: HashMap::new(),
            item_ty_refs: HashMap::new(),
            expr_ty_refs: HashMap::new(),
            struct_lit_refs: HashMap::new(),

            objects: Vec::new(),
            types: Vec::new(),
            base_scope:symbols::Scope::new(None),
        };
        let mut tid = s.declare_type(symbols::Type::I32);
        s.base_scope.declare_type("i32".into(), tid).unwrap();
        tid = s.declare_type(symbols::Type::F32);
        s.base_scope.declare_type("f32".into(), tid).unwrap();
        s
    }
    pub fn new_vardec_ref(&mut self, stmtid: parser::StmtId, objectid: symbols::ObjectId) {
        self.vardec_refs.insert(stmtid, objectid);
    }
    pub fn get_vardec_ref(&self, stmtid: parser::StmtId) -> Option<symbols::ObjectId> {
        self.vardec_refs.get(&stmtid).copied()
    }
    pub fn new_expr_ref(&mut self, exprid: parser::ExprId, objectid: symbols::ObjectId) {
        self.expr_refs.insert(exprid, objectid);
    }
    pub fn new_item_fn_ref(&mut self, itemid: parser::ItemId, objectid: symbols::ObjectId) {
        self.item_fn_refs.insert(itemid, objectid);
    }
    pub fn new_item_ty_ref(&mut self, itemid: parser::ItemId, tyid: symbols::TypeId) {
        self.item_ty_refs.insert(itemid, tyid);
    }
    pub fn new_struct_lit_ref(&mut self, expr: parser::ExprId, ty: symbols::TypeId) {
        self.struct_lit_refs.insert(expr, ty);
    }
    pub fn get_expr_ref(&self, exprid: parser::ExprId) -> Option<symbols::ObjectId> {
        Some(self.expr_refs.get(&exprid).unwrap().clone())
    }
    pub fn get_item_fn_ref(&self, itemid: parser::ItemId) -> Option<symbols::ObjectId> {
        Some(self.item_fn_refs.get(&itemid).unwrap().clone())
    }
    pub fn get_item_ty_ref(&self, itemid: parser::ItemId) -> Option<symbols::TypeId> {
        Some(self.item_ty_refs.get(&itemid).unwrap().clone())
    }
    pub fn get_struct_lit_ref(&mut self, expr: parser::ExprId) -> Option<symbols::TypeId> {
        Some(self.struct_lit_refs.get(&expr).unwrap().clone())
    }
    pub fn new_expr_ty_ref(&mut self, exprid: parser::ExprId, typeid: symbols::TypeId) {
        self.expr_ty_refs.insert(exprid, typeid);
    }
    pub fn get_expr_ty_ref(&self, exprid: parser::ExprId) -> Option<symbols::TypeId> {
        Some(self.expr_ty_refs.get(&exprid).unwrap().clone())
    }

    pub fn new_stmt(&mut self, stmt: parser::Stmt, span: lexer::Span) -> parser::StmtId {
        self.stmts.push(stmt);
        self.stmts_span.push(span);
        return parser::StmtId::new(self.stmts.len() - 1);
    }
    pub fn new_expr(&mut self, expr: parser::Expr, span: lexer::Span) -> parser::ExprId {
        self.exprs.push(expr);
        self.exprs_span.push(span);
        return parser::ExprId::new(self.exprs.len() - 1);
    }
    pub fn new_item(&mut self, item: parser::Item, span: lexer::Span) -> parser::ItemId {
        self.items.push(item);
        self.items_span.push(span);
        return parser::ItemId::new(self.items.len() - 1);
    }
    pub fn new_mod(&mut self, m: parser::Module, span: lexer::Span) -> parser::ModId {
        self.mods.push(m);
        self.mods_span.push(span);
        return parser::ModId::new(self.mods.len() - 1);
    }
    pub fn get_expr(&self, id: parser::ExprId) -> Result<&parser::Expr, String> {
        self.exprs.get(id.id()).ok_or(
            format!("expr {:?} doesn't exist", id))
    }
    pub fn get_module(&self, id: parser::ModId) -> Result<&parser::Module, String> {
        self.mods.get(id.id()).ok_or(
            format!("module {:?} doesn't exist", id))
    }
    pub fn get_item(&self, id: parser::ItemId) -> Result<&parser::Item, String> {
        self.items.get(id.id()).ok_or(
            format!("item {:?} doesn't exist", id))
    }

    pub fn get_stmt(&self, id: parser::StmtId) -> Result<&parser::Stmt, String> {
        self.stmts.get(id.id()).ok_or(
            format!("stmt {:?} doesn't exist", id))
    }
    pub fn get_object(&mut self, id: symbols::ObjectId) -> Option<&symbols::Object> {
        self.objects.get(id.0)
    }
    pub fn get_type(&mut self, id: symbols::TypeId)
            -> Option<&symbols::Type> {
        self.types.get(id.0)
    }
    pub fn declare_object(&mut self, obj: symbols::Object) -> symbols::ObjectId {
        self.objects.push(obj);
        symbols::ObjectId(self.objects.len() -1)
    }
    pub fn update_object(&mut self, id: symbols::ObjectId, obj: symbols::Object)
            -> symbols::ObjectId {
        self.objects[id.0] = obj;
        id
    }
    pub fn update_type(&mut self, id: symbols::TypeId, ty: symbols::Type)
            -> symbols::TypeId {
        self.types[id.0] = ty;
        id
    }
    pub fn declare_type(&mut self, ty: symbols::Type) -> symbols::TypeId {
        self.intern_type(ty)
        /*self.types.push(ty);
        symbols::TypeId(self.types.len() -1)*/
    }
    pub fn base_scope_get_type(&mut self, name: &String) ->
        Result<symbols::TypeId, String> {
            match self.base_scope.get_type(name) {
                Some(t) => {
                    debugln!("Base type: {:?}", t);
                    Ok(*t)
                },
                None => Err(String::from(format!("Type {} doesn't exist", name))),
            }
    }
    // intern types
    pub fn intern_type(&mut self, ty: symbols::Type) -> symbols::TypeId {
        // check if it already exists
        for (id, existing) in self.types.iter().enumerate() {
            if *existing == ty {
                return symbols::TypeId(id);
            }
        }
        self.types.push(ty);
        symbols::TypeId(self.types.len() - 1)
    }
    pub fn add_module(&mut self, f: String) {
        let f = read_to_string(f).unwrap();
        let l = lexer::Lexer::from_code(&f).unwrap();
        let mid = parser::Parser::parse(self, l);
        let _ = symbols::SymbolResolver::resolve(self, mid).unwrap();
        type_checker::TypeChecker::type_check(self, mid).unwrap();
    }
}
