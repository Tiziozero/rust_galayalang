mod resolver;
mod parser;
mod lexer;
mod symbols;

fn main() -> Result<(), parser::ParserErr> {
    resolver::ModuleContext::new();
    // let _  = resolver::Resolver::new(p).unwrap();
    /*let types = type_check::TypeChecker::resolve(&mut resolved, &p).unwrap();
    // owns it all now
    interpreter::Interpreter::run(p, resolved, types).unwrap();*/
    println!("Run successfully");
    Ok(())
}


