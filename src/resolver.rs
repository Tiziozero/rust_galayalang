use std::fs::read_to_string;
use crate::parser;
use crate::lexer;
use crate::symbols;
use crate::symbols::ObjectId;
use crate::debugln;
use crate::type_checker;

pub struct Context {
    objects: Vec<symbols::Object>,
    types: Vec<symbols::Type>,
    base_scope: symbols::Scope,
}

impl Context {
    pub fn new() -> Self {
        let mut s = Self {
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
    pub fn update_object(&mut self, id: ObjectId, obj: symbols::Object)
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
        self.types.push(ty);
        symbols::TypeId(self.types.len() -1)
    }
    pub fn base_scope_get_type(&mut self, name: &String) ->
        Result<symbols::TypeId, String> {
            match self.base_scope.get_type(name) {
                Some(t) => {
                    debugln!("Base type: {:?}", t);
                    Ok(*t)
                },
                None => Err(String::from("Type doesn't exist")),
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
        let p = parser::Parser::parse(l);
        let module = symbols::SymbolResolver::resolve(self, p).unwrap();
        type_checker::TypeChecker::type_check(self, &module).unwrap();
    }
}
