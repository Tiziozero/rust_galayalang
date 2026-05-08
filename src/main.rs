use std::fs::read_to_string;

mod lexer;
mod parser;
mod resolver;
// mod type_check;
// mod interpreter;


fn main()  {
    let code = read_to_string("main.gala").unwrap();
    let lexer = lexer::Lexer::from_code(&code).unwrap_or_else(|err| {
        panic!("error {}", err);});
    println!("end");
    let _p = parser::Parser::parse(lexer); // takes ownership
    // let _  = resolver::Resolver::new(p).unwrap();
    /*let types = type_check::TypeChecker::resolve(&mut resolved, &p).unwrap();
    // owns it all now
    interpreter::Interpreter::run(p, resolved, types).unwrap();*/
    println!("Run successfully");
}


